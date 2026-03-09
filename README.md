# maskops

> High-speed PII masking as a native Polars plugin — powered by Rust.

**maskops** extends Polars with zero-overhead PII detection and masking expressions.
No NLP models. No intermediate files. Just regex + Rust running directly on Arrow buffers.

## Why

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