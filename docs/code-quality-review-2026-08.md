# Code quality review — August 2026

Full-repository review carried out at v2.1.0, before starting the v2.2 milestone. Criteria:
readability, single responsibility, no duplicated knowledge, idiomatic Rust and Python, and the
repository's own rule that names and types are the only documentation. The mechanical pass was
`cargo clippy --all-targets -- -W clippy::pedantic` (194 warnings, 100 auto-fixable).

Two defects were confirmed by reproducing them through the public Python API. Both are fixed on
`fix/selected-pattern-order`. Everything else in this document is logged, not fixed.

---

## Fixed

### 1. Selected patterns were applied in the caller's list order

`mask_pii`, `mask_pii_fpe` and `mask_pii_consistent` iterated the caller's `patterns` list directly,
so the order of the argument changed what was masked. The full pipeline masks non-digit families
before digit families for exactly this reason, and `mask_all_audit_selected` already walked a fixed
order — the three selected-mask loops did not.

Reproduction at v2.1.0:

```
mask_pii('c', patterns=['email','credit_card']) -> contacto ****************@example.com
mask_pii('c', patterns=['credit_card','email']) -> contacto 411111**********@example.com
mask_pii_audit('c', patterns=['credit_card','email']) -> contacto ****************@example.com
```

The card BIN and part of the email local part survived, and `mask_pii` disagreed with
`mask_pii_audit` on the same selection.

Fix: `PATTERN_NAMES` was reordered to be the canonical pipeline order and is now the single source
of truth for both validation and dispatch; the three loops iterate it filtered by the selection via
`in_pipeline_order`. Regression tests: `patterns::cascade_tests::test_selected_masking_is_order_independent`,
`test_selected_masking_matches_audit_for_same_selection`, `test_selected_fpe_is_order_independent`,
and `tests/test_masking.py::TestSelectionOrderIndependence`.

### 2. A blank pattern name silently disabled masking

`mask_pii('c', patterns=[''])` returned the column unmasked with no error. The Python guard only
rejected an empty list; the Rust side filtered empty entries out of the comma-separated string and
then validated an empty selection, which passed. This is the surviving edge of the v2.1.0 fix that
made unknown pattern names raise.

Fix: `validate_patterns` rejects an empty selection, and `_validate_patterns_arg` rejects blank or
non-string entries. Regression tests: `patterns::cascade_tests::test_empty_pattern_selection_is_rejected`
and `tests/test_masking.py::TestBlankPatternNames`.

### 3. No guard against pattern-registry drift

Added `tests/test_registry_sync.py`, which reads the engine's own pattern list, the `extract_pii`
struct fields and the `mask_pii_audit` count fields at runtime and asserts they agree with
`PII_FAMILIES`, `REGULATION`, `REVERSIBLE_FAMILIES` and the `mask_pii` docstring. All registries were
in sync at the time of the review; the test exists so that stops being a matter of luck.

---

## Logged, not fixed

### Structural

**The 36 pattern names are hand-copied across twelve registries.** `PATTERN_NAMES`, the three
selected-mask loops, `contains_any_selected`, `mask_all_audit_selected`, `extract_all`,
`extract_all_selected`, `ExtractResult`, `AuditCounts`, `extract_pii_output` plus its series vector,
`mask_pii_audit_output` plus its count-series vector, and `PII_FAMILIES` / `REGULATION` /
`REVERSIBLE_FAMILIES` in `_manifest.py`. Adding one pattern is roughly fourteen coordinated edits,
two of which are positionally coupled: the struct field order must match the series order or the
resulting struct is silently mislabeled. A macro or table driving the enum and all dispatch would
collapse this; the drift guard above is the cheap mitigation.

**`transform()` is copy-pasted verbatim** in `src/patterns/mea/israel.rs:42` and
`src/patterns/mea/south_africa.rs:59`, identical but for the regex constant (one written with
`match ... None => {}`, the other with `if let`). It exists because the shared `replace_counted`
cannot express position-aware validation, which both files need for `boundary_ok`. A
`replace_counted_at` taking `Fn(&Match) -> Option<String>` would let both copies go.

**Roughly 23 `mask_*_fpe` / `mask_*_consistent` pairs are byte-identical** apart from which closure
they build around `cipher.encrypt` or `hasher.encrypt`.

**`src/patterns/apac/korea_rrn.rs` has both `valid_rrn` and `rrn_valid`** — near-identical names with
different contracts (the first expects pre-stripped digits, the second strips them). `contains_rrn`
and `extract_rrn` re-inline the strip-and-validate logic instead of calling either.

**`src/lib.rs:213` (138 lines) and `src/lib.rs:406` (173 lines)** each declare 36 vectors, perform 36
pushes and 36 series constructions; `mask_pii_audit` additionally repeats 36 `push(0)` calls for the
null branch. `src/lib.rs:267` hand-builds an all-`None` `ExtractResult` field by field although the
struct derives `Default`.

**`src/lib.rs:16`** exports `pub use patterns::fpe::Ff3Cipher as MaskopsFpe` — unused, and
unreachable from outside a `cdylib`.

### Python package

- `maskops/__init__.py:5` and `:8` both `import polars as pl`; `TYPE_CHECKING` is imported for an
  empty `if TYPE_CHECKING: pass`; `load_policy` and the manifest helpers are imported at the bottom
  of the module (`:321`).
- Public signatures use `patterns: list = None`, `salt: str = None`, `columns: list = None`,
  `source: str = None` instead of `list[str] | None` / `str | None`. The `.pyi` stubs are correct, so
  mypy and pyright stay green, but the runtime signatures of a release whose headline was full
  typing support are wrong.
- `pl.col(expr) if isinstance(expr, str) else expr` followed by `args.append(pl.lit(",".join(patterns)))`
  is repeated in six functions.
- `maskops/_policy.py:119-126` is a dead version branch — both arms import `tomllib` — and leaves
  three trailing-whitespace lines where comments were stripped. `Policy.apply` re-binds `df` once per
  column instead of building one `with_columns`, and `Policy` never validates pattern names, so a
  typo in a policy file fails later inside Rust rather than at load time.
- `maskops/_manifest.py:174` swallows every exception to write `"maskops_version": "unknown"` into a
  document whose purpose is auditor evidence. `masking_manifest` also calls the plugin once per
  column in a Python loop.

### Mechanical (clippy, 194 warnings)

- 44 x `once_cell::sync::Lazy` to `std::sync::LazyLock`, which would drop the `once_cell` dependency
  from the pattern layer entirely.
- 41 x redundant closure, 16 x uninlined `format!` args, 6 x `is_multiple_of`, 4 x let-and-return
  (the `mask_non_digit` / `mask_digit` chains), 3 x `div_ceil`, 3 x needless `into_iter()`,
  18 x lossless `u8 as u32` that should be `u32::from`.
- Roughly 12 truncating casts in the crypto core — `src/patterns/fpe.rs:115,135,145,252` and
  `src/patterns/fpe_ff1.rs:128,147-156` (`usize as u32`, `u128 as u8`, `usize as u8`). Bounded by the
  FPE domain in practice, but unannotated and unchecked, and comments are not an option here, so they
  should become checked conversions with a real error path.
- `c as u32 - b'0' as u32` appears in several validators (`korea_rrn.rs:19`, `israel.rs:16` and
  others). It is only correct because the regex happens to guarantee digits; `to_digit(10)` would
  make that true by construction.

### Repository hygiene

- `dist/maskops-0.1.0-cp314-cp314-win_amd64.whl` and `dist/maskops-0.1.0.tar.gz` are tracked in git —
  stale 0.1.0 artifacts in a 2.1.0 repository. They should be removed from version control and the
  path ignored.
- `CLAUDE.md` states that Python must contain no docstrings. Every public function has a numpydoc
  docstring, and `tools/hooks/no_comments_guard.py` only inspects `.rs` files. The code and the hook
  are right; the written rule should be narrowed to Rust plus non-docstring Python comments.
- `tests/test_masking.py` is 2489 lines in one file.
