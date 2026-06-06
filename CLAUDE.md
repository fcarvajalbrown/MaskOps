# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Rules

**Always work sequentially** — one tool call at a time, never parallel, even for independent steps.

**Skip brainstorming for already-specced work** — if the feature has a roadmap entry, a design doc in `docs/superpowers/specs/`, or is described in detail anywhere in the repo, do not invoke the brainstorming skill. Go straight to writing-plans or implementation.

**Never assume** — if any detail is unclear, ask before implementing.

**Never force-push** without telling the user and waiting for confirmation.

**Creating `v*` tags is a production action** — it triggers the PyPI publish workflow. Treat it the same as a deployment: require explicit user approval before creating any tag that matches `v*`. "Proceed" or "go ahead" in context counts as approval.

**One commit per logical change** — no layer-split commits.

**Every task gets its own branch** — never implement features or fixes directly on `main`. Before starting any implementation task:
1. Create a branch: `git checkout -b <type>/<short-description>` (e.g. `feat/extract-pii`, `fix/ssn-validation`)
2. Do the work and commit(s) on that branch
3. Open a PR with a STAR-format description (see below)
4. After merge: delete the branch and `git checkout main && git pull`

**PR description format (STAR):**
```
## Situation
<What was the context — what problem or gap existed?>

## Task
<What specifically needed to be done?>

## Action
<What was implemented, and key decisions made?>

## Result
<What changed for users — behavior, performance, API surface?>
```

**"Add to AGENTS.md"** means write to that file locally and stop — do not commit or push unless explicitly asked.

## Roadmap

When a feature ships, check it off in `README.md` under `## Roadmap`. If it was not already listed, add it as a checked item. Only list user-visible capabilities — skip internal refactors, CI changes, and dependency bumps.

## Changelog

Update `docs/CHANGELOG.md` with every commit that adds, removes, or changes public API behavior (new patterns, new expressions, breaking changes, significant bug fixes). One entry per change, one line max. Skip internal refactors, test changes, CI tweaks, and dependency bumps unless they affect behavior. Target 3–10 entries per release — never dump raw commit messages.

## Code style

- **NO comments of any kind** — no `//`, `//!`, `///`, `/* */` in Rust; no `#` comments or docstrings in Python. Zero. Names and types are the only documentation.
- Bug fixes: root cause only — never patch test parameters or add workarounds to make tests pass.
- Never write code just to make it compile; code must reflect real behavior.

## Rust conventions

- `thiserror` for error types in libraries.
- `serde` + `serde_json` for serialization.
- `rayon` for parallelism.

## What this is

MaskOps is a native Polars plugin for high-speed PII masking. The Rust core compiles to a `cdylib` (`.so`/`.pyd`) that Polars loads as an expression plugin — no Python overhead per row. The Python package (`maskops/`) exposes three expressions: `mask_pii`, `contains_pii`, and `mask_pii_fpe`.

## Build & develop

Always work inside a `.venv` at the project root. If it doesn't exist, create it before running any Python or maturin command — regardless of which machine you're on:

```bash
python3 -m venv .venv
source .venv/bin/activate          # macOS/Linux
pip install maturin faker polars pytest
maturin develop --release          # compiles Rust + installs editable Python package
```

On Windows (PowerShell), run each command separately — no `&&`. Use `.venv\Scripts\activate` instead of `source`.

Never assume a `.venv` already exists. Always check with `ls .venv` or just re-run `python3 -m venv .venv` (safe to run on an existing venv — it no-ops).

## Tests

```bash
python tests/generate_fixtures.py  # must run first; creates fixture CSVs (gitignored)
pytest tests/ -v                   # 97 tests across all PII families
pytest tests/test_masking.py::TestMaskIBAN -v  # run a single class
```

`maturin develop` must be re-run after any Rust change before running tests.

## Key dependency constraints

`pyo3` and `pyo3-polars` versions must stay in sync — do not bump `pyo3` independently. Check `Cargo.toml` for current pinned versions before upgrading either crate.

## Architecture

```
src/lib.rs                    # Polars expression registration (mask_pii, contains_pii, mask_pii_fpe)
src/patterns/mod.rs           # Aggregators: mask_all(), mask_all_fpe(), contains_any_pii()
src/patterns/<family>.rs      # One file per PII type
maskops/__init__.py           # Python API — wraps register_plugin_function
```

The pattern pipeline in `mod.rs` is: non-digit PII first (`mask_non_digit`), then digit PII (`mask_digit` or `mask_digit_fpe`). **Adding a new pattern = new file + import in `mod.rs` + call in the appropriate aggregator(s).**

### FPE vs asterisk masking

Digit-based PII (credit cards, phones, RUT, CPF) supports two modes:
- **Asterisk** (`mask_all`): irreversible anonymization — replaces digits with `*`. No recovery possible.
- **FPE** (`mask_all_fpe`): FF3-1 AES-256 format-preserving encryption — same length/format, reversible with the same key+tweak.

Non-digit PII (IBAN, VAT, email, IP, EU IDs, CURP) is always asterisked regardless of mode.

`mask_pii_fpe` requires a 32-byte key and 7-byte tweak passed as Polars `Binary` literals.

### GDPR / data protection compliance model

See [`docs/gdpr/gdpr-reference.md`](docs/gdpr/gdpr-reference.md) for the full reference.

Hard rules — never break these in any code change:

1. **FPE = pseudonymization, not anonymization** (GDPR Art. 4(5)) — never claim FPE output is anonymous.
2. **Key separation is mandatory** — the FPE key must never be stored alongside masked data. Client owns the key. MaskOps never sees it.
3. **Asterisk masking is irreversible** — do not add any recovery mechanism.
4. **No network calls, ever** — MaskOps must remain 100% air-gappable.
5. **New patterns must declare compliance category** in their module docstring: which regulation, FPE or asterisk-only, and what validation prevents false positives.

## CI notes

- **Ubuntu + Python 3.12 is excluded** from the test matrix — the compiled extension fails to load (`dlopen` error). Same tests pass on Windows and Ubuntu 3.10/3.11.
- Coverage uploads from the Ubuntu 3.11 job.
- GitHub Actions node version: Node.js 20 is deprecated as of June 2026; actions need updating to support Node.js 24 before September 16, 2026.

## Commits

`<type>(<scope>): <description>` — lowercase, present-tense imperative. Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`, `ci`.

No `Co-Authored-By` trailers. No "Generated with Claude Code" or any AI attribution in commit messages, PR descriptions, or code comments.

## Publishing

PyPI publish is triggered by pushing a version tag (`v*`). The workflow is in `.github/workflows/publish.yml`.
