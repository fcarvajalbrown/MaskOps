# tools/social

Publishing scripts for MaskOps marketing. Currently: dev.to only.

## Credential setup

`devto_post.py` reads `DEVTO_API_KEY` from the environment. Two ways to supply it:

**Option A — `.env` file (recommended for local dev):**
Paste your key into `.env` at the project root:
```
DEVTO_API_KEY=your_key_here
```
The script loads this automatically via `python-dotenv` (`pip install python-dotenv`). The `.env` file is gitignored — it will never be committed. Get the key at: dev.to > Settings > Account > API Keys.

**Option B — persistent environment variable (Windows):**
```
setx DEVTO_API_KEY "your_key_here"
```
Restart the terminal after running `setx`.

---

## Writing style

Albert Camus + Dijkstra. Short declarative sentences. First-person but not self-absorbed. No sentiment, no hedging, no adjectives that don't earn their place.

**The story comes first.** Lead with the human context — what you were doing, what interrupted it, what brought you back. Then the technical content. Readers trust honesty more than polish.

State what happened. State what the thing does. State what it doesn't do. Stop.

No "excited to share", no "thrilled to announce", no "journey". Say the thing.

---

## Publishing workflow

Publishing to dev.to is public and irreversible. Always follow this sequence:

**1. Print the rules:**
```
python tools/social/devto_post.py --rules
```

**2. Write the article** as a `draft_*.md` file inside `tools/social/`.

**3. Dry-run — confirm title, tags, char count, and cooldown pass:**
```
python tools/social/devto_post.py --dry-run \
  --title "Your Title" \
  --body tools/social/draft_something.md \
  --tags polars,rust,python,privacy
```

**4. Get explicit user confirmation** before publishing. The dry-run is not enough — Claude must stop and wait for "yes, publish" before proceeding to step 5.

**5. Publish (user confirms, then Claude runs):**
```
python tools/social/devto_post.py \
  --title "Your Title" \
  --body tools/social/draft_something.md \
  --tags polars,rust,python,privacy
```

**6. Delete the draft file** after a confirmed successful publish. Draft files (`draft_*.md`) are temporary — they do not belong in the repo.

---

## Cooldown and cadence

- Minimum 5 days between posts (enforced by the script — it will exit if too soon). Rationale: ~80% of a post's reactions land in the first 4 days, so 5 days lets the previous post clear its visibility window before the next competes for the same audience.
- A 21-day reminder fires automatically at the end of each Claude Code session via the Stop hook in `.claude/settings.json`. It only prints if >= 21 days have passed since the last post.
- **Post at milestones only:** performance sweep results, v2.0.0, tutorials, benchmark comparisons. Not every release.

## Timing (data-backed)

- **Best days:** Monday, Tuesday, Wednesday. Avoid weekends.
- **Best window:** 12:00–18:00 UTC (8am–2pm EST / 5am–11am PST).
- **Visibility cliff:** ~80% of reactions accumulate in the first 4 days after publish — timing at publish matters more than anything done after.
- **Length:** 5-min read is the practical sweet spot. Longer articles get more reactions only when the depth earns it.

## History

```
python tools/social/devto_post.py --history
```

Post history is stored in `devto_history.json` (gitignored, local only).

---

## Files

| File | Purpose |
|------|---------|
| `devto_post.py` | Publish articles to dev.to |
| `devto_reminder.py` | Session-end reminder (run by Stop hook) |
| `devto_history.json` | Runtime post history (gitignored) |
| `draft_*.md` | Working drafts — deleted after publish |
