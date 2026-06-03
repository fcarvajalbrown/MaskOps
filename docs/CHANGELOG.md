# Changelog

All notable changes to this project will be documented in this file.
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Changed
- License: MIT → GPL-3.0-or-later
- PyPI classifiers: added Healthcare Industry, expanded keyword coverage

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
