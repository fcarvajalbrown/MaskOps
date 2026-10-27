# Roadmap history

Finished items moved from `ROADMAP.md` word for word, newest first.

## Phase 3 — Ecosystem `v2.1 → v3.0`

- [x] Listed on `ddotta/awesome-polars` (ddotta/awesome-polars #620 merged; split from the former v2.6 milestone)
- [x] **v2.1** — Python typing stubs (`.pyi`) · full mypy/pyright support

## Phase 2 — Enterprise `v1.1 → v2.0`

Goal: the enterprise surface — audit, reversible FPE, and configurable policy — that turns adoption into measurable compliance value for regulated teams.

- [x] **v2.0** — Enterprise release: configurable patterns + structured output + audit unified · migration guide
- [x] **v1.9** — MEA: South African ID, Israeli ID number
- [x] **v1.8** — FPE crypto & key management: key rotation, tweak derivation, validation · FF1 mode (NIST-surviving FPE alternative to FF3-1)
- [x] **v1.7** — LATAM depth & Chile readiness: Brazilian CNPJ (legal-entity confidentiality masking) · masking manifest / RAT export (per-column PII family, regulation, mask mode, and match counts — feeds Ley 21.719's data-processing register and auditor evidence)
- [x] **v1.6** — `mask_pii_audit` expression: masked value + per-family match counts in one pass
- [x] **v1.5** — APAC: Japanese My Number, South Korean RRN
- [x] **v1.4** — Multi-column referential integrity: consistent masking across joined tables
- [x] **v1.3** — `extract_pii` expression: struct column with one field per PII family
- [x] **v1.2** — EU depth: Polish PESEL, Dutch BSN, Swedish personnummer
- [x] **v1.1** — Policy files: YAML/TOML config for per-column masking rules (pairs with v1.0 CLI)

## Phase 1 — Foundation `v0.2 → v1.0`

Goal: broad, check-digit-validated coverage and a stable public API — the credibility base that makes the engine production-adoptable.

- [x] **v1.0** — API stability guarantee · CLI tool (`maskops run`) · GitHub Pages docs site · PR to official Polars plugins page
- [x] **v0.9** — EU depth: French NIR (INSEE), Italian codice fiscale · Uruguayan cédula · APAC start: Canadian SIN, Australian TFN
- [x] **v0.8** — Consistent masking: deterministic hash-based pseudonymization (same input → same output, no FPE key required)
- [x] **v0.7** — Healthcare: US NPI, Medicare Beneficiary ID, NHS number (UK) · Peruvian DNI
- [x] **v0.6** — Configurable pattern selection: `mask_pii("col", patterns=["email", "ssn"])` · Ecuadorian cédula
- [x] **v0.5** — LatAm depth: Argentine DNI · Colombian CC/NIT · IPv4 range fix
- [x] **v0.4** — Parquet streaming (lazy scan pipeline support) · benchmark refresh
- [x] **v0.3** — US identifiers: SSN, US passport number
- [x] **v0.2** — Module reorganization (`eu/`, `latam/`, `us/`, `healthcare/`, `contact/`, `financial/`) · NIN + Personalausweis check digit validation
