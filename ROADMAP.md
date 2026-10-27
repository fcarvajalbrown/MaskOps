# Roadmap

MaskOps is the only open-source PII-masking engine that runs natively inside Polars, fully air-gapped, with check-digit-validated coverage of Latin American identifiers (RUT, CPF, CURP, and more) alongside EU, US, and APAC families. This roadmap takes it from that position to a production-grade compliance standard for regulated data teams.

**Market timing:** Chile's Ley 21.719 enters force on 1 December 2026 — the primary near-term trigger — while Open Finance (CMF's Sistema de Finanzas Abiertas, NCG 514 as amended by NCG 569 on 1 June 2026) now phases in from July 2027. Both push regulated personal data through fintech pipelines and turn in-pipeline masking into a near-term compliance requirement across Latin America.

> **Current version:** v2.1.0 — milestones v0.2–v2.1 shipped. Phase 1 (Foundation) and Phase 2 (Enterprise) complete; Phase 3 (Ecosystem) underway.
> **Phase 1 — Foundation:** coverage + API stability. **Phase 2 — Enterprise:** audit, FPE, and configurable policy. **Phase 3 — Ecosystem:** integrations, standardization, long-term API stability. Coverage and infrastructure releases alternate each minor.

---

## Phase 1 — Foundation `v0.2 → v1.0`: Done

Finished items in [docs/ROADMAP-HISTORY.md](docs/ROADMAP-HISTORY.md).

## Phase 2 — Enterprise `v1.1 → v2.0`: Done

Finished items in [docs/ROADMAP-HISTORY.md](docs/ROADMAP-HISTORY.md).

---

## Phase 3 — Ecosystem `v2.1 → v3.0`: In Progress

Goal: integrations, official listings, and a stable long-term API that position MaskOps as the default privacy layer in the Polars ecosystem. Finished items in [docs/ROADMAP-HISTORY.md](docs/ROADMAP-HISTORY.md).

- [ ] **v2.2** — Docs site as free-tool documentation: pricing, pilot and contact sections removed; benchmark numbers and sitemap refreshed
- [ ] **v2.3** — Official Polars plugins page PR merged (pola-rs/polars #27846 was closed unmerged)
- [ ] **v2.4** — Arrow IPC + DuckDB compatibility
- [ ] **v2.5** — Advanced CLI: batch mode, multiple output formats (CSV, Arrow IPC, JSON)
- [ ] **v2.6** — APAC depth: Singapore NRIC/FIN, Indian Aadhaar
- [ ] **v2.7** — US driver's license (state-by-state) · date of birth patterns
- [ ] **v2.8** — Performance: `rayon` multi-column parallelism · SIMD investigation
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
