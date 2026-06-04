---
name: kimi-optimize
description: Run a Kimi K2.6 Rust performance review of the current diff. Use before every PR.
disable-model-invocation: true
allowed-tools: Bash Read
---

## Current diff
!`git diff HEAD`

## Benchmark context
!`cargo criterion --benches --no-run --message-format human 2>&1 | tail -20`

## Instructions

### 1. Build the task brief
Read [prompt-template.md](prompt-template.md). Fill in:
- `{{DIFF}}` — the full diff injected above
- `{{BENCH_CONTEXT}}` — the benchmark context injected above

Write the filled brief to `/tmp/kimi_optimize_brief.md`.

### 2. Run Kimi

Run this command:

```
kimi --quiet -p "$(cat /tmp/kimi_optimize_brief.md)"
```

### 3. Filter suggestions

Discard any suggestion that:
- Requires changing correctness behavior or test expectations
- Introduces `unsafe` without a documented safety invariant
- Changes public API signatures (`mask_pii`, `contains_pii`, `mask_pii_fpe`)

### 4. Report

For each confirmed suggestion:
- **Impact:** estimated gain (high / medium / low)
- **Location:** file:line
- **Before / After:** concrete Rust code snippets
