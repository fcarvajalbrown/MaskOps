# Agent Notes — MaskOps

## Identity & Affiliation

- **Name:** Felipe Carvajal Brown
- **Company:** Felipe Carvajal Brown Software (formerly Instituto Igualdad — never use the old name)
- **Academic:** Magíster en Simulaciones Numéricas, Universidad Politécnica de Madrid (UPM)
- **ORCID:** 0000-0002-8300-7587
- **Email:** fcarvajalbrown@gmail.com
- **Location:** Santiago, Chile

### Affiliation rules by context
| Context | Affiliation to use |
|---------|-------------------|
| Project footers, Cargo.toml authors, branding | Felipe Carvajal Brown Software |
| Security reports (PDF), responsible disclosure | Magíster en Simulaciones Numéricas, UPM |
| Academic papers | UPM + ORCID 0000-0002-8300-7587 |
| Never use | Instituto Igualdad, UC Chile, or any other institution |

---

## File Delivery Rules

- Present files one at a time — wait for feedback before the next file.
- Fixes and improvements: diffs/snippets only — never full files unless explicitly asked.
- Never volunteer a full file when a targeted change is sufficient.

---

## Code Style (All Languages)

- **No comments of any kind** — no `//`, `//!`, `///`, `/* */` in Rust; no `#` or docstrings in Python. Zero.
- Bug fixes: always at the root cause — never patch test parameters or create workarounds to produce passing results.
- Never write code just to make it compile — code must reflect real behavior.
- Tools that ship to real institutions: correctness is non-negotiable.

---

## Language & Environment

- **IDE:** VS Code
- **Terminal:** PowerShell (Windows) — never use `&&` separator, always separate commands.
- **Shell for Linux tools:** WSL (Kali)
- **Primary languages:** Rust, Python, TypeScript

### Python-specific
- Always create and activate a venv before installing any dependencies — multiple conflicting Python installs on Windows via PyManager (system + user). Never skip this step.
- Use `pip install <pkg> --break-system-packages` in the Claude container only.
- PowerShell: `Remove-Item`, `Invoke-WebRequest` (not rm/curl).

### Rust-specific (this project)
- Single `cdylib` crate — a Polars expression plugin. No workspace, no binary crates.
- `src/lib.rs` registers the expressions (`mask_pii`, `contains_pii`, `mask_pii_fpe`); one file per PII family under `src/patterns/<region>/<family>.rs`, wired up in the `mod.rs` aggregators.
- Re-run `maturin develop --release` after any Rust change before running tests.
- Actual stack: detection via `regex` + `once_cell`; FPE crypto via `aes` + `cipher` (FF3-1/FF1) with `sha2` + `hmac`; Python bridge via `pyo3` + `pyo3-polars` + `polars-core`. No `thiserror`/`serde`/`rayon` in use today.

---

## Agent Interaction Rules

- "Add to AGENTS.md" means write to the file locally and STOP. Do NOT commit or push unless explicitly asked.
- This is a scratchpad for deferred work, not an action item to execute immediately.

---

## Response Style

- Brief and factually correct — no over-explaining simple things.
- No bullet points for conversational answers — prose only.
- No emojis unless Felipe uses them first.
- When asked for a recommendation, give one — don't hedge with 5 options.
- If something needs research before answering, search the web first — don't guess.

---

## CI / Coverage

- **Ubuntu + Python 3.12 is excluded from CI** because the compiled Rust extension fails to load (`dlopen` error) on `ubuntu-latest` with Python 3.12. The same tests pass on Windows and on Ubuntu 3.10/3.11.
- Coverage is uploaded from the **Ubuntu 3.11** job. See `.github/workflows/ci.yml`.

---

## Project TODOs

- [ ] Fix Rust extension `dlopen` failure on `ubuntu-latest` + Python 3.12.
  Suspected causes: `maturin develop` editable-install path mismatch, or missing `.so` in the source tree when Polars tries to load the plugin.
- [ ] Add Rust coverage (e.g. `cargo tarpaulin`) and merge with Python report on Codecov for an accurate combined number.
- [x] Bump `actions/checkout` (v5) and `actions/setup-python` (v6) to their Node.js 24 majors in `ci.yml`, `pages.yml`, `benchmark_presidio.yml` (done 2026-06-11).
  `publish.yml` stays on checkout@v4 / setup-python@v5 per its do-not-modify rule, but is covered by `FORCE_JAVASCRIPT_ACTIONS_TO_NODE24: 'true'` (also set in `ci.yml`). Node.js 20 is removed from runners on September 16th 2026; before then, drop the FORCE env once every action runs on a Node 24 major.
  See: https://github.blog/changelog/2025-09-19-deprecation-of-node-20-on-github-actions-runners/
