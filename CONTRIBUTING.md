# Contributing

## Contributor License Agreement (CLA)

By submitting a pull request you automatically accept the [CLA](CLA.md). No signature or bot interaction is required.

The CLA grants the project maintainer the right to use your contribution under any license (including commercial). It does not transfer copyright — you keep ownership of your code. If you have questions, open an issue or email [fcarvajalbrown@gmail.com](mailto:fcarvajalbrown@gmail.com).

---

## Adding a new PII pattern

1. Create `src/patterns/<group>/<family>.rs` — implement `mask_<family>` and/or `contains_<family>`. Groups: `eu/`, `latam/`, `contact/`, `financial/`, `us/`, `healthcare/`.
2. Declare the module in `src/patterns/<group>/mod.rs`, then import the functions in `src/patterns/mod.rs` and call them inside `mask_non_digit` or `mask_digit` (and the FPE equivalent if digit-based).
3. Add a compliance category docstring to the new module: which regulation, FPE or asterisk-only, and what validation prevents false positives.
4. Add fixture generation to `tests/generate_fixtures.py` and a test class in `tests/test_masking.py`.
5. Run `maturin develop --release && python tests/generate_fixtures.py && pytest tests/ -v`.

## Dev setup

```bash
python -m venv .venv && source .venv/bin/activate
pip install maturin faker polars pytest
maturin develop --release
```

## Guidelines

- One pattern per file, one PR per pattern.
- **No comments of any kind** — no `//`, `//!`, `///`, `/* */` in Rust; no `#` or docstrings in Python.
- Bug fixes must address the root cause — no patching test parameters.
- Commit format: `<type>(<scope>): <description>` (lowercase, present-tense imperative).
