# Changelog

All notable changes to this project will be documented in this file.
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased] — v1.0

- `maskops run <config.toml> <input.parquet> <output.parquet>` — CLI for pipeline use without writing Python

## [Unreleased] — v0.9

- French NIR (INSEE social security number): detection + asterisk masking
- Italian Codice Fiscale: detection + asterisk masking
- Uruguayan cédula de identidad: detection + asterisk, FPE, and consistent masking
- Canadian SIN: detection + asterisk, FPE, and consistent masking (formatted and compact)
- Australian TFN: detection + asterisk, FPE, and consistent masking (spaced and compact)

## [Unreleased] — v0.8

- `mask_pii(mode="consistent", salt=...)`: HMAC-SHA256 deterministic pseudonymization for digit PII

## [0.5.1](https://github.com/fcarvajalbrown/MaskOps/compare/v0.5.0...v0.5.1) (2026-06-03)


### Documentation

* mark v0.2–v0.7 complete in ROADMAP, update current version ([b9eadda](https://github.com/fcarvajalbrown/MaskOps/commit/b9eaddad03b6726e5dc3f37a214ff27c81549886))

## [0.5.0](https://github.com/fcarvajalbrown/MaskOps/compare/v0.4.0...v0.5.0) (2026-06-03)


### Features

* **patterns:** add NPI, MBI, NHS, Peruvian DNI (v0.7.0) ([96cff61](https://github.com/fcarvajalbrown/MaskOps/commit/96cff61702185fdf7763a28fde2818e64c8b2e61))

## [0.7.0] — 2026-06-03

### Added
- US NPI (National Provider Identifier): 10-digit HIPAA Luhn-validated; asterisk and FPE masking.
- Medicare Beneficiary Identifier (MBI): 11-char CMS format; always asterisked.
- NHS number (UK): 10-digit Modulus 11 validated; asterisk and FPE masking.
- Peruvian DNI: 8-digit compact format; asterisk and FPE masking.
- All four patterns available via `patterns=` selector.

## [0.6.0] — 2026-06-03

### Added
- `patterns=` argument on `mask_pii`, `contains_pii`, and `mask_pii_fpe` — limits detection/masking to the specified PII families. Backward-compatible: omitting `patterns` preserves existing behavior.

## [0.3.0](https://github.com/fcarvajalbrown/MaskOps/compare/v0.2.0...v0.3.0) (2026-06-03)


### Features

* **patterns:** add Ecuadorian cédula (Módulo 10, LOPDP compliance) ([2118200](https://github.com/fcarvajalbrown/MaskOps/commit/2118200db1e15edfd9220fe7b0e0dcaac10748ac))


### Documentation

* add v* tag deployment rule to CLAUDE.md ([517e256](https://github.com/fcarvajalbrown/MaskOps/commit/517e256521e6b1222dc31444d0dee178c3088cd6))

## [0.2.0](https://github.com/fcarvajalbrown/MaskOps/compare/v0.1.5...v0.2.0) (2026-06-03)


### Features

* **patterns:** add ARG DNI, CO CC, CO NIT + fix IPv4 range validation (v0.5.0) ([412d2d8](https://github.com/fcarvajalbrown/MaskOps/commit/412d2d8c7fbb9fa933e6990b6180bcfc2e281581))
* **patterns:** add US SSN and passport patterns (v0.3.0) ([d6a2ecc](https://github.com/fcarvajalbrown/MaskOps/commit/d6a2ecc67415e09fa797041e5070e3b0b467de22))
* **patterns:** reorganize src/patterns into named subfolders (v0.2.0) ([f1cb10d](https://github.com/fcarvajalbrown/MaskOps/commit/f1cb10dbff45685cbb9a35b762d939b06b851cc9))
* **pipeline:** verify lazy scan streaming + benchmark refresh (v0.4.0) ([8fcc7a4](https://github.com/fcarvajalbrown/MaskOps/commit/8fcc7a4a785e3db029fa926d004de5b994987219))
* **site:** add GitHub Pages website, SVG assets, i18n, and pricing ([e26b35b](https://github.com/fcarvajalbrown/MaskOps/commit/e26b35b5f25dfa2c4db89fdd3e55a55c683e1d44))


### Documentation

* add changelog ([8c9a3a5](https://github.com/fcarvajalbrown/MaskOps/commit/8c9a3a5b467ff54b4edece99e15d2c09470a200c))
* add changelog update rule to CLAUDE.md ([c19ee10](https://github.com/fcarvajalbrown/MaskOps/commit/c19ee105c82aafeda57611d520b23a3d96ddf2ba))
* add Chile GTM design spec ([7be3425](https://github.com/fcarvajalbrown/MaskOps/commit/7be3425d114c3790f9e90550500debc3d7109075))
* add CLAUDE.md with project guidance for Claude Code ([8d5f950](https://github.com/fcarvajalbrown/MaskOps/commit/8d5f9505da4c01ee642333b6202e869430427dbc))
* add contributor guide for adding new PII patterns ([2571184](https://github.com/fcarvajalbrown/MaskOps/commit/2571184e69eebd77fbe09e091ef0397bdefbb3bb))
* add discoverability research notes ([fae1a1d](https://github.com/fcarvajalbrown/MaskOps/commit/fae1a1d8f8886e615be8f233694683a4d5e4bbdd))
* add GDPR reference doc, slim CLAUDE.md compliance section ([4d9a0d1](https://github.com/fcarvajalbrown/MaskOps/commit/4d9a0d1953719cc79c045e347b6eb7ed6e3f9bf0))
* add GDPR/compliance hard rules to CLAUDE.md ([d49b4c8](https://github.com/fcarvajalbrown/MaskOps/commit/d49b4c83fd7e8b227518182820be360c58bbc14a))
* add ISO certification context to GTM spec and Chile regulatory reference ([3f69b6d](https://github.com/fcarvajalbrown/MaskOps/commit/3f69b6d6ff6de40cece9af5283ff9f4af7dfd6bd))
* add LATAM privacy law reference (8 jurisdictions) ([c464428](https://github.com/fcarvajalbrown/MaskOps/commit/c4644288a0d62aad37620b682bd0a49e9fa70a3a))
* add Node.js 20 deprecation TODO to AGENTS.md ([cb91c46](https://github.com/fcarvajalbrown/MaskOps/commit/cb91c469233b8e2aea73ca9ba6ec89928fb7d120))
* add security policy and responsible disclosure ([c2551af](https://github.com/fcarvajalbrown/MaskOps/commit/c2551afa9656cc5c795580284f41f3029c58b10f))
* add sequential work rule to CLAUDE.md ([632e6be](https://github.com/fcarvajalbrown/MaskOps/commit/632e6be4d0a57506a2c9258de0448092ef10b2f9))
* add universal working preferences to AGENTS.md ([fa3853d](https://github.com/fcarvajalbrown/MaskOps/commit/fa3853daafefaab91664b1b9506da61968f727df))
* **legal:** add CLA and GitHub Action enforcement ([364ad5f](https://github.com/fcarvajalbrown/MaskOps/commit/364ad5f8327384261cba56ca6c48711a45f62e93))
* reshuffle roadmap for LATAM commercial priority ([990c4c9](https://github.com/fcarvajalbrown/MaskOps/commit/990c4c95e633e7463b2ef4ddf041a262b21ad2d5))
* update AGENTS.md with agent interaction rules and clarify file handling ([831ca9d](https://github.com/fcarvajalbrown/MaskOps/commit/831ca9da87a0ff13e14037113a8e9ffd7d2fb5ab))

## [Unreleased]

### Changed
- License: MIT → GPL-3.0-or-later
- PyPI classifiers: added Healthcare Industry, expanded keyword coverage

## [0.5.0] — 2026-06-03

### Added
- Argentine DNI: dotted format (7–8 digits), suffix guard prevents false positives on RUT/CPF; asterisk and FPE masking.
- Colombian cédula de ciudadanía (CC): dotted format (7–10 digits), same suffix guard; asterisk and FPE masking.
- Colombian NIT: 9-digit body with DIAN Módulo 11 check digit validation; asterisk and FPE masking, check digit preserved.

### Fixed
- IPv4 masking: added octet range validation (0–255) — numbers like `1.234.567.890` no longer treated as IP addresses.

## [0.4.0] — 2026-06-03

### Added
- Lazy scan pipeline support verified: `scan_parquet` → `sink_parquet` works with all three expressions (`mask_pii`, `contains_pii`, `mask_pii_fpe`); 5 streaming integration tests added.
- Benchmark refresh: US family (SSN, passport) added; "All patterns" baseline updated to cover full pattern set.

## [0.3.0] — 2026-06-03

### Added
- SSN (US): dashed-format detection with area/group/serial validation, ITIN exclusion, and two known-invalid numbers excluded; supports asterisk and FPE masking.
- US passport number: letter + 8-digit format (ICAO 9303); always asterisked.

## [0.1.5] — 2026-05-xx

_Initial public release._
