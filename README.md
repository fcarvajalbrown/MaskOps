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
├── Cargo.toml               # Rust dependencies (pyo3 0.21, pyo3-polars 0.18, polars 0.46)
├── pyproject.toml           # maturin build backend + PyPI metadata
├── src/
│   ├── lib.rs               # Polars expression registration (mask_pii, contains_pii)
│   └── patterns/
│       ├── mod.rs           # mask_all() and contains_any_pii() aggregators
│       ├── iban.rs          # IBAN regex + masking
│       └── vat.rs           # EU VAT regex + masking
├── maskops/
│   └── __init__.py          # Python API via register_plugin_function
└── tests/
    ├── test_masking.py      # pytest suite
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

## Supported patterns (v0.1)

| Pattern | Example input | Masked output |
|---------|--------------|---------------|
| IBAN    | `DE89370400440532013000` | `DE89******************` |
| EU VAT  | `DE123456789` | `DE*********` |

Tested against 8 EU locales: DE, FR, ES, IT, NL, PL, PT, SE.

## Roadmap

- [ ] Email, phone, IP address patterns
- [ ] Format-Preserving Encryption (FPE/FF3-1) for reversible masking
- [ ] Latin American IDs (RUT, CPF, CURP)
- [ ] Benchmark vs Presidio
- [ ] Parquet streaming support
- [ ] PyPI publish via GitHub Actions

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

Or just run the setup script:

```powershell
# Windows
.\setup.bat

# Linux/macOS
./setup.sh
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

### maskops throughput

| Profile | Expression | Time | Rows/s | MB/s |
|---------|-----------|------|--------|------|
| clean (no PII) | `mask_pii` | 0.404s | 2,477,599 | 54.5 |
| clean (no PII) | `contains_pii` | 0.169s | 5,915,970 | 130.2 |
| dense (all PII) | `mask_pii` | 1.385s | 722,104 | 15.9 |
| dense (all PII) | `contains_pii` | 0.059s | 16,987,879 | 373.7 |
| mixed (50/50) | `mask_pii` | 0.760s | 1,315,407 | 28.9 |
| mixed (50/50) | `contains_pii` | 0.133s | 7,498,315 | 165.0 |

### vs pure Python regex (same machine)

| Profile | maskops `mask_pii` | Python `re` | Speedup |
|---------|-------------------|-------------|---------|
| clean | 0.404s | 0.925s | **2.3×** |
| dense | 1.385s | 1.653s | **1.2×** |
| mixed | 0.760s | 1.337s | **1.8×** |

> On clean and mixed data maskops is consistently faster. On dense data (every row is a full IBAN) both are regex-bound — the bottleneck is the pattern itself, not Python overhead.

### vs Microsoft Presidio (estimated)

Presidio processes structured DataFrames via `presidio-structured`, which runs a spaCy NLP pipeline per row. Based on community reports and the architecture:

| Tool | Throughput (structured data) | Requires NLP model |
|------|------------------------------|-------------------|
| maskops | ~700K–17M rows/s | No |
| Presidio (regex-only recognizers) | ~10–50K rows/s* | No |
| Presidio (spaCy NER) | ~1–5K rows/s* | Yes (250MB+) |

\* Estimated from community benchmarks and Presidio's own documentation noting it is "not optimized for bulk structured data." [Microsoft confirmed no official throughput benchmarks exist.](https://github.com/microsoft/presidio/discussions/1226)

**maskops is purpose-built for structured data pipelines where Presidio's NLP overhead is unnecessary.**