# Contributing

## Contributor License Agreement (CLA)

**All contributions require signing the CLA before a PR can be merged.**

The CLA grants the project maintainer the right to use your contribution under any license (including commercial). It does not transfer copyright — you keep ownership of your code.

**How to sign:** Open a pull request and post a comment containing exactly:

> I have read the CLA and agree to its terms.

The CLA bot will record your signature automatically. You only sign once; it covers all future contributions.

The full agreement is in [CLA.md](CLA.md). If you have questions, open an issue or email [fcarvajalbrown@gmail.com](mailto:fcarvajalbrown@gmail.com).

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
