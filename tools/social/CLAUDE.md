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

**No em dashes (—).** The em dash is the single most recognizable tell of AI writing. Detectors weight it, and readers notice. Never use one. Replace it with a period (most often), a comma, a colon, or parentheses. Restructure the sentence rather than reach for a dash. (The en dash `–` is fine inside numeric ranges like `11–163×`. That is standard typography, not a tell.)

---

## SEO & discoverability (do this on every post)

dev.to ranks well on Google, so the title and first lines carry real long-tail search weight. Optimize every post — not just for the dev.to feed, but for search, since capturing intent (e.g. the "Presidio alternative for LATAM" angle) is the strategic point.

1. **Title** — front-load real search keywords (`PII`, `Polars`, `Rust`, `Python`, `mask`, `anonymize`). A pure curiosity hook with no keywords ranks for nothing. Keep the hook, but put the keywords first.
2. **Meta description** — always pass `--description` (~150 chars, keyword-rich). Without it dev.to falls back to the first sentence, which is usually keyword-free.
3. **Cover image** — always pass `--cover` (URL, ~1000x420). More clicks and a proper social card.
4. **High-intent phrases in the body** — work in the terms buyers and LATAM actually search: "PII masking in Polars", "anonymize PII in Python", "Presidio alternative", "GDPR", "RUT/CPF detection".
5. **Outbound links** — link the GitHub repo and the PyPI page. Helps SEO and conversion.
6. **Tags** — balance reach feeds (`rust`, `python`, `polars`) with intent tags (`privacy`, `datascience`). Max 4.
7. **Canonical** — pass `--canonical` if the content lives elsewhere first.

The script supports `--description`, `--cover`, and `--canonical`; `--rules` prints this checklist too.

## Cover images

Every article gets a cover. The look is **classic LaTeX** — a typeset academic paper, not a render and not an AI image. This is deliberate: AI image generators (and rigid branded templates) read as machine-made; a plain typeset paper reads as human.

**Aesthetic rules:**
- **Font: Latin Modern / Computer Modern serif** (the real LaTeX face). The OTFs are bundled in `tools/social/fonts/` (`lmroman10-regular.otf`, `lmroman10-italic.otf`) and referenced from the SVG via `@font-face` with a relative `url('../fonts/...otf')`. Never substitute Calibri/Arial/AI fonts.
- **Hand-write each cover as a compact SVG** in `tools/social/covers/`, 1000x420. Keep it small — one SVG is the entire token cost; no base64 fonts in the file (the renderer embeds them at render time), no heavy pipeline.
- **Do not reuse one rigid template across articles.** A human varies layout — wording, spacing, what's emphasized. Identical verbose covers every time is itself an AI tell. Vary each one.
- **Brand palette, but kept dev.to-appropriate.** Use a **light paper base** (`#FBFBF6`) so it doesn't clash with dev.to's feed — never a dark slab. Bring the brand in through accents only: title in logo green `#0F2318`, a small mint `#3DDB81` pixel-grid mark nodding to the logo, rule and figures in `#2BA562`, muted footer in `#6B88A3`. (Full brand palette is a dark theme — `#0C1A2E`/`#0F2318` bg, `#3DDB81`/`#2BA562` green, `#D4E3F0`/`#6B88A3` text — but covers stay light.)

**Render to PNG** (dev.to's `--cover` needs a raster URL, not SVG):
```
python tools/social/render_cover.py covers/<name>.svg
```
This embeds the bundled fonts as data URIs and screenshots via headless Edge at 2x (output `<name>.png`, 2000x840), so the real Computer Modern face is baked in. Then host the PNG (commit it and use the raw GitHub URL) and pass that URL to `--cover`.

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

**6. Archive the post, then delete the scratch draft.** Always keep an archived copy of every generated post — dev.to in `tools/social/devto/`, LinkedIn in `tools/social/linkedin/`, named `<YYYY-MM-DD>-<slug>.md` with a header carrying the published URL, tags, cover path, and description. After archiving, delete the working `draft_*.md` (those are scratch and don't belong in the repo; the archive does).

---

## Cooldown and cadence

- Minimum 5 days between posts (enforced by the script — it will exit if too soon). Rationale: ~80% of a post's reactions land in the first 4 days, so 5 days lets the previous post clear its visibility window before the next competes for the same audience.
- **The user owns the cooldown clock.** `python tools/social/devto_post.py --restart-cooldown` resets the anchor to now (use when the user says "restart/reset the cooldown" or "the cooldown starts now"). `--force` overrides the cooldown for a single publish. Never re-argue an old post date as a blocker — if the user wants to post, use `--force`; if they want a fresh clock, use `--restart-cooldown`.
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

## LinkedIn

LinkedIn is the user's own channel ("my place"). It pairs with dev.to posts but has its own rules.

- **Language:** Spanish is the default; also produce an English version (ES leads, EN second) when asked.
- **Link placement:** when the post links to a dev.to article, put the link in the **first comment**, never the post body — LinkedIn suppresses reach on posts with outbound links in the body.
- **Hashtags — research fresh every time.** Do not reuse a static set. For each post, search for hashtags that are currently active but **niche enough to stand out** — a small mix of broad-reach and specific (e.g. broad `#DataEngineering` alongside niche `#Polars` / `#PIIMasking`). The goal is to peak interest each post, not repeat the same tags.
- **Cover image — yes, by exception.** Covers are normally a dev.to thing, but when the user asks for a LinkedIn post, generate one with the **same LaTeX process** (`covers/*.svg` → `render_cover.py`). LinkedIn favours a square 1080x1080 or a 1200x627 link image, so size the SVG for that, not dev.to's 1000x420.

## Files

| File | Purpose |
|------|---------|
| `devto_post.py` | Publish articles to dev.to |
| `devto_reminder.py` | Session-end reminder (run by Stop hook) |
| `devto_history.json` | Runtime post history (gitignored) |
| `render_cover.py` | Render a cover SVG to PNG (fonts baked in) |
| `draft_*.md` | Working scratch drafts — deleted after publish |
| `devto/<date>-<slug>.md` | Archived published dev.to posts |
| `linkedin/<date>-<slug>.md` | Archived LinkedIn posts (ES + EN) |
| `covers/` | Cover SVGs + rendered PNGs |
| `fonts/` | Latin Modern (Computer Modern) OTFs for covers |
