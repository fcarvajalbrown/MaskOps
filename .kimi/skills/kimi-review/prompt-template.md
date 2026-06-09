# Code Correctness Review — MaskOps

You are reviewing a diff for MaskOps, a Rust-based PII masking library (Polars expression plugin). The Rust core compiles to a cdylib. It processes DataFrames with millions of rows — correctness is critical and bugs may silently pass PII through unmasked.

## What to look for

1. **Incorrect pattern matching** — regex that fails to match valid PII (false negatives) or matches non-PII (false positives), including Unicode edge cases, optional separators, and country-specific formats
2. **Edge case gaps** — empty string input, strings with only whitespace, NULL/None values, single-character inputs, inputs at the exact length boundary of a valid PII token
3. **API contract violations** — a function that claims to mask returns the original value, a contains check returns wrong bool, FPE that changes the length or format of the output
4. **Unsafe Option/Result handling** — `.unwrap()` or `.expect()` on values that can legitimately be None/Err at runtime (not test code)
5. **Integer overflow/underflow** — index arithmetic that could panic on very long or very short strings
6. **Incorrect masking scope** — masking too little (leaving PII characters visible) or too much (destroying non-PII structure like IBAN check digits needed for format preservation)
7. **Missing test coverage** — a new code path or branch that has no corresponding test case in `tests/`

## Test files in this repo

{{TEST_FILES}}

## Diff to review

```diff
{{DIFF}}
```

## Output format

List each finding as:

**[SEVERITY]** `file.rs:line` — description
Trigger: what input or condition exposes the bug
Fix: specific change needed

Severity: HIGH (PII leaks through unmasked, or function panics at runtime) / MEDIUM (wrong output for a subset of valid inputs) / LOW (missing test, minor contract drift)

If no issues found, respond with exactly: `No issues found.`
