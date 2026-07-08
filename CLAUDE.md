# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Rules

**Always work sequentially** — one tool call at a time, never parallel, even for independent steps.

**Skip brainstorming for already-specced work** — if the feature has a roadmap entry, a design doc in `docs/superpowers/specs/`, or is described in detail anywhere in the repo, do not invoke the brainstorming skill. Go straight to writing-plans or implementation.

**Never assume** — if any detail is unclear, ask before implementing.

**No emojis anywhere** — not in docs, commit messages, PR descriptions, code, or chat responses. Plain text only.

**Never force-push** without telling the user and waiting for confirmation.

**Creating `v*` tags is a production action** — it triggers the PyPI publish workflow. Treat it the same as a deployment: require explicit user approval before creating any tag that matches `v*`. "Proceed" or "go ahead" in context counts as approval.

**release-please owns version numbers** — do not hand-edit the version in `Cargo.toml`, `pyproject.toml`, or `.release-please-manifest.json` in ordinary feature/fix work. release-please derives the next version from conventional-commit messages and bumps those files itself. Hand-bumping versions in a commit collides with release-please's own bump on `main` and causes merge conflicts across all version files (manifest, Cargo.toml, pyproject.toml, CHANGELOG). Only edit versions by hand when the user explicitly asks for a specific version (e.g. "release as 1.5.1"), and bump all of those files together so they stay in sync.

**A release is a deliberate roadmap event — never a side effect of a commit.** There is no auto-release. `release-please.yml` runs only on manual `workflow_dispatch` (never on push), and `pages.yml`/`publish.yml` are scoped so ordinary commits publish nothing. Cut a new release **only** when a roadmap milestone has actually shipped (worked on + `ROADMAP.md` updated + merged via PR). Releasing then means: bump the version files, then create and push the `vX.Y.Z` tag (a production action — needs explicit user approval; the hooks in `.claude/settings.json` block tag creation, tag pushes, and `gh release create` otherwise), which triggers `publish.yml`. For everything else — `chore`, `docs`, `ci`, `style`, config, tooling, housekeeping — **do nothing release-related**: no tag, no GitHub Release, no version bump. When in doubt, do not release.

**`ROADMAP.md` is the single source of truth for the version — read it before any version or release action** (not the design specs under `docs/superpowers/`). Its "Current version" line and checked milestones define what the version *is* and what the next one will be; the repo's version strings must never run ahead of the current completed milestone. The version lives in **five** files that must always match and bump together: `Cargo.toml`, `pyproject.toml`, `.release-please-manifest.json`, the `maskops` entry in `Cargo.lock`, and the `__version__` string in `src/lib.rs` — this last one release-please cannot update (the no-comments rule blocks its update marker), so it must be bumped by hand. If they disagree, the lowest value that matches a shipped roadmap milestone is the real one.

**Finishing a task includes updating the changelog and roadmap — automatically, without being asked.** When work changes user-visible behavior or completes a roadmap milestone, update `docs/CHANGELOG.md` and check off the item in `ROADMAP.md` as the last step of the task, in the same change. Conversely, pure `chore`/`docs`/`ci`/tooling/config work touches neither. Never leave the changelog or roadmap stale waiting to be told.

**One commit per logical change** — no layer-split commits.

**Branch only for roadmap releases** — create a branch when the work ships a feature or fix that is listed on the roadmap. Tooling, config, and housekeeping commits go directly to `main`. For roadmap work:
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
pytest tests/ -v                   # 394 tests across all PII families
pytest tests/test_masking.py::TestMaskIBAN -v  # run a single class
```

`maturin develop` must be re-run after any Rust change before running tests.

## Key dependency constraints

`pyo3` and `pyo3-polars` versions must stay in sync — do not bump `pyo3` independently. Check `Cargo.toml` for current pinned versions before upgrading either crate.

## Architecture

```
src/lib.rs                    # Polars expression registration (mask_pii, contains_pii, mask_pii_fpe, mask_pii_fpe_rekey, mask_pii_consistent, extract_pii, mask_pii_audit)
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
5. **New patterns must be named and scoped to their compliance category** — which regulation, FPE or asterisk-only, and what validation prevents false positives. No comments; names and types carry this.

## CI notes

- **Ubuntu + Python 3.12 is excluded** from the test matrix — the compiled extension fails to load (`dlopen` error). Same tests pass on Windows and Ubuntu 3.10/3.11.
- Coverage uploads from the Ubuntu 3.11 job.
- GitHub Pages deploys via `.github/workflows/pages.yml` (Pages `build_type: workflow`) only when site files change (`index.html`, `sitemap.xml`, `assets/**`) — not on every push.
- GitHub Actions node version: Node.js 20 is deprecated as of June 2026; actions need updating to support Node.js 24 before September 16, 2026.
- **`typecheck` job (gating):** runs mypy (strict) and pyright (standard) over the `.pyi` stubs and the `tests/typing/` usage snippet — no Rust build needed. Config lives in `[tool.mypy]` / `[tool.pyright]` in `pyproject.toml`. Public-API type changes must keep both green; run `python -m mypy` and `python -m pyright` locally before pushing.

## Commits

`<type>(<scope>): <description>` — lowercase, present-tense imperative. Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`, `ci`.

No `Co-Authored-By` trailers. No "Generated with Claude Code" or any AI attribution in commit messages, PR descriptions, or code comments.

## Publishing

Releases are manual and deliberate — there is no auto-release (see the release rules above). `release-please.yml` runs only on manual `workflow_dispatch`.

Cut a release with the helper: **`python tools/release/release.py X.Y.Z`** is a dry run that validates all five version files, the changelog section, and the roadmap agree on the version; **`python tools/release/release.py X.Y.Z --yes`** then tags, pushes, and creates the GitHub Release. Pushing the tag triggers `publish.yml` (`.github/workflows/publish.yml`), which builds wheels and uploads to PyPI. **Do not modify `publish.yml` — it works; the release flow only pushes a tag it already reacts to.**

**Claude may run the script with `--yes`** to cut the release itself once the dry-run is green and the milestone has actually shipped (worked on + `ROADMAP.md` updated + merged via PR). Run the dry-run first (`python tools/release/release.py X.Y.Z`) to confirm all five version files, the changelog section, and the roadmap agree; only then run `python tools/release/release.py X.Y.Z --yes`, which tags, pushes, and creates the GitHub Release. This is still a deliberate roadmap event — never a side effect of a routine commit. Note: PyPI has lagged behind GitHub tags before (cancelled/failed publish runs), so verify the live PyPI version separately rather than trusting the tag list.

## Release marketing reminders

**dev.to** — script at `tools/social/devto_post.py`, run it here in Claude Code (not via CI). Remind at milestones only — not every release:
- Tutorial, deep-dive, or benchmark comparison angle
- Major version releases (v3.0+)

Rules baked into the script: enforces 5-day cooldown, max 4 tags, Camus+Dijkstra prose. Run `--rules` to print full guidelines. A quiet-nudge reminder fires automatically via the Stop hook in settings.json; the interval and weekday gating are documented in `tools/social/CLAUDE.md`.

**LinkedIn** — at major milestones (v3.0+), remind the user to publish a full LinkedIn article (not just a post): SEO keywords, publish timing, hashtags.
