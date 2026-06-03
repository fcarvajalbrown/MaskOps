# Contributing

## Adding a new PII pattern

1. Create `src/patterns/<family>.rs` — implement `mask_<family>` and/or `contains_<family>`.
2. Import it in `src/patterns/mod.rs` and call it inside `mask_non_digit` or `mask_digit` (and the FPE equivalent if digit-based).
3. Add fixture generation to `tests/generate_fixtures.py` and a test class in `tests/test_masking.py`.
4. Run `maturin develop --release && python tests/generate_fixtures.py && pytest tests/ -v`.

## Dev setup

```bash
python -m venv .venv && source .venv/bin/activate
pip install maturin faker polars pytest
maturin develop --release
```

## Guidelines

- One pattern per file, one PR per pattern.
- No block comments; 1-line max.
- Bug fixes must address the root cause — no patching test parameters.
- Commit format: `<type>(<scope>): <description>` (lowercase, present-tense imperative).
