# Agent Notes

## CI / Coverage

- **Ubuntu + Python 3.12 is excluded from CI** because the compiled Rust extension
  fails to load (`dlopen` error) on `ubuntu-latest` with Python 3.12. The same
  tests pass on Windows and on Ubuntu 3.10/3.11.
- Coverage is uploaded from the **Ubuntu 3.11** job (was 3.12 before exclusion).
  See `.github/workflows/ci.yml`.

## TODO

- [ ] Fix Rust extension `dlopen` failure on `ubuntu-latest` + Python 3.12.
  Suspected causes: `maturin develop` editable-install path mismatch, or
  missing `.so` in the source tree when Polars tries to load the plugin.
- [ ] Add Rust coverage (e.g. `cargo tarpaulin`) and merge with Python report
  on Codecov for an accurate combined number.
