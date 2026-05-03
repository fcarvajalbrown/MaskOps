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

- Comments: 1-line only — no multi-line or block comments anywhere. Informal tone is fine if needed to fit in one line.
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
- Workspace pattern: `core` lib + binary crates.
- Always use `thiserror` for error types in libraries.
- `serde` + `serde_json` for serialization.
- Rayon for parallelism.

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
- [ ] Update GitHub Actions (`actions/checkout`, `actions/setup-python`) to versions supporting Node.js 24.
  Node.js 20 is deprecated; forced default becomes Node.js 24 on June 2nd 2026 and Node.js 20 is removed from runners on September 16th 2026.
  See: https://github.blog/changelog/2025-09-19-deprecation-of-node-20-on-github-actions-runners/
