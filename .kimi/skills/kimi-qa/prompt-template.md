# Codebase Q&A — MaskOps

You are answering a question about MaskOps, a native Polars plugin for high-speed PII masking written in Rust.

## Architecture

- `src/lib.rs` — Polars expression registration (mask_pii, contains_pii, mask_pii_fpe)
- `src/patterns/mod.rs` — aggregators: mask_all(), mask_all_fpe(), contains_any_pii()
- `src/patterns/<family>.rs` — one file per PII type (eu/, latam/, contact/, financial/, healthcare/)
- `maskops/__init__.py` — Python API, wraps register_plugin_function

Pattern pipeline in mod.rs: non-digit PII first (mask_non_digit), then digit PII (mask_digit or mask_digit_fpe).

FPE mode uses FF3-1 AES-256. Asterisk mode is irreversible. Non-digit PII is always asterisked regardless of mode.

## File tree

{{FILE_TREE}}

## Recent changes

{{RECENT_CHANGES}}

## Relevant source code

{{RELEVANT_CODE}}

## Question

{{QUESTION}}

## Output format

Answer the question directly and concisely. Cite specific file:line locations for any code you reference. If you are uncertain, say so rather than guessing.
