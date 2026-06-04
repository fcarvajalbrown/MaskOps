---
name: kimi-security
description: Run a Kimi K2.6 security and GDPR compliance review of the current diff. Use before every PR.
disable-model-invocation: true
allowed-tools: Bash Read
---

## Current diff
!`git diff HEAD`

## Instructions

### 1. Load supporting context
Read [gdpr-rules.md](gdpr-rules.md).

### 2. Build the task brief
Read [prompt-template.md](prompt-template.md). Fill in:
- `{{DIFF}}` — the full diff injected above
- `{{GDPR_RULES}}` — contents of gdpr-rules.md

Write the filled brief to `/tmp/kimi_security_brief.md`.

### 3. Run Kimi

Run this command:

```
kimi --print --quiet -f /tmp/kimi_security_brief.md --prompt "Execute the security and GDPR compliance review in the attached file. Return findings only."
```

### 4. Validate findings

Discard any finding that:
- Claims FPE producing same-format output is a bug (it is correct behavior)
- Claims asterisk masking should be reversible (it must not be)
- Contradicts the hard rules in gdpr-rules.md

### 5. Report

For each confirmed finding:
- **Severity:** HIGH / MEDIUM / LOW
- **Location:** file:line
- **Issue:** what is wrong
- **Fix:** specific change needed

Do not proceed to PR creation if any HIGH findings remain unresolved.
