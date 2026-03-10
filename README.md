# MaskOps

> High-speed PII masking as a native Polars plugin — powered by Rust.

**MaskOps** extends Polars with zero-overhead PII detection and masking expressions.
No NLP models. No intermediate files. Just regex + Rust running directly on Arrow buffers.

## How It Works

```mermaid
flowchart LR
    A[🐍 Python\nPolars DataFrame] -->|mask_pii / contains_pii| B[Polars\nExpression Engine]
    B -->|Arrow buffer\nzero-copy| C[🦀 Rust Core\nmaskops]
    C -->|IBAN / VAT| D[Masked\nSeries]
    C -->|Email / Phone| D
    C -->|IP / Credit Card| D
    C -->|DNI / NIE / NIN| D
    C -->|Personalausweis| D
    C -->|RUT / CPF / CURP| D
    D -->|back to Python| A
    style A fill:#306998,color:#fff
    style C fill:#CE422B,color:#fff
    style B fill:#2E2E2E,color:#fff
    style D fill:#2E7D32,color:#fff
```

No Python objects created per row. No NLP model loaded. No intermediate files.

- **Presidio** is heavy — it spins up NLP models for structured CSV data that doesn't need them.
- **Pure Python regex** on large DataFrames is slow.
- **MaskOps** compiles to a native `.so` that Polars calls directly — same speed as built-in expressions.

## Architecture

```
maskops/
├── Cargo.toml               # Rust dependencies
├── pyproject.toml           # maturin build backend + PyPI metadata
├── src/
│   ├── lib.rs               # Polars expression registration (mask_pii, contains_pii)
│   └── patterns/
│       ├── mod.rs           # mask_all() and contains_any_pii() aggregators
│       ├── iban.rs          # IBAN regex + masking
│       ├── vat.rs           # EU VAT regex + masking
│       ├── email.rs         # Email regex + masking (local part)
│       ├── phone.rs         # E.164 phone regex + masking
│       ├── ip.rs            # IPv4/IPv6 regex + masking
│       ├── latam_id.rs      # RUT (Chile), CPF (Brazil), CURP (Mexico)
│       ├── european_id.rs   # DNI/NIE (Spain), NIN (UK), Personalausweis (Germany)
│       ├── credit_card.rs   # Visa, Mastercard, Amex, Discover, Maestro + Luhn
│       └── country_codes.rs # Country prefix lookup table
├── maskops/
│   └── __init__.py          # Python API via register_plugin_function
├── benchmarks/
│   └── benchmark.py         # Per-family throughput benchmarks (1M rows)
└── tests/
    ├── test_masking.py      # pytest suite (66 tests)
    ├── generate_fixtures.py # Faker-based EU test data generator
    └── fixtures/            # Generated CSVs (gitignored)
```

The Rust layer operates directly on Arrow buffers — zero Python object overhead per row.
Each PII type is its own module: adding a new pattern = new file + one line in `mod.rs`.

## Install

```bash
pip install maskops
```

## Usage

```python
import polars as pl
import maskops

df = pl.read_csv("payments.csv")

# Mask all PII in a column
df.with_columns(maskops.mask_pii("notes"))

# Filter rows that contain PII
df.filter(maskops.contains_pii("free_text"))
```

## Supported patterns (v0.1.4)

| Pattern | Example input | Masked output |
|---------|--------------|---------------|
| IBAN    | `DE89370400440532013000` | `DE89******************` |
| EU VAT  | `DE123456789` | `DE*********` |
| Email   | `john.doe@example.com` | `********@example.com` |
| Phone   | `+14155552671` | `+1**********` |
| IP Address | `192.168.1.100` | `192.168.*.*` |
| RUT (Chile) | `76.354.771-K` | `**********-K` |
| CPF (Brazil) | `529.982.247-25` | `*********-25` |
| CURP (Mexico) | `BADD110313HCMLNS09` | `******************` |
| DNI (Spain) | `12345678Z` | `********Z` |
| NIE (Spain) | `X1234567L` | `********L` |
| NIN (UK) | `AB 12 34 56 C` | `*********** C` |
| Personalausweis (Germany) | `T220001293` | `**********` |
| Credit Card (Visa/MC/Amex/Discover/Maestro) | `4111111111111111` | `411111******1111` |

Tested against 8 EU locales: DE, FR, ES, IT, NL, PL, PT, SE.
Email and phone follow RFC 5322 and E.164 respectively.
RUT and CPF include Módulo 11 check digit validation.
DNI and NIE include modulo 23 check letter validation.
Credit cards include Luhn validation — format-only matches are rejected.
Personalausweis and NIN: format-only matching; check digit validation pending (v0.2.0+).

## Roadmap

- [x] Email, phone patterns
- [x] IP address patterns
- [x] Latin American IDs (RUT, CPF, CURP)
- [x] European IDs (DNI/NIE Spain, NIN UK, Personalausweis Germany)
- [x] Credit cards (Visa, Mastercard, Amex, Discover, Maestro) with Luhn validation
- [x] PyPI publish via GitHub Actions
- [ ] Format-Preserving Encryption (FPE/FF3-1) for reversible masking
- [ ] Check digit validation for Personalausweis (Germany) and NIN (UK)
- [ ] Benchmark vs Presidio
- [ ] Parquet streaming support

## Build from source

### Windows (PowerShell)

```powershell
python -m venv .venv
.venv\Scripts\activate
pip install maturin faker polars pytest
maturin develop --release
python tests/generate_fixtures.py
pytest tests/ -v
```

### Linux / macOS

```bash
python -m venv .venv
source .venv/bin/activate
pip install maturin faker polars pytest
maturin develop --release
python tests/generate_fixtures.py
pytest tests/ -v
```

## Key dependency versions

| Package | Version |
|---------|---------|
| pyo3 | 0.21 |
| pyo3-polars | 0.18 |
| polars | 0.46 |
| maturin | >=1.7,<2.0 |

> **Note:** pyo3 must be 0.21 to match pyo3-polars 0.18. Do not bump pyo3 independently.

## License

MIT

## Benchmarks

Tested on 1,000,000 rows, Intel i-series CPU, Python 3.11, Ubuntu (CI).

Median of 3 runs per benchmark.
Baseline uses equivalent regex coverage to maskops per family.

> **Note on per-family benchmarks:** maskops always runs the full pattern set —
> there is no per-family dispatch. A "Credit Card only" benchmark still pays for
> IBAN, VAT, email, phone, LatAm ID, and EU ID checks. The Python baseline only
> runs one regex. This is why maskops underperforms on isolated families with
> dense PII. The advantage emerges when all patterns are active simultaneously,
> which is the realistic production case.

### EU patterns (IBAN, VAT, Email, Phone)

| Profile | Expression | Time | Rows/s | Python re | Speedup |
|---------|-----------|------|--------|-----------|---------|
| clean | `mask_pii` | 2.252s | 444,043 | 2.822s | **1.3×** |
| clean | `contains_pii` | 0.786s | 1,272,071 | — | — |
| dense | `mask_pii` | 3.750s | 266,674 | 2.539s | 0.7× |
| dense | `contains_pii` | 0.199s | 5,030,634 | — | — |
| mixed | `mask_pii` | 4.200s | 238,086 | 2.843s | 0.7× |
| mixed | `contains_pii` | 0.418s | 2,394,223 | — | — |

### LatAm patterns (RUT, CPF, CURP)

| Profile | Expression | Time | Rows/s | Python re | Speedup |
|---------|-----------|------|--------|-----------|---------|
| clean | `mask_pii` | 2.605s | 383,825 | 2.320s | 0.9× |
| clean | `contains_pii` | 0.793s | 1,260,977 | — | — |
| dense | `mask_pii` | 2.952s | 338,780 | 1.657s | 0.6× |
| dense | `contains_pii` | 0.626s | 1,597,899 | — | — |
| mixed | `mask_pii` | 2.798s | 357,436 | 1.793s | 0.6× |
| mixed | `contains_pii` | 0.695s | 1,438,803 | — | — |

> RUT and CPF include Módulo 11 check digit validation per row — this is the cost of zero false positives.

### Network patterns (IP)

| Profile | Expression | Time | Rows/s | Python re | Speedup |
|---------|-----------|------|--------|-----------|---------|
| clean | `mask_pii` | 2.364s | 422,974 | 2.060s | 0.9× |
| clean | `contains_pii` | 0.796s | 1,255,531 | — | — |
| dense | `mask_pii` | 2.496s | 400,576 | 1.469s | 0.6× |
| dense | `contains_pii` | 0.208s | 4,815,812 | — | — |
| mixed | `mask_pii` | 2.453s | 407,628 | 1.665s | 0.7× |
| mixed | `contains_pii` | 0.362s | 2,763,168 | — | — |

### Credit card patterns (Visa, Mastercard, Amex, Discover, Maestro)

| Profile | Expression | Time | Rows/s | Python re | Speedup |
|---------|-----------|------|--------|-----------|---------|
| clean | `mask_pii` | 2.333s | 428,549 | 0.952s | 0.4× |
| clean | `contains_pii` | 0.794s | 1,259,331 | — | — |
| dense | `mask_pii` | 2.755s | 362,944 | 0.974s | 0.4× |
| dense | `contains_pii` | 0.594s | 1,684,403 | — | — |
| mixed | `mask_pii` | 2.696s | 370,982 | 0.981s | 0.4× |
| mixed | `contains_pii` | 0.658s | 1,520,816 | — | — |

> Luhn validation runs per candidate match — this eliminates false positives at the cost of single-family throughput.

### European ID patterns (DNI/NIE, NIN, Personalausweis)

| Profile | Expression | Time | Rows/s | Python re | Speedup |
|---------|-----------|------|--------|-----------|---------|
| clean | `mask_pii` | 2.316s | 431,758 | 1.397s | 0.6× |
| clean | `contains_pii` | 0.816s | 1,225,246 | — | — |
| dense | `mask_pii` | 2.558s | 390,956 | 1.082s | 0.4× |
| dense | `contains_pii` | 0.636s | 1,573,124 | — | — |
| mixed | `mask_pii` | 3.754s | 266,371 | 1.669s | 0.4× |
| mixed | `contains_pii` | 0.969s | 1,031,843 | — | — |

### All patterns active

> This is the realistic production workload — all 13 pattern types running simultaneously.
> maskops is up to **5.4× faster** than an equivalent pure Python approach.
> `contains_pii` reaches 2.0M rows/s on mixed data — use it to pre-filter before masking in hot pipelines.

| Profile | Expression | maskops | Python `re` | Speedup |
|---------|-----------|---------|-------------|---------|
| clean | `mask_pii` | 3.445s | 18.459s | **5.4×** |
| clean | `contains_pii` | 1.176s | — | — |
| dense | `mask_pii` | 3.966s | 5.949s | **1.5×** |
| dense | `contains_pii` | 0.626s | — | — |
| mixed | `mask_pii` | 2.988s | 6.966s | **2.3×** |
| mixed | `contains_pii` | 0.502s | — | — |

> maskops throughput stays roughly flat as pattern count grows — Python regex degrades with each additional pattern.
> The clean profile gap (5.4×) reflects Python's overhead of compiling and scanning a large combined regex on short-circuit misses.

### vs Microsoft Presidio (estimated)

Presidio processes structured DataFrames via `presidio-structured`, which runs a spaCy NLP pipeline per row. Based on community reports and the architecture:

| Tool | Throughput (structured data) | Requires NLP model |
|------|------------------------------|-------------------|
| maskops | ~252K–5.0M rows/s (measured) | No |
| Presidio (regex-only recognizers) | ~10–50K rows/s* | No |
| Presidio (spaCy NER) | ~1–5K rows/s* | Yes (250MB+) |

\* Estimated from community benchmarks and Presidio's own documentation noting it is "not optimized for bulk structured data." [Microsoft confirmed no official throughput benchmarks exist.](https://github.com/microsoft/presidio/discussions/1226)

**maskops is purpose-built for structured data pipelines where Presidio's NLP overhead is unnecessary.**