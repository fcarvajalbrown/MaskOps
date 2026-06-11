# Next session handoff

One-time notes from the 2026-06-11 session. Act on these, then delete this file and remove the pointer line from `CLAUDE.md`.

## 1. Audit the repo against Anthropic best practices (primary task)

Now that v2.0 has shipped and settled, do a full pass over the whole repo for Anthropic / Claude Code best practices. Cover at least:
- `CLAUDE.md` (root and `tools/social/`): accuracy, clarity, anything contradicting current code.
- `.claude/settings.json`: hooks and permissions, release-guard hooks, least-privilege.
- Agent-readiness, security-review surface, and the documented code conventions (no-comments rule, no AI attribution, no em dashes in published prose, etc.).
- Anything drifting from the documented rules.
Produce findings plus concrete fixes, not just observations.

## 2. Decide the Enterprise Edition question

Revisit whether MaskOps should have an "Enterprise Edition" after 2.0. It may be nothing more than the same software plus support and updates, not new closed features.

Objective truth the user stated: Motion B (selling the IP) can realistically only happen after Motion A has clients. A buyer wants traction first. So building enterprise FEATURES now is Motion A work the user is wary of. Lean toward "support + updates" as the edition, not a pile of new closed code, unless there are clients to justify it. Decide direction with the user before building anything.

## 3. Re-think the stale-files / end-of-session hook

The user wants to reconsider this. Facts from last session:
- A `SessionEnd` hook runs on `exit`, but its output is not shown to the user, so it is for cleanup/logging only.
- A gated `Stop` hook is what actually surfaces a reminder (the dev.to reminder works this way).
- If built, keep it conservative and silent by default: only flag real drift (the 5 version files disagree, or a `draft_*.md` older than ~7 days). Never flag normal uncommitted work.
Decide whether to build it, and on which event.
