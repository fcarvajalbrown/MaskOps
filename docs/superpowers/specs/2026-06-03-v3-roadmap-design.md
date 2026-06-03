# MaskOps v3 Roadmap Design

**Date:** 2026-06-03
**Current version:** 0.1.5
**Target:** v3.0.0 in 0.1 increments

---

## Conceptual milestones

| Major | Theme |
|-------|-------|
| v1.0 | API stability seal + complete PII coverage across major global ID families |
| v2.0 | Enterprise release: configurable patterns, structured output, audit |
| v3.0 | Ecosystem maturity: integrations, hosted docs, official Polars listing, clean API |

---

## Overall approach

Coverage and infrastructure milestones alternate every 0.1 minor. Enterprise building blocks land individually in infra slots throughout 1.x, then unify into the full enterprise API at 2.0. This keeps the codebase healthy as pattern count grows and delivers enterprise value incrementally.

---

## Phase 1 — Foundation (v0.2.0 → v1.0.0)

### v0.2.0 — Infra: Module reorganization + check digit validation

**src/patterns/ restructured into named subfolders:**

```
src/patterns/
  eu/          ← iban, vat, european_id
  latam/       ← latam_id
  us/          ← (placeholder; future: ssn, passport, driver_license)
  healthcare/  ← (placeholder; future: npi, medicare, nhs)
  contact/     ← email, phone, ip
  financial/   ← credit_card
  fpe.rs       ← stays at patterns root (used cross-module)
  country_codes.rs
```

No API change. Zero breaking changes.

**Check digit validation completed:**
- NIN (UK) — format-only → full check digit validation
- Personalausweis (DE) — format-only → full check digit validation

---

### v0.3.0 — Coverage: US identifiers

- SSN (US) — 9-digit, area/group/serial field validation, known invalid ranges excluded
- US passport number — 9-character alphanumeric format

---

### v0.4.0 — Infra: Parquet streaming + benchmark refresh

- Lazy scan pipeline compatibility (Polars `scan_parquet` → `sink_parquet` chain)
- Benchmark suite updated to cover new patterns added since 0.1.5

---

### v0.5.0 — Coverage: LatAm depth (LATAM commercial priority)

- Argentine DNI — 8 digits (reform bill in Congress makes this urgent)
- Colombian cédula de ciudadanía (CC) — 6–10 digits
- Colombian NIT (Número de Identificación Tributaria) — 9 digits + check digit

---

### v0.6.0 — Infra: Configurable pattern selection + Ecuadorian cédula

First cut of per-call pattern selection:

```python
maskops.mask_pii("col", patterns=["email", "ssn", "credit_card"])
```

- Patterns default to all enabled (backward-compatible)
- Pattern names match the module folder structure
- `contains_pii` gains the same `patterns=` argument

**Also:** Ecuadorian cédula de identidad — 10-digit Módulo 10 validated (SPDP first enforcement actions in 2024).

---

### v0.7.0 — Coverage: Healthcare identifiers + Peruvian DNI

- US NPI (National Provider Identifier) — 10-digit Luhn-validated
- Medicare Beneficiary Identifier (MBI) — 11-character alphanumeric
- NHS number (UK) — 10-digit Modulus 11 validated
- Peruvian DNI — 8 digits

---

### v0.8.0 — Infra: Consistent masking (enterprise building block 2)

Deterministic hash-based pseudonymization — same input always produces the same masked output without requiring an FPE key. Useful for referential integrity in test data generation and across joined tables.

```python
maskops.mask_pii("col", mode="consistent", salt="my-secret-salt")
```

- Backed by HMAC-SHA256 truncated to match output format
- Not reversible (unlike FPE) — one-way pseudonymization
- Only applies to digit-based PII; non-digit PII is always asterisked

---

### v0.9.0 — Coverage: EU depth + Uruguayan cédula + APAC start

- French NIR (INSEE social security number)
- Italian codice fiscale
- Uruguayan cédula de identidad — 7–8 digits (EU adequacy bridge jurisdiction)
- Canadian SIN (Social Insurance Number)
- Australian TFN (Tax File Number)

---

### v1.0.0 — Milestone: API stability + CLI + documentation + ecosystem

- API stability guarantee: no breaking changes without a major version bump from this point
- **CLI tool:** `maskops run config.toml input.parquet output.parquet` — municipalities and non-Python users need this
- GitHub Pages documentation site live
- PR submitted to official Polars plugins page (`docs.pola.rs/user-guide/plugins/`)
- 20 GitHub topics applied to the repo (per discoverability research)

---

## Phase 2 — Enterprise Building Blocks (v1.1.0 → v2.0.0)

### v1.1.0 — Infra: Policy files (pairs with v1.0 CLI)

YAML/TOML config declaring masking rules per column, loaded once at pipeline start:

```yaml
columns:
  notes:
    patterns: [email, phone, credit_card]
    mode: asterisk
  customer_ref:
    patterns: [ssn, rut]
    mode: consistent
    salt: ${MASK_SALT}
```

Moved from v1.8.0 — the CLI at v1.0 is only useful once policy files exist.

### v1.2.0 — Coverage: EU depth

- Polish PESEL
- Dutch BSN (Burgerservicenummer)
- Swedish personnummer

### v1.3.0 — Infra: `extract_pii` expression (enterprise building block 3)

Returns a struct column with one boolean/string field per PII family:

```python
df.with_columns(maskops.extract_pii("notes"))
# → struct { email: str|null, ssn: str|null, credit_card: str|null, ... }
```

Enables downstream routing, reporting, and selective masking.

### v1.4.0 — Infra: Multi-column referential integrity

Consistent masking across related columns — the same logical value masked identically whether it appears in `customer_id`, `reference_id`, or a free-text note field. Requires a shared salt and the `consistent` mode from v0.8.0.

### v1.5.0 — Coverage: APAC

- Japanese My Number (個人番号) — 12-digit with check digit
- South Korean RRN (주민등록번호) — 13-digit

### v1.6.0 — Infra: Audit expression (enterprise building block 4)

```python
maskops.mask_pii_audit("col")
# → struct { masked: str, audit: { email: int, ssn: int, credit_card: int, ... } }
```

Returns the masked value alongside a match-count struct per PII family. Enables compliance reporting without a separate scan pass.

### v1.7.0 — Coverage: MEA

- South African ID number — 13-digit with check digit
- Israeli ID number (Mispar Zehut) — 9-digit Luhn-validated

### v1.8.0 — Infra: FPE key management helpers

- Key rotation utilities
- Tweak derivation helpers
- Key validation (length, entropy check)
- Moved from v2.1.0 — critical for air-gapped enterprise deployments where key management is manual

### v1.9.0 — Coverage: US depth + dates

- US driver's license patterns (state-by-state format matching)
- Date of birth patterns (structured date fields, configurable locale)

### v2.0.0 — Milestone: Enterprise release

Consolidation release — no new coverage. Unifies the building blocks from 0.6.0, 0.8.0, 1.3.0, 1.4.0, 1.6.0, 1.1.0 into a cohesive, documented enterprise API. Includes a migration guide for users upgrading from pre-1.0.

---

## Phase 3 — Ecosystem Maturity (v2.1.0 → v3.0.0)

### v2.1.0 — Infra: Python typing stubs

`.pyi` stubs for all public expressions — full mypy and pyright support.

### v2.2.0 — Ecosystem: Documentation site

GitHub Pages docs site with full API reference, pattern coverage matrix, and usage guides.

### v2.3.0 — Coverage: More APAC

- Singapore NRIC/FIN
- Indian Aadhaar — 12-digit Verhoeff-validated

### v2.4.0 — Infra: Python typing stubs

`.pyi` stubs for all public expressions — full mypy and pyright support.

### v2.5.0 — Integration: CLI tool

```bash
maskops run config.toml input.parquet output.parquet
```

Pipeline use without writing Python. Reads policy files from v1.8.0. Output can be Parquet, CSV, or Arrow IPC.

### v2.6.0 — Ecosystem: Listings

- `ddotta/awesome-polars` listing submitted and merged
- Official Polars plugins page PR merged (submitted at v1.0.0, tracked here as merged milestone)

### v2.7.0 — Performance: Parallelism

- `rayon`-based parallelism for multi-column batch masking
- SIMD investigation for hot regex paths
- Benchmark updated to reflect multi-column workloads

### v2.8.0 — Integration: Arrow IPC + DuckDB

- Arrow IPC reader/writer for zero-copy pipeline integration
- DuckDB compatibility validation

### v2.9.0 — Polish: Pre-3.0 cleanup

- API surface review
- Deprecation notices for any rough edges
- Migration guide for 3.0 breaking changes

### v3.0.0 — Milestone: Ecosystem maturity

Permitted breaking changes from 2.9.0 review applied. Stable typing, full docs, all ecosystem listings confirmed. This version represents the library as a mature, production-grade standard for Polars PII masking.

---

## Pattern coverage matrix at each major

| Family | v1.0 | v2.0 | v3.0 |
|--------|------|------|------|
| EU (IBAN, VAT, DNI/NIE, NIN, Personalausweis, NIR, CF, PESEL, BSN, personnummer) | ✓ | ✓ | ✓ |
| LatAm (RUT, CPF, CURP, ARG DNI, CO CC/NIT, PE DNI) | ✓ | ✓ | ✓ |
| US (SSN, passport, driver's license, NPI, Medicare, dates) | ✓ | ✓ | ✓ |
| Healthcare (NPI, MBI, NHS) | ✓ | ✓ | ✓ |
| APAC (SIN, TFN, My Number, RRN, NRIC, Aadhaar) | partial | ✓ | ✓ |
| MEA (ZA ID, IL ID) | — | ✓ | ✓ |
| Contact (email, phone, IP) | ✓ | ✓ | ✓ |
| Financial (credit cards, IBAN, VAT) | ✓ | ✓ | ✓ |

---

## Key decisions recorded

- **`global/` folder rejected** — replaced with `contact/` and `financial/` for specificity
- **Interleaved cadence** — coverage and infra milestones alternate to keep codebase healthy as pattern count grows
- **Enterprise features start in 1.x** — land as individual building blocks (0.6, 0.8, 1.2, 1.4, 1.6, 1.8), unify at 2.0
- **v3.0 is a permitted-break release** — breaking changes from 2.9.0 review applied, otherwise ecosystem maturity only
- **Consistent masking** uses HMAC-SHA256 + salt (one-way), distinct from FPE (reversible)
