# Security and GDPR Compliance Review — MaskOps

You are reviewing a diff for MaskOps, a Rust-based PII masking library (Polars expression plugin).

## Hard rules — these are CORRECT behavior, do not flag them

{{GDPR_RULES}}

## What to look for

1. **GDPR violations** — overclaiming anonymization, missing compliance category declarations in module docstrings
2. **FPE misuse** — key stored alongside masked data, key logged or exposed, recovery mechanism added to asterisk masking
3. **Network calls** — any socket, HTTP, DNS, or external API call (all are HIGH severity)
4. **Key handling** — FPE key passed through unsafe channels, stored in memory beyond the call scope
5. **ReDoS-risk regex** — patterns with catastrophic backtracking (nested quantifiers, ambiguous alternation on long inputs)
6. **Unsafe Rust** — `unsafe` blocks without a documented safety invariant comment

## Diff to review

```diff
{{DIFF}}
```

## Output format

List each finding as:

**[SEVERITY]** `file.rs:line` — description
Fix: specific change

Severity: HIGH (exploitable or compliance-breaking) / MEDIUM (risk present, not immediately exploitable) / LOW (best-practice violation)

If no issues found, respond with exactly: `No issues found.`
