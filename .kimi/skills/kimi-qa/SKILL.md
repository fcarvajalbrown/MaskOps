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
kimi --quiet -p "$(cat /tmp/kimi_qa_brief.md)"
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
