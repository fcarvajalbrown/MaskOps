# Kimi Skills Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Create three Claude Code skills (`/kimi-security`, `/kimi-optimize`, `/kimi-qa`) that build a structured Kimi CLI prompt, run it, and validate the response — triggered before every PR.

**Architecture:** Each skill is a directory under `.claude/skills/` with a `SKILL.md` (instructions for Claude), a `prompt-template.md` (Kimi task brief scaffold), and where needed a `gdpr-rules.md` (extracted compliance context). Claude fills the template, writes it to `/tmp/`, and calls `kimi --print --quiet` via Bash.

**Tech Stack:** Claude Code skill system (YAML frontmatter + markdown), Kimi CLI (`kimi --print --quiet`), Bash, `git diff`

---

## File layout

```
.claude/skills/
  kimi-security/
    SKILL.md              ← create
    gdpr-rules.md         ← create
    prompt-template.md    ← create
  kimi-optimize/
    SKILL.md              ← create
    prompt-template.md    ← create
  kimi-qa/
    SKILL.md              ← create
    prompt-template.md    ← create
```

`.claude/settings.json` is already done — do not touch it.

---

## Task 1: kimi-security skill

**Files:**
- Create: `.claude/skills/kimi-security/SKILL.md`
- Create: `.claude/skills/kimi-security/gdpr-rules.md`
- Create: `.claude/skills/kimi-security/prompt-template.md`

- [ ] **Step 1: Create the skill directory**

```bash
mkdir -p .claude/skills/kimi-security
```

- [ ] **Step 2: Write SKILL.md**

Create `.claude/skills/kimi-security/SKILL.md` with this exact content:

```markdown
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
```

- [ ] **Step 3: Write gdpr-rules.md**

Create `.claude/skills/kimi-security/gdpr-rules.md` with this exact content:

```markdown
# GDPR Hard Rules for MaskOps Security Review

These are CORRECT behaviors in MaskOps — do not flag them as issues.

## FPE (Format-Preserving Encryption)

1. FPE output is pseudonymization (GDPR Art. 4(5)), NOT anonymization. Never claim FPE output is anonymous. This is correct by design.
2. FPE key is passed as a Polars Binary literal by the caller. MaskOps never stores it. Key separation is the compliance model. This is correct.
3. FPE produces same-length, same-format output as input. This is the definition of format-preserving encryption — not a bug.

## Asterisk masking

4. Asterisk masking is irreversible. There is no recovery mechanism. This is required, not a flaw.
5. Non-digit PII (IBAN, VAT, email, IP, EU IDs, CURP) is always asterisked regardless of FPE mode. This is correct.

## Network and air-gap

6. MaskOps makes zero network calls. Any code that opens a socket, makes an HTTP request, or calls an external API is a HIGH finding.

## Pattern compliance

7. Every pattern module must declare its compliance category in its module docstring: which regulation, FPE or asterisk-only, and what validation prevents false positives. Missing declarations are a LOW finding.
```

- [ ] **Step 4: Write prompt-template.md**

Create `.claude/skills/kimi-security/prompt-template.md` with this exact content:

```markdown
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
```

- [ ] **Step 5: Verify the skill is discovered**

Start or restart Claude Code in the MaskOps directory. Run:

```
/kimi-security
```

Expected: Claude loads the skill, runs `git diff HEAD` via dynamic injection, reads `gdpr-rules.md` and `prompt-template.md`, fills the template, writes `/tmp/kimi_security_brief.md`, and calls `kimi --print --quiet`.

If the skill does not appear in the `/` menu, run `/reload-plugins` or restart Claude Code.

- [ ] **Step 6: Commit**

```bash
git add .claude/skills/kimi-security/
git commit -m "feat(skills): add /kimi-security skill for pre-PR GDPR and security review"
```

---

## Task 2: kimi-optimize skill

**Files:**
- Create: `.claude/skills/kimi-optimize/SKILL.md`
- Create: `.claude/skills/kimi-optimize/prompt-template.md`

- [ ] **Step 1: Create the skill directory**

```bash
mkdir -p .claude/skills/kimi-optimize
```

- [ ] **Step 2: Write SKILL.md**

Create `.claude/skills/kimi-optimize/SKILL.md` with this exact content:

```markdown
---
name: kimi-optimize
description: Run a Kimi K2.6 Rust performance review of the current diff. Use before every PR.
disable-model-invocation: true
allowed-tools: Bash Read
---

## Current diff
!`git diff HEAD`

## Benchmark context
!`cargo criterion --benches --no-run --message-format human 2>&1 | tail -20`

## Instructions

### 1. Build the task brief
Read [prompt-template.md](prompt-template.md). Fill in:
- `{{DIFF}}` — the full diff injected above
- `{{BENCH_CONTEXT}}` — the benchmark context injected above

Write the filled brief to `/tmp/kimi_optimize_brief.md`.

### 2. Run Kimi

Run this command:

```
kimi --print --quiet -f /tmp/kimi_optimize_brief.md --prompt "Execute the Rust performance review in the attached file. Return optimization suggestions only."
```

### 3. Filter suggestions

Discard any suggestion that:
- Requires changing correctness behavior or test expectations
- Introduces `unsafe` without a documented safety invariant
- Changes public API signatures (`mask_pii`, `contains_pii`, `mask_pii_fpe`)

### 4. Report

For each confirmed suggestion:
- **Impact:** estimated gain (high / medium / low)
- **Location:** file:line
- **Before / After:** concrete Rust code snippets
```

- [ ] **Step 3: Write prompt-template.md**

Create `.claude/skills/kimi-optimize/prompt-template.md` with this exact content:

```markdown
# Rust Performance Review — MaskOps

You are reviewing a diff for MaskOps, a native Polars plugin for high-speed PII masking. The Rust core compiles to a cdylib. Per-row performance matters — this runs on DataFrames with millions of rows.

## Benchmark context

{{BENCH_CONTEXT}}

## What to look for

1. **Unnecessary heap allocations** — `.to_string()`, `.collect::<Vec<_>>()`, `.clone()` that could be avoided with borrowed types or in-place mutation
2. **Unparallelized hot loops** — iteration over Series or ChunkedArray rows that does not use `rayon::par_iter()` or `par_bridge()`
3. **Regex compiled inside loops** — `Regex::new(...)` called per-row instead of via `once_cell::sync::Lazy` or compiled once at the call site
4. **Inefficient iterator chains** — multiple `.map().filter().collect()` passes where a single pass would do
5. **String copies in masking** — `replace()` or `format!()` allocating new strings where byte-level in-place replacement would work

## Diff to review

```diff
{{DIFF}}
```

## Output format

For each suggestion:

**[IMPACT: high/medium/low]** `file.rs:line`

Before:
```rust
// existing code
```

After:
```rust
// optimized code
```

Reason: one sentence

Skip anything that requires unsafe, changes correctness, or modifies the public API signatures (mask_pii, contains_pii, mask_pii_fpe).

If no opportunities found, respond with exactly: `No optimization opportunities found.`
```

- [ ] **Step 4: Verify the skill is discovered**

Run in Claude Code:

```
/kimi-optimize
```

Expected: Claude loads the skill, injects the diff and benchmark context, fills the template, writes `/tmp/kimi_optimize_brief.md`, and calls `kimi --print --quiet`.

- [ ] **Step 5: Commit**

```bash
git add .claude/skills/kimi-optimize/
git commit -m "feat(skills): add /kimi-optimize skill for pre-PR Rust performance review"
```

---

## Task 3: kimi-qa skill

**Files:**
- Create: `.claude/skills/kimi-qa/SKILL.md`
- Create: `.claude/skills/kimi-qa/prompt-template.md`

- [ ] **Step 1: Create the skill directory**

```bash
mkdir -p .claude/skills/kimi-qa
```

- [ ] **Step 2: Write SKILL.md**

Create `.claude/skills/kimi-qa/SKILL.md` with this exact content:

```markdown
---
name: kimi-qa
description: Answer a MaskOps codebase question using Kimi K2.6 with full project context. Usage: /kimi-qa your question here
disable-model-invocation: true
argument-hint: [question]
allowed-tools: Bash Read Grep
---

## Question
$ARGUMENTS

## File tree
!`find src -name '*.rs' | sort`

## Recent changes
!`git log --oneline -15`

## Instructions

### 1. Identify relevant files

Based on the question in `$ARGUMENTS`, identify the 2–4 most relevant `.rs` source files. Read them.

### 2. Build the task brief
Read [prompt-template.md](prompt-template.md). Fill in:
- `{{QUESTION}}` — the question from `$ARGUMENTS`
- `{{FILE_TREE}}` — the file list injected above
- `{{RECENT_CHANGES}}` — the git log injected above
- `{{RELEVANT_CODE}}` — the source file contents you read in step 1

Write the filled brief to `/tmp/kimi_qa_brief.md`.

### 3. Run Kimi

Run this command:

```
kimi --print --quiet -f /tmp/kimi_qa_brief.md --prompt "Answer the question in the attached file."
```

### 4. Verify symbols

Extract every function name, type name, and module path Kimi mentions in its answer. For each one, grep:

```bash
grep -rn "SYMBOL" src/
```

Flag any symbol that returns zero matches as UNVERIFIED.

### 5. Report

Present Kimi's answer, then append:
- `VERIFIED` — if all mentioned symbols were confirmed in the codebase
- `UNVERIFIED symbols: [list]` — for any that could not be found
```

- [ ] **Step 3: Write prompt-template.md**

Create `.claude/skills/kimi-qa/prompt-template.md` with this exact content:

```markdown
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
```

- [ ] **Step 4: Verify the skill is discovered**

Run in Claude Code:

```
/kimi-qa what does mask_all do?
```

Expected: Claude loads the skill, identifies `src/patterns/mod.rs` as relevant, reads it, fills the template, writes `/tmp/kimi_qa_brief.md`, calls `kimi --print --quiet`, and verifies the symbols in Kimi's answer.

- [ ] **Step 5: Commit**

```bash
git add .claude/skills/kimi-qa/
git commit -m "feat(skills): add /kimi-qa skill for on-demand codebase Q&A via Kimi"
```

---

## Task 4: Final commit — settings.json and design doc

`.claude/settings.json` and the design doc were created earlier in the session but not committed yet.

- [ ] **Step 1: Commit settings.json and design doc**

```bash
git add .claude/settings.json docs/superpowers/specs/2026-06-04-kimi-skills-design.md docs/superpowers/plans/2026-06-04-kimi-skills.md
git commit -m "chore(config): add pre-push hooks, attribution enforcement, and Kimi skills design"
```

- [ ] **Step 2: Push**

```bash
git push
```
