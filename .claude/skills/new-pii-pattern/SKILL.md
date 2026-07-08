---
name: new-pii-pattern
description: Scaffold a new PII pattern end-to-end for MaskOps — new src/patterns/<family> file, full wiring into every aggregator in mod.rs, lib.rs registration, tests, and changelog/roadmap updates. Use when adding support for a new identifier type (a national ID, tax number, health ID, card scheme, etc.).
---

# Adding a new PII pattern

Adding a pattern is a fixed, wide ritual: one new file, then wiring into **every** aggregator in `src/patterns/mod.rs`, plus `lib.rs` registration and tests. Missing one aggregator ships a pattern that works in `mask_pii` but silently does nothing in `extract_pii`, `mask_pii_audit`, or the `patterns=` selector. Follow this checklist in order and do not skip steps.

## Step 0 — classify before writing any code

Decide and record (in the PR description, not in code comments — this codebase has zero comments):
- **Which regulation / jurisdiction** the identifier belongs to and its `snake_case` pattern name (e.g. `br_cnh`).
- **Digit-based or non-digit?** Digit-based PII (card, phone, national numbers) is FPE-eligible and reversible; non-digit PII (email, IBAN, alphanumeric IDs) is always asterisked. This decides which pipelines you wire into.
- **False-positive guard.** Every pattern must validate structure — a checksum (Luhn, mod-97, mod-11, verifier digit) or strict length/format — never a bare loose regex. Identify the real validation rule before coding.

## Step 1 — create the pattern file

Put it under the matching family directory: `src/patterns/{eu,latam,us,apac,mea,healthcare,financial,contact}/<name>.rs`, and add `pub mod <name>;` to that family's `mod.rs`.

Copy the shape of an existing pattern of the same class rather than writing from scratch:
- **Non-digit (asterisk-only) example:** `src/patterns/eu/iban.rs` — exposes `mask_iban`, `contains_iban`, `extract_iban`, `mask_iban_counted`.
- **Digit-based (FPE + consistent capable) example:** `src/patterns/apac/canada_sin.rs` — exposes `mask_sin`, `contains_sin`, `extract_sin`, `mask_sin_fpe`, `mask_sin_consistent`, `mask_sin_counted`.

A digit-based pattern must expose all six functions; a non-digit pattern exposes the four (no `_fpe`, no `_consistent`). Use `once_cell` for the compiled `Regex`, and `replace_counted` / `mask_family` / `reinsert_digits` / `TokenClaims` from `src/patterns/mod.rs` — do not reimplement them.

## Step 2 — wire into src/patterns/mod.rs

This is where patterns are most often half-added. Every location below lives in `src/patterns/mod.rs`:

1. Add the `use crate::patterns::<family>::<name>::{...}` imports (mask/contains/extract, plus `_fpe`/`_consistent`/`_counted` for digit-based).
2. Add the name string to `PATTERN_NAMES`.
3. **Asterisk pipeline:** call your `mask_*` in `mask_non_digit()` (non-digit PII) **or** `mask_digit()` (digit PII). Non-digit runs before digit — respect that ordering.
4. **FPE pipeline** (digit-based only): add `mask_*_fpe(&s, cipher, &claims)` to `mask_digit_fpe()`.
5. **Consistent pipeline** (digit-based only): add `mask_*_consistent(&s, hasher, &claims)` to `mask_digit_consistent()`.
6. **Selector variants:** add match arms to `mask_all_selected`, `mask_all_selected_fpe`, `mask_all_selected_consistent`, and `contains_any_selected`.
7. **Whole-value aggregators:** add to `contains_any_pii()`.
8. **Struct fields + population:** add a field to `ExtractResult` and populate it in `extract_all()` and `extract_all_selected()`; add a field to `AuditCounts` and populate it in `mask_all_audit()` and `mask_all_audit_selected()` using your `_counted` fn.

## Step 3 — register in lib.rs

Expose the pattern through the Polars expressions in `src/lib.rs` (the `mask_pii`, `contains_pii`, `mask_pii_fpe`, `extract_pii`, `mask_pii_audit` registrations). Mirror an existing pattern of the same class.

## Step 4 — tests

Add a test class in `tests/` (mirror an existing one, e.g. the SIN or IBAN class) covering:
- **True positives** — realistic valid identifiers get masked.
- **True negatives** — near-misses (wrong checksum, wrong length, lookalike numbers) are left untouched. This is the important half.
- **Round-trip** (FPE-eligible patterns only) — `decrypt(mask_fpe(x)) == x`, and format/length preserved.
- If the identifier needs generated fixtures, add a generator to `tests/generate_fixtures.py` that produces valid check digits.

## Step 5 — build, run, verify

```
maturin develop --release
python tests/generate_fixtures.py
pytest tests/ -v
```

`maturin develop --release` must be re-run after the Rust change before pytest.

## Step 6 — docs

- Add one line to `docs/CHANGELOG.md` under the unreleased section (new pattern = user-visible).
- Check off / add the capability under `## Roadmap` in `README.md` if it is a roadmap item.
- Do **not** bump any version file — release-please owns versions.

## Final self-check

Run the `pii-pattern-reviewer` subagent on the diff to confirm classification, false-positive guarding, complete wiring, and test coverage before opening the PR. For anything touching FPE/crypto, also run `crypto-compliance-reviewer`.
