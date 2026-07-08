# Roadmap

MaskOps is the only open-source PII-masking engine that runs natively inside Polars, fully air-gapped, with check-digit-validated coverage of Latin American identifiers (RUT, CPF, CURP, and more) alongside EU, US, and APAC families. This roadmap takes it from that position to a production-grade compliance standard for regulated data teams.

**Market timing:** Chile's Ley 21.719 enters force on 1 December 2026 — the primary near-term trigger — while Open Finance (CMF's Sistema de Finanzas Abiertas, NCG 514 as amended by NCG 569 on 1 June 2026) now phases in from July 2027. Both push regulated personal data through fintech pipelines and turn in-pipeline masking into a near-term compliance requirement across Latin America.

> **Current version:** v2.0.0 — milestones v0.2–v2.0 shipped. Phase 1 (Foundation) and Phase 2 (Enterprise) complete.
> **Phase 1 — Foundation:** coverage + API stability. **Phase 2 — Enterprise:** audit, FPE, and configurable policy. **Phase 3 — Ecosystem:** integrations, standardization, long-term API stability. Coverage and infrastructure releases alternate each minor.

---

## Phase 1 — Foundation `v0.2 → v1.0`

Goal: broad, check-digit-validated coverage and a stable public API — the credibility base that makes the engine production-adoptable.

- [x] **v0.2** — Module reorganization (`eu/`, `latam/`, `us/`, `healthcare/`, `contact/`, `financial/`) · NIN + Personalausweis check digit validation
- [x] **v0.3** — US identifiers: SSN, US passport number
- [x] **v0.4** — Parquet streaming (lazy scan pipeline support) · benchmark refresh
- [x] **v0.5** — LatAm depth: Argentine DNI · Colombian CC/NIT · IPv4 range fix
- [x] **v0.6** — Configurable pattern selection: `mask_pii("col", patterns=["email", "ssn"])` · Ecuadorian cédula
- [x] **v0.7** — Healthcare: US NPI, Medicare Beneficiary ID, NHS number (UK) · Peruvian DNI
- [x] **v0.8** — Consistent masking: deterministic hash-based pseudonymization (same input → same output, no FPE key required)
- [x] **v0.9** — EU depth: French NIR (INSEE), Italian codice fiscale · Uruguayan cédula · APAC start: Canadian SIN, Australian TFN
- [x] **v1.0** — API stability guarantee · CLI tool (`maskops run`) · GitHub Pages docs site · PR to official Polars plugins page

---

## Phase 2 — Enterprise `v1.1 → v2.0`

Goal: the enterprise surface — audit, reversible FPE, and configurable policy — that turns adoption into measurable compliance value for regulated teams.

- [x] **v1.1** — Policy files: YAML/TOML config for per-column masking rules (pairs with v1.0 CLI)
- [x] **v1.2** — EU depth: Polish PESEL, Dutch BSN, Swedish personnummer
- [x] **v1.3** — `extract_pii` expression: struct column with one field per PII family
- [x] **v1.4** — Multi-column referential integrity: consistent masking across joined tables
- [x] **v1.5** — APAC: Japanese My Number, South Korean RRN
- [x] **v1.6** — `mask_pii_audit` expression: masked value + per-family match counts in one pass
- [x] **v1.7** — LATAM depth & Chile readiness: Brazilian CNPJ (legal-entity confidentiality masking) · masking manifest / RAT export (per-column PII family, regulation, mask mode, and match counts — feeds Ley 21.719's data-processing register and auditor evidence)
- [x] **v1.8** — FPE crypto & key management: key rotation, tweak derivation, validation · FF1 mode (NIST-surviving FPE alternative to FF3-1)
- [x] **v1.9** — MEA: South African ID, Israeli ID number
- [x] **v2.0** — Enterprise release: configurable patterns + structured output + audit unified · migration guide

---

## Phase 3 — Ecosystem `v2.1 → v3.0`

Goal: integrations, official listings, and a stable long-term API that position MaskOps as the default privacy layer in the Polars ecosystem.

- [x] **v2.1** — Python typing stubs (`.pyi`) · full mypy/pyright support
- [ ] **v2.2** — Docs site re-check & refresh: audit the live GitHub Pages site against shipped v2.0 capabilities (audit expression, FF1/FF3-1 FPE, RAT/manifest export, MEA + APAC coverage), refresh benchmark numbers and copy, SEO/sitemap pass, fix stale links · pricing research (market rates for PII-masking / compliance tooling in Chile + Brazil) · go-to-market research (most convincing ways to get teams to adopt or contract the product)
- [ ] **v2.3** — APAC depth: Singapore NRIC/FIN, Indian Aadhaar
- [ ] **v2.4** — Arrow IPC + DuckDB compatibility
- [ ] **v2.5** — Performance: `rayon` multi-column parallelism · SIMD investigation
- [ ] **v2.6** — Listed on `ddotta/awesome-polars` · official Polars plugins page PR merged
- [ ] **v2.7** — US driver's license (state-by-state) · date of birth patterns
- [ ] **v2.8** — Advanced CLI: batch mode, multiple output formats (CSV, Arrow IPC, JSON)
- [ ] **v2.9** — API review · deprecations · 3.0 migration guide
- [ ] **v3.0** — Breaking changes from 2.9 review applied · stable typing · ecosystem complete

---

## Pattern coverage targets

| Family | v1.0 | v2.0 | v3.0 |
|--------|:----:|:----:|:----:|
| EU (IBAN, VAT, DNI/NIE, NIN, Personalausweis, NIR, CF, PESEL, BSN, personnummer) | ✓ | ✓ | ✓ |
| LatAm (RUT, CPF, CURP, ARG DNI, CO CC/NIT, EC cédula, PE DNI, UY cédula) | ✓ | ✓ | ✓ |
| US (SSN, passport, driver's license, NPI, Medicare) | ✓ | ✓ | ✓ |
| Healthcare (NPI, MBI, NHS) | ✓ | ✓ | ✓ |
| APAC (SIN, TFN, My Number, RRN, NRIC, Aadhaar) | partial | ✓ | ✓ |
| MEA (ZA ID, IL ID) | — | ✓ | ✓ |
| Contact (email, phone, IP) | ✓ | ✓ | ✓ |
| Financial (credit cards, IBAN, VAT) | ✓ | ✓ | ✓ |

---

## Phase 4 — Horizon `v4.0`

Goal: the growth headroom beyond the Polars core — widen the addressable market once MaskOps is the established standard.

- [ ] **Optional NER layer (free-text names/locations)** — opt-in detection of PERSON / LOCATION / ORG in free text, as a separate layer *outside* the deterministic fast core: gazetteer-first to stay air-gapped and no-ML, with an optional ML module where higher recall is required. Closes the one capability gap versus ML tools like Presidio without sacrificing MaskOps' speed, determinism, or air-gap. Built when a client's data needs it, never bundled into the core masking path.
- [ ] **pandas-compatible API** — expose the same Rust engine to pandas / PyArrow workflows, widening the addressable market from Polars-native teams to the far larger pandas ecosystem, with no second implementation of the core.
- [ ] **GUI (optional)** — desktop interface wrapping the CLI + policy files for non-technical compliance users (municipalities, compliance officers): configure masking rules and run batch jobs without writing code.

---

Want to request a pattern or report a false positive? Open an issue.
