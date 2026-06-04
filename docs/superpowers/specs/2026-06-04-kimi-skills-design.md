# Kimi Skills Design

**Date:** 2026-06-04
**Status:** Approved

---

## Problem

MaskOps releases need a lightweight second-opinion pass before each PR: security/GDPR compliance, Rust optimization opportunities, and ad-hoc codebase Q&A. Claude Code handles architecture and implementation; Kimi K2.6 (running in a separate terminal via Kimi CLI) handles these specialized reviews.

---

## Solution

Three Claude Code skills that drive a context → Kimi → validate loop, triggered before each PR. One settings.json with hard guards and a pre-push reminder.

---

## Components

### 1. `.claude/settings.json` (implemented)

Four behavioral rules moved out of CLAUDE.md into enforced hooks:

| Hook | Type | Trigger |
|---|---|---|
| Force-push blocker | Hard block (`continue: false`) | `git push --force*` / `git push -f*` |
| v* tag blocker | Hard block (`continue: false`) | `git tag v*` |
| Kimi commit reminder | System message (non-blocking) | `git commit*` |
| Kimi push reminder | System message (non-blocking) | `git push*` |

Also sets `attribution.commit: ""` and `attribution.pr: ""` — enforces no AI attribution mechanically instead of by instruction.

### 2. `/kimi-security` skill

**File:** `.claude/skills/kimi-security/SKILL.md`

**When invoked:**
1. Read `docs/gdpr/gdpr-reference.md` and the CLAUDE.md hard rules section
2. Inject live diff via `!`git diff HEAD``
3. Build a structured task brief covering: GDPR violations, FPE misuse, network calls, key handling, ReDoS-risk regex patterns
4. Run: `kimi --print --quiet --prompt "[brief]"`
5. Filter findings against known-correct behavior (e.g., FPE is always pseudonymization — not a finding)
6. Report confirmed findings with severity (HIGH/MEDIUM/LOW), file:line, and suggested fix

**Hard rules Kimi checks against:**
- FPE = pseudonymization, not anonymization (GDPR Art. 4(5)) — never a false positive
- FPE key must never be stored alongside masked data
- Asterisk masking is irreversible — no recovery mechanism allowed
- No network calls, ever
- New patterns must declare compliance category in module docstring

**Supporting files:**
- `gdpr-rules.md` — extracted hard rules for Kimi context
- `prompt-template.md` — Kimi task brief scaffold

### 3. `/kimi-optimize` skill

**File:** `.claude/skills/kimi-optimize/SKILL.md`

**When invoked:**
1. Inject live diff via `!`git diff HEAD``
2. Run `cargo criterion --benches --no-run 2>&1 | head -40` for benchmark context
3. Build task brief: identify Rust-specific perf issues in new code (unnecessary allocations, unparallelized hot loops, suboptimal iterator chains, rayon misuse)
4. Run: `kimi --print --quiet --prompt "[brief]"`
5. For each suggestion: present as before/after snippet with estimated impact
6. Skip suggestions that touch correctness or safety — perf only

**Supporting files:**
- `prompt-template.md` — Kimi task brief scaffold

### 4. `/kimi-qa` skill

**File:** `.claude/skills/kimi-qa/SKILL.md`

**Arguments:** `$ARGUMENTS` — your question

**When invoked:**
1. Inject file tree + key type definitions from `src/` via `!`find src -name '*.rs' | head -30``
2. Inject recent git log via `!`git log --oneline -10``
3. Build task brief with the user's question + codebase snapshot
4. Run: `kimi --print --quiet --prompt "[brief]"`
5. Grep for any symbols Kimi mentions — flag ones that don't exist in the codebase
6. Present answer with a confidence flag (VERIFIED / UNVERIFIED symbols)

**Supporting files:**
- `prompt-template.md` — Kimi task brief scaffold

---

## File layout

```
.claude/
  settings.json                          ← hooks + attribution (done)
  skills/
    kimi-security/
      SKILL.md
      gdpr-rules.md
      prompt-template.md
    kimi-optimize/
      SKILL.md
      prompt-template.md
    kimi-qa/
      SKILL.md
      prompt-template.md
```

---

## Invocation cadence

| Skill | When |
|---|---|
| `/kimi-security` | Before every PR |
| `/kimi-optimize` | Before every PR |
| `/kimi-qa` | On demand |

The pre-push/commit hook in settings.json provides the reminder automatically.

---

## Kimi CLI command

```bash
kimi --print --quiet --prompt "TASK BRIEF"
```

- `--print` — non-interactive, exits after response
- `--quiet` — final message only, no spinner or metadata
- Model: Kimi K2.6 (configured in Kimi CLI settings)
- API: Moonshot AI (`api.moonshot.ai/v1`)

---

## What stays in CLAUDE.md

Rules that require interpretation rather than automation: code style, Rust conventions, bug fix philosophy, GDPR compliance model, build instructions, commit message format, response style. Settings.json has no concept of these.

---

## Out of scope

- Automatic Kimi invocation without user trigger
- Kimi writing code (review only)
- Any changes to opencode.json
