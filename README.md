# maskops

> High-speed PII masking as a native Polars plugin — powered by Rust.

**maskops** extends Polars with zero-overhead PII detection and masking expressions.
No NLP models. No intermediate files. Just regex + Rust running directly on Arrow buffers.

## How It Works

```mermaid
flowchart LR
    A[🐍 Python\nPolars DataFrame] -->|mask_pii / contains_pii| B[Polars\nExpression Engine]
    B -->|Arrow buffer\nzero-copy| C[🦀 Rust Core\nmaskops]
    C -->|IBAN regex| D[Masked\nSeries]
    C -->|VAT regex| D
    C -->|Email regex| D
    C -->|Phone regex| D
    D -->|back to Python| A

    style A fill:#306998,color:#fff
    style C fill:#CE422B,color:#fff
    style B fill:#2E2E2E,color:#fff
    style D fill:#2E7D32,color:#fff
```

No Python objects created per row. No NLP model loaded. No intermediate files.

- **Presidio** is heavy — it spins up NLP models for structured CSV data that doesn't need them.
- **Pure Python regex** on large DataFrames is slow.
- **maskops** compiles to a native `.so` that Polars calls directly — same speed as built-in expressions.

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
│       └── country_codes.rs # Country prefix lookup table
├── maskops/
│   └── __init__.py          # Python API via register_plugin_function
├── benchmarks/
│   └── benchmark.py         # Per-family throughput benchmarks (1M rows)
└── tests/
    ├── test_masking.py      # pytest suite (53 tests)
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

## Supported patterns (v0.1.3)

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

Tested against 8 EU locales: DE, FR, ES, IT, NL, PL, PT, SE.
Email and phone follow RFC 5322 and E.164 respectively.
RUT and CPF include Módulo 11 check digit validation.

## Roadmap

- [x] Email, phone patterns
- [x] IP address patterns
- [x] Latin American IDs (RUT, CPF, CURP)
- [x] PyPI publish via GitHub Actions
- [ ] Format-Preserving Encryption (FPE/FF3-1) for reversible masking
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

Tested on 1,000,000 rows, Intel i-series CPU, Python 3.14, Windows.

Median of 3 runs per benchmark. 
Baseline uses equivalent regex coverage to maskops per family.

Benchmarks are broken down by pattern family so you only pay for what you use.

### EU patterns (IBAN, VAT, Email, Phone)

| Profile | Expression | Time | Rows/s | Python re | Speedup |
|---------|-----------|------|--------|-----------|---------|
| clean | `mask_pii` | 1.285s | 778,053 | 2.779s | **2.2×** |
| clean | `contains_pii` | 0.378s | 2,645,946 | — | — |
| dense | `mask_pii` | 1.989s | 502,750 | 1.745s | 0.9× |
| dense | `contains_pii` | 0.130s | 7,690,710 | — | — |
| mixed | `mask_pii` | 1.869s | 535,082 | 1.943s | 1.0× |
| mixed | `contains_pii` | 0.180s | 5,546,207 | — | — |

### LatAm patterns (RUT, CPF, CURP)

| Profile | Expression | Time | Rows/s | Python re | Speedup |
|---------|-----------|------|--------|-----------|---------|
| clean | `mask_pii` | 1.339s | 746,721 | 2.283s | **1.7×** |
| clean | `contains_pii` | 0.375s | 2,668,890 | — | — |
| dense | `mask_pii` | 2.033s | 491,792 | 1.618s | 0.8× |
| dense | `contains_pii` | 0.632s | 1,582,181 | — | — |
| mixed | `mask_pii` | 1.884s | 530,879 | 1.783s | 0.9× |
| mixed | `contains_pii` | 0.588s | 1,701,419 | — | — |

> RUT and CPF include Módulo 11 check digit validation per row — this is the cost of zero false positives.

### Network patterns (IP)

| Profile | Expression | Time | Rows/s | Python re | Speedup |
|---------|-----------|------|--------|-----------|---------|
| clean | `mask_pii` | 1.413s | 707,909 | 2.969s | **2.1×** |
| clean | `contains_pii` | 0.376s | 2,658,206 | — | — |
| dense | `mask_pii` | 2.346s | 426,330 | 2.192s | 0.9× |
| dense | `contains_pii` | 0.324s | 3,081,799 | — | — |
| mixed | `mask_pii` | 2.213s | 451,958 | 2.423s | 1.1× |
| mixed | `contains_pii` | 0.385s | 2,596,104 | — | — |

### All patterns active

| Profile | Expression | maskops | Python `re` | Speedup |
|---------|-----------|---------|-------------|---------|
| clean | `mask_pii` | 1.997s | 6.350s | **3.2×** |
| clean | `contains_pii` | 0.562s | — | — |
| dense | `mask_pii` | 1.910s | 3.232s | **1.7×** |
| dense | `contains_pii` | 0.319s | — | — |
| mixed | `mask_pii` | 1.869s | 3.594s | **1.9×** |
| mixed | `contains_pii` | 0.326s | — | — |

> maskops throughput stays flat as pattern count grows — Python regex degrades linearly.
> With all 8 patterns active, maskops is up to 3.2× faster than an equivalent pure Python approach.
> `contains_pii` is consistently the fastest path — use it for filtering before masking in hot pipelines.

### vs Microsoft Presidio (estimated)

Presidio processes structured DataFrames via `presidio-structured`, which runs a spaCy NLP pipeline per row. Based on community reports and the architecture:

| Tool | Throughput (structured data) | Requires NLP model |
|------|------------------------------|-------------------|
| maskops | ~427K–7.7M rows/s (measured) | No |
| Presidio (regex-only recognizers) | ~10–50K rows/s* | No |
| Presidio (spaCy NER) | ~1–5K rows/s* | Yes (250MB+) |

\* Estimated from community benchmarks and Presidio's own documentation noting it is "not optimized for bulk structured data." [Microsoft confirmed no official throughput benchmarks exist.](https://github.com/microsoft/presidio/discussions/1226)

**maskops is purpose-built for structured data pipelines where Presidio's NLP overhead is unnecessary.**