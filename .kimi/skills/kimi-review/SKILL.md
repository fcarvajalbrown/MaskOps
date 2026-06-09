---
name: kimi-review
description: Run a Kimi K2.6 code correctness review of the current diff. Catches logical bugs, edge cases, and API contract violations. Use before every PR alongside /kimi-security and /kimi-optimize.
disable-model-invocation: true
allowed-tools: Bash Read
---

## Current diff
!`git diff HEAD`

## Test file context
!`find tests -name '*.py' | sort`

## Instructions

### 1. Build the task brief
Read [prompt-template.md](prompt-template.md). Fill in:
- `{{DIFF}}` — the full diff injected above
- `{{TEST_FILES}}` — the test file list injected above

Write the filled brief to `/tmp/kimi_review_brief.md`.

### 2. Run Kimi

Run this command:

```
kimi --quiet -p "$(cat /tmp/kimi_review_brief.md)"
```

### 3. Filter findings

Discard any finding that:
- Flags FPE producing same-format output as a bug (correct behavior)
- Flags asterisk masking as needing a recovery path (must not exist)
- Contradicts behavior already validated by a passing test in `tests/`
- Is a style or performance suggestion (those belong in /kimi-optimize)

### 4. Report

For each confirmed finding:
- **Severity:** HIGH / MEDIUM / LOW
- **Location:** file:line
- **Bug:** what is wrong and what input triggers it
- **Fix:** specific change needed

Do not proceed to PR creation if any HIGH findings remain unresolved.
