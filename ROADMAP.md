# Roadmap

MaskOps follows a three-phase path from its initial public release to a production-grade standard for Polars PII masking.

> **Current version:** v0.1.5
> Coverage and infrastructure releases alternate each minor. Enterprise features land individually in 1.x and unify at 2.0.

---

## Phase 1 — Foundation `v0.2 → v1.0`

Goal: API stability + complete coverage across all major global ID families.

- [ ] **v0.2** — Module reorganization (`eu/`, `latam/`, `us/`, `healthcare/`, `contact/`, `financial/`) · NIN + Personalausweis check digit validation
- [ ] **v0.3** — US identifiers: SSN, US passport number
- [ ] **v0.4** — Parquet streaming (lazy scan pipeline support) · benchmark refresh
- [ ] **v0.5** — EU depth: French NIR (INSEE), Italian codice fiscale
- [ ] **v0.6** — Configurable pattern selection: `mask_pii("col", patterns=["email", "ssn"])`
- [ ] **v0.7** — Healthcare: US NPI, Medicare Beneficiary ID, NHS number (UK)
- [ ] **v0.8** — Consistent masking: deterministic hash-based pseudonymization (same input → same output, no FPE key required)
- [ ] **v0.9** — LatAm depth: Argentine DNI · APAC start: Canadian SIN, Australian TFN
- [ ] **v1.0** — API stability guarantee · GitHub Pages docs site · PR to official Polars plugins page

---

## Phase 2 — Enterprise `v1.1 → v2.0`

Goal: configurable patterns, structured output, and audit land individually then unify.

- [ ] **v1.1** — EU depth: Polish PESEL, Dutch BSN, Swedish personnummer
- [ ] **v1.2** — `extract_pii` expression: struct column with one field per PII family
- [ ] **v1.3** — LatAm depth: Colombian CC/NIT, Peruvian DNI
- [ ] **v1.4** — Multi-column referential integrity: consistent masking across joined tables
- [ ] **v1.5** — APAC: Japanese My Number, South Korean RRN
- [ ] **v1.6** — `mask_pii_audit` expression: masked value + per-family match counts in one pass
- [ ] **v1.7** — MEA: South African ID, Israeli ID number
- [ ] **v1.8** — Policy files: YAML/TOML config for per-column masking rules
- [ ] **v1.9** — US driver's license (state-by-state) · date of birth patterns
- [ ] **v2.0** — Enterprise release: configurable patterns + structured output + audit unified · migration guide

---

## Phase 3 — Ecosystem `v2.1 → v3.0`

Goal: integrations, hosted docs, official listings, clean API for long-term stability.

- [ ] **v2.1** — FPE key management helpers: rotation, tweak derivation, validation
- [ ] **v2.2** — GitHub Pages documentation site live
- [ ] **v2.3** — APAC depth: Singapore NRIC/FIN, Indian Aadhaar
- [ ] **v2.4** — Python typing stubs (`.pyi`) · full mypy/pyright support
- [ ] **v2.5** — CLI: `maskops run config.toml input.parquet output.parquet`
- [ ] **v2.6** — Listed on `ddotta/awesome-polars` · official Polars plugins page PR merged
- [ ] **v2.7** — Performance: `rayon` multi-column parallelism · SIMD investigation
- [ ] **v2.8** — Arrow IPC + DuckDB compatibility
- [ ] **v2.9** — API review · deprecations · 3.0 migration guide
- [ ] **v3.0** — Breaking changes from 2.9 review applied · stable typing · ecosystem complete

---

## Pattern coverage targets

| Family | v1.0 | v2.0 | v3.0 |
|--------|:----:|:----:|:----:|
| EU (IBAN, VAT, DNI/NIE, NIN, Personalausweis, NIR, CF, PESEL, BSN, personnummer) | ✓ | ✓ | ✓ |
| LatAm (RUT, CPF, CURP, ARG DNI, CO CC/NIT, PE DNI) | ✓ | ✓ | ✓ |
| US (SSN, passport, driver's license, NPI, Medicare) | ✓ | ✓ | ✓ |
| Healthcare (NPI, MBI, NHS) | ✓ | ✓ | ✓ |
| APAC (SIN, TFN, My Number, RRN, NRIC, Aadhaar) | partial | ✓ | ✓ |
| MEA (ZA ID, IL ID) | — | ✓ | ✓ |
| Contact (email, phone, IP) | ✓ | ✓ | ✓ |
| Financial (credit cards, IBAN, VAT) | ✓ | ✓ | ✓ |

---

Want to request a pattern or report a false positive? Open an issue.
