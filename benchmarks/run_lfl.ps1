[CmdletBinding()]
param(
    [string]$Python,
    [string]$Data,
    [string]$OutDir,
    [int]$QuietLoad = 20,
    [switch]$Force
)

$ErrorActionPreference = 'Stop'
$repo = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
if (-not $Python) { $Python = Join-Path $repo 'target\bench312\Scripts\python.exe' }
if (-not $Data) { $Data = Join-Path $repo 'target\bench_data.parquet' }
if (-not $OutDir) { $OutDir = Join-Path $repo 'target\bench-results' }
$benchDir = Join-Path $repo 'benchmarks'
$spinner = @('|', '/', '-', '\')

function Get-CpuLoad {
    [int](Get-CimInstance Win32_Processor | Measure-Object -Property LoadPercentage -Average).Average
}

function Get-LoadColor([int]$Load) {
    if ($Load -le $QuietLoad) { 'Green' } elseif ($Load -le 50) { 'Yellow' } else { 'Red' }
}

function Format-Span([timespan]$Span) {
    '{0:00}:{1:00}:{2:00}' -f [int][math]::Floor($Span.TotalHours), $Span.Minutes, $Span.Seconds
}

function Write-Banner([string]$Text, [string]$Color) {
    $line = '=' * 74
    Write-Host ''
    Write-Host $line -ForegroundColor DarkGray
    Write-Host ("  " + $Text) -ForegroundColor $Color
    Write-Host $line -ForegroundColor DarkGray
}

function Write-OutputLine([string]$Line) {
    $color = 'Gray'
    if ($Line -match 'rows/s') { $color = 'Cyan' }
    elseif ($Line -match '^(=+|Result)') { $color = 'DarkGray' }
    elseif ($Line -match 'Running|Building|dataset') { $color = 'White' }
    elseif ($Line -match 'Error|Traceback') { $color = 'Red' }
    Write-Host ("  " + $Line) -ForegroundColor $color
}

function Invoke-Tool([string]$Tool, [string[]]$ExtraArgs) {
    $resultPath = Join-Path $OutDir "$Tool-lfl.json"
    if ((Test-Path $resultPath) -and -not $Force) {
        $saved = Get-Content $resultPath -Raw | ConvertFrom-Json
        Write-Host ("  {0}: reusing saved result, {1:N0} rows/s (use -Force to rerun)" -f $Tool, $saved.rows_per_s) -ForegroundColor DarkYellow
        return $saved
    }

    $log = Join-Path $OutDir "$Tool-lfl.log"
    $err = Join-Path $OutDir "$Tool-lfl.err"
    $script = Join-Path $benchDir 'benchmark_1m.py'
    $argList = @('-I', "`"$script`"", '--tool', $Tool, '--mode', 'lfl', '--data', "`"$Data`"") + $ExtraArgs
    $proc = Start-Process -FilePath $Python -ArgumentList $argList -NoNewWindow -PassThru -RedirectStandardOutput $log -RedirectStandardError $err
    $null = $proc.Handle

    $start = Get-Date
    $shown = 0
    $tick = 0
    $loads = New-Object System.Collections.Generic.List[int]
    while (-not $proc.HasExited) {
        $lines = @(Get-Content $log -ErrorAction SilentlyContinue)
        if ($lines.Count -gt $shown) {
            Write-Host ("`r" + (' ' * 78) + "`r") -NoNewline
            $lines[$shown..($lines.Count - 1)] | ForEach-Object { Write-OutputLine $_ }
            $shown = $lines.Count
        }
        $load = Get-CpuLoad
        $loads.Add($load)
        $elapsed = Format-Span ((Get-Date) - $start)
        Write-Host ("`r  {0} {1,-9} elapsed {2}   CPU " -f $spinner[$tick % 4], $Tool, $elapsed) -NoNewline -ForegroundColor Magenta
        Write-Host ("{0,3}%   " -f $load) -NoNewline -ForegroundColor (Get-LoadColor $load)
        $tick++
        Start-Sleep -Milliseconds 700
    }
    $proc.WaitForExit()
    Write-Host ("`r" + (' ' * 78) + "`r") -NoNewline
    $lines = @(Get-Content $log -ErrorAction SilentlyContinue)
    if ($lines.Count -gt $shown) { $lines[$shown..($lines.Count - 1)] | ForEach-Object { Write-OutputLine $_ } }

    if ($proc.ExitCode -ne 0) {
        Get-Content $err -Tail 15 | ForEach-Object { Write-Host ("  " + $_) -ForegroundColor Red }
        throw "$Tool benchmark failed with exit code $($proc.ExitCode)"
    }

    $rateLine = $lines | Where-Object { $_ -match '([\d,]+) rows/s' } | Select-Object -Last 1
    if (-not $rateLine) { throw "No rows/s figure found in $log" }
    [void]($rateLine -match '([\d,]+) rows/s')
    $result = [pscustomobject]@{
        tool          = $Tool
        rows_per_s    = [double]($Matches[1] -replace ',', '')
        wall_seconds  = [math]::Round(((Get-Date) - $start).TotalSeconds, 1)
        avg_cpu_load  = [int]($loads | Measure-Object -Average).Average
        python        = (& $Python -I -c "import sys;print(sys.version.split()[0])")
        finished_utc  = (Get-Date).ToUniversalTime().ToString('s') + 'Z'
    }
    $result | ConvertTo-Json | Set-Content -Path $resultPath -Encoding UTF8
    Write-Host ("  saved {0}" -f $resultPath) -ForegroundColor DarkGreen
    $result
}

foreach ($path in @($Python, $Data)) {
    if (-not (Test-Path $path)) { throw "Missing $path" }
}
New-Item -ItemType Directory -Force -Path $OutDir | Out-Null

Write-Banner 'MaskOps vs Presidio, like-for-like, 1M rows' 'Cyan'
Write-Host '  Sampling CPU load...' -ForegroundColor DarkGray
$idle = [int](@(1..5 | ForEach-Object { Get-CpuLoad; Start-Sleep -Milliseconds 400 }) | Measure-Object -Average).Average
Write-Host '  Idle load: ' -NoNewline
Write-Host ("{0}%" -f $idle) -ForegroundColor (Get-LoadColor $idle)
if ($idle -gt $QuietLoad) {
    Write-Host ("  Machine is not quiet (over {0}%). Close heavy apps for a clean number." -f $QuietLoad) -ForegroundColor Red
    for ($i = 10; $i -gt 0; $i--) {
        Write-Host ("`r  Starting anyway in {0,2}s, Ctrl+C to abort " -f $i) -NoNewline -ForegroundColor Yellow
        Start-Sleep -Seconds 1
    }
    Write-Host ''
}

Write-Banner 'MaskOps' 'Green'
$maskops = Invoke-Tool 'maskops' @()
Write-Banner 'Presidio (full 1M rows, about 10-25 min)' 'Yellow'
$presidio = Invoke-Tool 'presidio' @('--presidio-full-rows')

$speedup = $maskops.rows_per_s / $presidio.rows_per_s
Write-Banner 'Summary' 'Cyan'
'{0,-10} {1,14} {2,14} {3,10}' -f 'Tool', 'rows/s', '1M rows', 'avg CPU' | Write-Host -ForegroundColor DarkGray
foreach ($r in @($maskops, $presidio)) {
    $seconds = 1000000 / $r.rows_per_s
    $time = if ($seconds -ge 60) { '{0:N1} min' -f ($seconds / 60) } else { '{0:N2} s' -f $seconds }
    Write-Host ('{0,-10} {1,14:N0} {2,14}' -f $r.tool, $r.rows_per_s, $time) -NoNewline -ForegroundColor White
    Write-Host ('{0,9}%' -f $r.avg_cpu_load) -ForegroundColor (Get-LoadColor $r.avg_cpu_load)
}
Write-Host ''
Write-Host ('  Speedup: ~{0:N0}x' -f $speedup) -ForegroundColor Green
if ($maskops.avg_cpu_load -gt $QuietLoad -or $presidio.avg_cpu_load -gt $QuietLoad) {
    Write-Host '  Warning: background load was high during a run; this ratio is not a clean number.' -ForegroundColor Red
}
