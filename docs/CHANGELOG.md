# Changelog

All notable changes to this project will be documented in this file.
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Features

* **audit:** add `mask_pii_audit` expression — masked value plus per-family validated match counts in a single pass, returned as a nested Struct (`masked` + `counts`)

## [1.5.1](https://github.com/fcarvajalbrown/MaskOps/compare/v1.5.0...v1.5.1) (2026-06-07)

### Features

* **consistent:** cross-column referential integrity — same value + same salt produces identical masked output across any number of columns

## [1.5.0](https://github.com/fcarvajalbrown/MaskOps/compare/v1.4.0...v1.5.0) (2026-06-07)


### Features

* add CPF, CURP masking + per-family benchmarks for v0.1.3 ([7929795](https://github.com/fcarvajalbrown/MaskOps/commit/7929795dcd2483d3f57f89672949324c1c8adfbd))
* **apac:** Japanese My Number + South Korean RRN (v1.5) ([#22](https://github.com/fcarvajalbrown/MaskOps/issues/22)) ([ff4ae73](https://github.com/fcarvajalbrown/MaskOps/commit/ff4ae733a11f4ce6fad6038dab88577f758271be))
* **api:** add patterns= argument to mask_pii, contains_pii, mask_pii_fpe (v0.6.0) ([b4efc2d](https://github.com/fcarvajalbrown/MaskOps/commit/b4efc2d8d59cee8da7f9ba17cb24c20435735eb4))
* **cli:** add maskops run command ([3e320c3](https://github.com/fcarvajalbrown/MaskOps/commit/3e320c3530b1308397c43b601de3c1ff2803d026))
* **cli:** delegate to load_policy(), add YAML and TOML dict format support ([aaf23ae](https://github.com/fcarvajalbrown/MaskOps/commit/aaf23ae93a0fb8abf99e36d12fe49481d08cbd32))
* **consistent:** add HMAC-SHA256 deterministic pseudonymization (v0.8) ([d7228ef](https://github.com/fcarvajalbrown/MaskOps/commit/d7228efc0517651fa223e53271261d5095608a96))
* **coverage:** add NIR, Codice Fiscale, UY CI, Canadian SIN, Australian TFN (v0.9) ([53f8c7b](https://github.com/fcarvajalbrown/MaskOps/commit/53f8c7bacc01e79e28ddfb87be40c9951abef0d1))
* **eu:** add Dutch BSN detection and masking ([6fe53e5](https://github.com/fcarvajalbrown/MaskOps/commit/6fe53e529e3e9503afb6ec231626f319d804d539))
* **eu:** add Polish PESEL detection and masking ([d3e94fd](https://github.com/fcarvajalbrown/MaskOps/commit/d3e94fda09a16c91c962ece796ce648b161aeb0b))
* **eu:** add Swedish personnummer detection and masking ([8d3a361](https://github.com/fcarvajalbrown/MaskOps/commit/8d3a361a936295e2e1f05e17cb1a3cf777bc554c))
* **extract:** add extract_pii expression returning 31-field Struct ([#17](https://github.com/fcarvajalbrown/MaskOps/issues/17)) ([672a5c9](https://github.com/fcarvajalbrown/MaskOps/commit/672a5c91a9c3e47b71ef7fe333ece87682b1afbc))
* **pages:** add sitemap, robots.txt, schema.org, og/twitter tags, icon png ([218ac4b](https://github.com/fcarvajalbrown/MaskOps/commit/218ac4b984c92f9f5733bb47b8448de20ec2c31e))
* **patterns:** add ARG DNI, CO CC, CO NIT + fix IPv4 range validation (v0.5.0) ([412d2d8](https://github.com/fcarvajalbrown/MaskOps/commit/412d2d8c7fbb9fa933e6990b6180bcfc2e281581))
* **patterns:** add Ecuadorian cédula (Módulo 10, LOPDP compliance) ([2118200](https://github.com/fcarvajalbrown/MaskOps/commit/2118200db1e15edfd9220fe7b0e0dcaac10748ac))
* **patterns:** add NPI, MBI, NHS, Peruvian DNI (v0.7.0) ([96cff61](https://github.com/fcarvajalbrown/MaskOps/commit/96cff61702185fdf7763a28fde2818e64c8b2e61))
* **patterns:** add US SSN and passport patterns (v0.3.0) ([d6a2ecc](https://github.com/fcarvajalbrown/MaskOps/commit/d6a2ecc67415e09fa797041e5070e3b0b467de22))
* **patterns:** reorganize src/patterns into named subfolders (v0.2.0) ([f1cb10d](https://github.com/fcarvajalbrown/MaskOps/commit/f1cb10dbff45685cbb9a35b762d939b06b851cc9))
* **pipeline:** verify lazy scan streaming + benchmark refresh (v0.4.0) ([8fcc7a4](https://github.com/fcarvajalbrown/MaskOps/commit/8fcc7a4a785e3db029fa926d004de5b994987219))
* **policy:** add load_policy() with YAML support and env var interpolation ([cb2b8c5](https://github.com/fcarvajalbrown/MaskOps/commit/cb2b8c5186b897373759a2ea2be4be4d200c1c81))
* **policy:** add Policy class with apply() method ([1ad2d2b](https://github.com/fcarvajalbrown/MaskOps/commit/1ad2d2bb2e5b4fc9369bd8559ecc9ca67f64eb6c))
* **site:** add GitHub Pages website, SVG assets, i18n, and pricing ([e26b35b](https://github.com/fcarvajalbrown/MaskOps/commit/e26b35b5f25dfa2c4db89fdd3e55a55c683e1d44))
* **skills:** add /kimi-optimize skill for pre-PR Rust performance review ([8537c79](https://github.com/fcarvajalbrown/MaskOps/commit/8537c79bedd89e64fcee2d71e5aa43a61770a81b))
* **skills:** add /kimi-qa skill for on-demand codebase Q&A via Kimi ([c407402](https://github.com/fcarvajalbrown/MaskOps/commit/c4074027da5213daae77d892a0bfdcb1fa024c14))
* **skills:** add /kimi-security skill for pre-PR GDPR and security review ([e7366ab](https://github.com/fcarvajalbrown/MaskOps/commit/e7366ab0790b66034620b8f0faf740d3c94fad71))
* v0.1.5 — FF3-1 FPE, European IDs, credit cards, 97 tests ([d6403fd](https://github.com/fcarvajalbrown/MaskOps/commit/d6403fd6e924a342d3f74e37fb796aac1d654029))


### Bug Fixes

* **publish:** use plain string for license field to fix twine metadata error ([c806eb9](https://github.com/fcarvajalbrown/MaskOps/commit/c806eb9a724b737013b5fee6abb874bdbf2f1c39))
* **skills:** correct kimi CLI invocation flag (-f → --quiet -p) ([f66024d](https://github.com/fcarvajalbrown/MaskOps/commit/f66024d3c6b5dbb32a65c3f465323a46546d71a2))
* use inline table for license to avoid PEP 639 license-file metadata header ([869fe09](https://github.com/fcarvajalbrown/MaskOps/commit/869fe091e398595ecbd4419a5770444096a12efe))


### Documentation

* add changelog ([8c9a3a5](https://github.com/fcarvajalbrown/MaskOps/commit/8c9a3a5b467ff54b4edece99e15d2c09470a200c))
* add changelog update rule to CLAUDE.md ([c19ee10](https://github.com/fcarvajalbrown/MaskOps/commit/c19ee105c82aafeda57611d520b23a3d96ddf2ba))
* add Chile GTM design spec ([7be3425](https://github.com/fcarvajalbrown/MaskOps/commit/7be3425d114c3790f9e90550500debc3d7109075))
* add CLAUDE.md with project guidance for Claude Code ([8d5f950](https://github.com/fcarvajalbrown/MaskOps/commit/8d5f9505da4c01ee642333b6202e869430427dbc))
* add CLI entry to changelog ([1e765c6](https://github.com/fcarvajalbrown/MaskOps/commit/1e765c6389d6faabddd9635526caa02bafd0037d))
* add contributor guide for adding new PII patterns ([2571184](https://github.com/fcarvajalbrown/MaskOps/commit/2571184e69eebd77fbe09e091ef0397bdefbb3bb))
* add discoverability research notes ([fae1a1d](https://github.com/fcarvajalbrown/MaskOps/commit/fae1a1d8f8886e615be8f233694683a4d5e4bbdd))
* add GDPR reference doc, slim CLAUDE.md compliance section ([4d9a0d1](https://github.com/fcarvajalbrown/MaskOps/commit/4d9a0d1953719cc79c045e347b6eb7ed6e3f9bf0))
* add GDPR/compliance hard rules to CLAUDE.md ([d49b4c8](https://github.com/fcarvajalbrown/MaskOps/commit/d49b4c83fd7e8b227518182820be360c58bbc14a))
* add ISO certification context to GTM spec and Chile regulatory reference ([3f69b6d](https://github.com/fcarvajalbrown/MaskOps/commit/3f69b6d6ff6de40cece9af5283ff9f4af7dfd6bd))
* add LATAM privacy law reference (8 jurisdictions) ([c464428](https://github.com/fcarvajalbrown/MaskOps/commit/c4644288a0d62aad37620b682bd0a49e9fa70a3a))
* add monthly downloads badge and v1.0.0 API stability guarantee ([9d2551a](https://github.com/fcarvajalbrown/MaskOps/commit/9d2551ae5cce3ac5a28ff6c642753c88d37a2c55))
* add Node.js 20 deprecation TODO to AGENTS.md ([cb91c46](https://github.com/fcarvajalbrown/MaskOps/commit/cb91c469233b8e2aea73ca9ba6ec89928fb7d120))
* add security policy and responsible disclosure ([c2551af](https://github.com/fcarvajalbrown/MaskOps/commit/c2551afa9656cc5c795580284f41f3029c58b10f))
* add sequential work rule to CLAUDE.md ([632e6be](https://github.com/fcarvajalbrown/MaskOps/commit/632e6be4d0a57506a2c9258de0448092ef10b2f9))
* add universal working preferences to AGENTS.md ([fa3853d](https://github.com/fcarvajalbrown/MaskOps/commit/fa3853daafefaab91664b1b9506da61968f727df))
* add v* tag deployment rule to CLAUDE.md ([517e256](https://github.com/fcarvajalbrown/MaskOps/commit/517e256521e6b1222dc31444d0dee178c3088cd6))
* add v2 migration guide ([e6c0530](https://github.com/fcarvajalbrown/MaskOps/commit/e6c05302f9d73b4e16940b7db54b1b1783c59372))
* **changelog:** add v1.2.0 EU depth entries ([9702710](https://github.com/fcarvajalbrown/MaskOps/commit/970271040ccb796f6d175a0cec20a22628191609))
* **claude:** branch only for roadmap releases ([43ff36b](https://github.com/fcarvajalbrown/MaskOps/commit/43ff36b96a6bc1686d4409d336e27316b9ff120e))
* **legal:** add CLA and GitHub Action enforcement ([364ad5f](https://github.com/fcarvajalbrown/MaskOps/commit/364ad5f8327384261cba56ca6c48711a45f62e93))
* mark v0.2–v0.7 complete in ROADMAP, update current version ([b9eadda](https://github.com/fcarvajalbrown/MaskOps/commit/b9eaddad03b6726e5dc3f37a214ff27c81549886))
* mark v0.8 complete, add consistent masking changelog entry ([d44a1e6](https://github.com/fcarvajalbrown/MaskOps/commit/d44a1e65541fb862dbb162ac7d29157e64ef6bfb))
* mark v0.9 complete, add coverage changelog entries ([a51ebe1](https://github.com/fcarvajalbrown/MaskOps/commit/a51ebe129d8dc739cfb578f252b8e4b4ce2b2f5c))
* mark v1.0 complete in roadmap ([f56b364](https://github.com/fcarvajalbrown/MaskOps/commit/f56b364d5eb5e36367119aae15bf4f6fd1b03f09))
* **readme:** restructure for seo and update stale refs ([75fcf28](https://github.com/fcarvajalbrown/MaskOps/commit/75fcf284fe75c2d0abe93414e2c7b844b6ae77ee))
* reshuffle roadmap for LATAM commercial priority ([990c4c9](https://github.com/fcarvajalbrown/MaskOps/commit/990c4c95e633e7463b2ef4ddf041a262b21ad2d5))
* update AGENTS.md with agent interaction rules and clarify file handling ([831ca9d](https://github.com/fcarvajalbrown/MaskOps/commit/831ca9da87a0ff13e14037113a8e9ffd7d2fb5ab))

## [0.14.1](https://github.com/fcarvajalbrown/MaskOps/compare/v0.14.0...v0.14.1) (2026-06-07)


### Documentation

* **claude:** branch only for roadmap releases ([43ff36b](https://github.com/fcarvajalbrown/MaskOps/commit/43ff36b96a6bc1686d4409d336e27316b9ff120e))

## [0.14.0](https://github.com/fcarvajalbrown/MaskOps/compare/v0.13.0...v0.14.0) (2026-06-06)


### Features

* **extract:** add extract_pii expression returning 31-field Struct ([#17](https://github.com/fcarvajalbrown/MaskOps/issues/17)) ([672a5c9](https://github.com/fcarvajalbrown/MaskOps/commit/672a5c91a9c3e47b71ef7fe333ece87682b1afbc))

## [0.14.0] (unreleased)

### Features

* **extract_pii:** new Polars expression returning a 31-field Struct with the first match per PII family; enables routing, reporting, and selective masking without re-scanning
* **extract_pii:** supports all 31 PII families: email, phone, ip, iban, vat, dni, nie, nin, personalausweis, nir, codice_fiscale, pesel, bsn, personnummer, credit_card, ssn, us_passport, rut, cpf, curp, arg_dni, co_cc, co_nit, ec_cedula, pe_dni, uy_ci, npi, mbi, nhs, sin, tfn

## [0.13.0](https://github.com/fcarvajalbrown/MaskOps/compare/v0.12.3...v0.13.0) (2026-06-06)


### Features

* **pages:** add sitemap, robots.txt, schema.org, og/twitter tags, icon png ([218ac4b](https://github.com/fcarvajalbrown/MaskOps/commit/218ac4b984c92f9f5733bb47b8448de20ec2c31e))


### Documentation

* **readme:** restructure for seo and update stale refs ([75fcf28](https://github.com/fcarvajalbrown/MaskOps/commit/75fcf284fe75c2d0abe93414e2c7b844b6ae77ee))

## [0.12.3](https://github.com/fcarvajalbrown/MaskOps/compare/v0.12.2...v0.12.3) (2026-06-04)


### Bug Fixes

* use inline table for license to avoid PEP 639 license-file metadata header ([869fe09](https://github.com/fcarvajalbrown/MaskOps/commit/869fe091e398595ecbd4419a5770444096a12efe))


### Documentation

* add v2 migration guide ([e6c0530](https://github.com/fcarvajalbrown/MaskOps/commit/e6c05302f9d73b4e16940b7db54b1b1783c59372))

## [0.12.2](https://github.com/fcarvajalbrown/MaskOps/compare/v0.12.1...v0.12.2) (2026-06-04)


### Documentation

* add monthly downloads badge and v1.0.0 API stability guarantee ([9d2551a](https://github.com/fcarvajalbrown/MaskOps/commit/9d2551ae5cce3ac5a28ff6c642753c88d37a2c55))
* mark v1.0 complete in roadmap ([f56b364](https://github.com/fcarvajalbrown/MaskOps/commit/f56b364d5eb5e36367119aae15bf4f6fd1b03f09))

## [0.12.1](https://github.com/fcarvajalbrown/MaskOps/compare/v0.12.0...v0.12.1) (2026-06-04)


### Bug Fixes

* **skills:** correct kimi CLI invocation flag (-f → --quiet -p) ([f66024d](https://github.com/fcarvajalbrown/MaskOps/commit/f66024d3c6b5dbb32a65c3f465323a46546d71a2))

## [0.12.0](https://github.com/fcarvajalbrown/MaskOps/compare/v0.11.0...v0.12.0) (2026-06-04)


### Features

* **skills:** add /kimi-optimize skill for pre-PR Rust performance review ([8537c79](https://github.com/fcarvajalbrown/MaskOps/commit/8537c79bedd89e64fcee2d71e5aa43a61770a81b))
* **skills:** add /kimi-qa skill for on-demand codebase Q&A via Kimi ([c407402](https://github.com/fcarvajalbrown/MaskOps/commit/c4074027da5213daae77d892a0bfdcb1fa024c14))
* **skills:** add /kimi-security skill for pre-PR GDPR and security review ([e7366ab](https://github.com/fcarvajalbrown/MaskOps/commit/e7366ab0790b66034620b8f0faf740d3c94fad71))

## [0.11.0](https://github.com/fcarvajalbrown/MaskOps/compare/v0.10.0...v0.11.0) (2026-06-04)


### Features

* **eu:** add Dutch BSN detection and masking ([6fe53e5](https://github.com/fcarvajalbrown/MaskOps/commit/6fe53e529e3e9503afb6ec231626f319d804d539))
* **eu:** add Polish PESEL detection and masking ([d3e94fd](https://github.com/fcarvajalbrown/MaskOps/commit/d3e94fda09a16c91c962ece796ce648b161aeb0b))
* **eu:** add Swedish personnummer detection and masking ([8d3a361](https://github.com/fcarvajalbrown/MaskOps/commit/8d3a361a936295e2e1f05e17cb1a3cf777bc554c))


### Documentation

* **changelog:** add v1.2.0 EU depth entries ([9702710](https://github.com/fcarvajalbrown/MaskOps/commit/970271040ccb796f6d175a0cec20a22628191609))

## [Unreleased]

### Features

- Add Polish PESEL detection and masking (asterisk, FPE, consistent; UODO/GDPR Art. 4(1))
- Add Dutch BSN detection and masking (asterisk, FPE, consistent; Dutch AVG/GDPR)
- Add Swedish personnummer detection and masking (asterisk, FPE, consistent; Dataskyddslagen)

## [0.10.0](https://github.com/fcarvajalbrown/MaskOps/compare/v0.9.0...v0.10.0) (2026-06-04)


### Features

* **cli:** delegate to load_policy(), add YAML and TOML dict format support ([aaf23ae](https://github.com/fcarvajalbrown/MaskOps/commit/aaf23ae93a0fb8abf99e36d12fe49481d08cbd32))
* **policy:** add load_policy() with YAML support and env var interpolation ([cb2b8c5](https://github.com/fcarvajalbrown/MaskOps/commit/cb2b8c5186b897373759a2ea2be4be4d200c1c81))
* **policy:** add Policy class with apply() method ([1ad2d2b](https://github.com/fcarvajalbrown/MaskOps/commit/1ad2d2bb2e5b4fc9369bd8559ecc9ca67f64eb6c))


### Bug Fixes

* **publish:** use plain string for license field to fix twine metadata error ([c806eb9](https://github.com/fcarvajalbrown/MaskOps/commit/c806eb9a724b737013b5fee6abb874bdbf2f1c39))

## [0.7.0](https://github.com/fcarvajalbrown/MaskOps/compare/v0.6.0...v0.7.0) (2026-06-03)


### Features

* **cli:** add maskops run command ([3e320c3](https://github.com/fcarvajalbrown/MaskOps/commit/3e320c3530b1308397c43b601de3c1ff2803d026))
* **coverage:** add NIR, Codice Fiscale, UY CI, Canadian SIN, Australian TFN (v0.9) ([53f8c7b](https://github.com/fcarvajalbrown/MaskOps/commit/53f8c7bacc01e79e28ddfb87be40c9951abef0d1))


### Documentation

* add CLI entry to changelog ([1e765c6](https://github.com/fcarvajalbrown/MaskOps/commit/1e765c6389d6faabddd9635526caa02bafd0037d))
* mark v0.9 complete, add coverage changelog entries ([a51ebe1](https://github.com/fcarvajalbrown/MaskOps/commit/a51ebe129d8dc739cfb578f252b8e4b4ce2b2f5c))

## [Unreleased] — v1.1

- `maskops.load_policy(path)`: load YAML or TOML policy files as a Python API
- Policy files support env var interpolation (`${VAR_NAME}`) in any string value
- CLI now accepts YAML policy files and TOML dict-format (`[columns.name]`) configs

## [Unreleased] — v1.0

- `maskops run <config.toml> <input.parquet> <output.parquet>` — CLI for pipeline use without writing Python

## [Unreleased] — v0.9

- French NIR (INSEE social security number): detection + asterisk masking
- Italian Codice Fiscale: detection + asterisk masking
- Uruguayan cédula de identidad: detection + asterisk, FPE, and consistent masking
- Canadian SIN: detection + asterisk, FPE, and consistent masking (formatted and compact)
- Australian TFN: detection + asterisk, FPE, and consistent masking (spaced and compact)

## [0.6.0](https://github.com/fcarvajalbrown/MaskOps/compare/v0.5.1...v0.6.0) (2026-06-03)


### Features

* **consistent:** add HMAC-SHA256 deterministic pseudonymization (v0.8) ([d7228ef](https://github.com/fcarvajalbrown/MaskOps/commit/d7228efc0517651fa223e53271261d5095608a96))


### Documentation

* mark v0.8 complete, add consistent masking changelog entry ([d44a1e6](https://github.com/fcarvajalbrown/MaskOps/commit/d44a1e65541fb862dbb162ac7d29157e64ef6bfb))

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
