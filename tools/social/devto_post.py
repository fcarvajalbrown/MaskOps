#!/usr/bin/env python3
import argparse
import json
import os
import sys
from datetime import datetime, timezone
from pathlib import Path

import requests

try:
    from dotenv import load_dotenv
    load_dotenv(Path(__file__).resolve().parent.parent.parent / ".env")
except ImportError:
    pass

DEVTO_API = "https://dev.to/api/articles"
HISTORY_FILE = Path(__file__).parent / "devto_history.json"
MIN_DAYS_BETWEEN_POSTS = 5

RULES = """
dev.to posting rules
====================
1. WAIT      At least 5 days since the last post — ~80% of a post's reactions land
             in the first 4 days, so 5 lets the previous one clear its window before
             the next competes for the same audience.
2. NO DUPE   Never republish the same article. Update the existing one instead.
3. TAGS      Max 4 tags. Use: polars, rust, python, privacy, datascience, dataengineering.
4. TYPE      Mix it up — announcements, tutorials, benchmarks, deep-dives.
             Don't only post release notes.
5. TITLE     Clear and factual. dev.to is friendlier than HN but clickbait still hurts reach.
6. ENGAGE    Reply to comments within 24 hours.
7. CANONICAL If the content lives elsewhere first (GitHub, your site), set canonical_url.

SEO / DISCOVERABILITY (do this every post — dev.to ranks well on Google)
====================
- TITLE       Front-load real search keywords (PII, Polars, Rust, Python, mask,
              anonymize). A pure curiosity hook with no keywords ranks for nothing.
- DESCRIPTION Always pass --description (~150 chars, keyword-rich). Without it dev.to
              falls back to your first sentence, which is usually keyword-free.
- COVER       Always pass --cover (URL, ~1000x420). More clicks + a real social card.
- KEYWORDS    Work the high-intent phrases into the body: "PII masking in Polars",
              "anonymize PII in Python", "Presidio alternative", "GDPR",
              "RUT/CPF detection". These are what buyers and LATAM search for.
- LINKS       Link the GitHub repo and the PyPI page — helps SEO and conversion.
- TAGS        Balance reach feeds (rust, python, polars) with intent (privacy,
              datascience). Max 4.

TIMING (data-backed)
====================
- Best days:  Monday, Tuesday, Wednesday. Avoid weekends.
- Best time:  12:00–18:00 UTC (8am–2pm EST / 5am–11am PST).
- Visibility: 80% of reactions accumulate in the first 4 days — timing matters most at publish.
- Length:     5-min read is the practical sweet spot. Longer (13 min) gets more reactions
              but only if the depth earns it.

WRITING STYLE
=============
Albert Camus + Dijkstra: short declarative sentences, first-person but not self-absorbed,
no sentiment, no hedging, no adjectives that don't earn their place.
The story comes first. Lead with the human context, then the technical content.
State what happened. State what the thing does. State what it doesn't do. Stop.
No "excited to share", no "thrilled to announce", no "journey". Say the thing.
No em dashes. They are the top AI-writing tell. Use a period, comma, colon, or
parentheses instead. The en dash in numeric ranges (11-163x) is fine.
"""


def load_history() -> dict:
    if HISTORY_FILE.exists():
        return json.loads(HISTORY_FILE.read_text())
    return {"posts": []}


def save_history(history: dict) -> None:
    HISTORY_FILE.write_text(json.dumps(history, indent=2))


def cooldown_anchor(history: dict) -> datetime | None:
    anchors = []
    if history["posts"]:
        anchors.append(datetime.fromisoformat(history["posts"][-1]["date"]))
    if history.get("cooldown_anchor"):
        anchors.append(datetime.fromisoformat(history["cooldown_anchor"]))
    return max(anchors) if anchors else None


def check_cooldown(history: dict) -> None:
    anchor = cooldown_anchor(history)
    if anchor is None:
        return
    days_ago = (datetime.now(timezone.utc) - anchor).days
    if days_ago < MIN_DAYS_BETWEEN_POSTS:
        wait = MIN_DAYS_BETWEEN_POSTS - days_ago
        sys.exit(
            f"BLOCKED: cooldown active (anchor {anchor.date()}, {days_ago}d ago).\n"
            f"Wait {wait} more day(s), restart it with --restart-cooldown, or override with --force."
        )


def resolve_body(body_arg: str) -> str:
    path = Path(body_arg)
    if path.exists():
        return path.read_text(encoding="utf-8")
    return body_arg


def build_article(title: str, body: str, tags: list[str], description: str | None,
                  cover_image: str | None, canonical_url: str | None) -> dict:
    article = {"title": title, "body_markdown": body, "published": True, "tags": tags}
    if description:
        article["description"] = description
    if cover_image:
        article["main_image"] = cover_image
    if canonical_url:
        article["canonical_url"] = canonical_url
    return article


def publish(api_key: str, article: dict) -> dict:
    resp = requests.post(
        DEVTO_API,
        headers={"api-key": api_key, "Content-Type": "application/json"},
        json={"article": article},
    )
    if resp.status_code not in (200, 201):
        sys.exit(f"dev.to API error {resp.status_code}: {resp.text}")
    return resp.json()


def update_article(api_key: str, article_id: str, article: dict) -> dict:
    resp = requests.put(
        f"{DEVTO_API}/{article_id}",
        headers={"api-key": api_key, "Content-Type": "application/json"},
        json={"article": article},
    )
    if resp.status_code not in (200, 201):
        sys.exit(f"dev.to API error {resp.status_code}: {resp.text}")
    return resp.json()


def main():
    parser = argparse.ArgumentParser(description="Publish an article to dev.to.")
    parser.add_argument("--title", help="Article title")
    parser.add_argument("--body", help="Path to a .md file or inline markdown string")
    parser.add_argument("--tags", default="polars,rust,python,privacy", help="Comma-separated tags (max 4)")
    parser.add_argument("--description", help="Meta description for SEO / social cards (~150 chars, keyword-rich)")
    parser.add_argument("--cover", help="Cover image URL (dev.to main_image, ~1000x420)")
    parser.add_argument("--canonical", help="canonical_url if the article lives elsewhere first")
    parser.add_argument("--dry-run", action="store_true", help="Print payload without publishing")
    parser.add_argument("--force", action="store_true", help="Override the cooldown for this publish")
    parser.add_argument("--update", help="Update an existing article by ID instead of publishing (no cooldown)")
    parser.add_argument("--restart-cooldown", action="store_true", help="Reset the cooldown clock to start now, then exit")
    parser.add_argument("--rules", action="store_true", help="Print posting rules and exit")
    parser.add_argument("--history", action="store_true", help="Print post history and exit")
    args = parser.parse_args()

    if args.rules:
        print(RULES)
        return

    history = load_history()

    if args.restart_cooldown:
        history["cooldown_anchor"] = datetime.now(timezone.utc).isoformat()
        save_history(history)
        print(f"Cooldown restarted now. Next post allowed in {MIN_DAYS_BETWEEN_POSTS} days "
              f"(or publish immediately with --force).")
        return

    if args.history:
        if not history["posts"]:
            print("No posts yet.")
        for p in history["posts"]:
            print(f"{p['date'][:10]}  {p['title']}")
            print(f"           {p.get('url', '')}")
        return

    if not args.title or not args.body:
        parser.print_help()
        sys.exit(1)

    api_key = os.environ.get("DEVTO_API_KEY")
    if not api_key:
        sys.exit("Set DEVTO_API_KEY environment variable. Get it at: dev.to > Settings > Account > API Keys")

    tags = [t.strip() for t in args.tags.split(",")][:4]
    body = resolve_body(args.body)
    article = build_article(args.title, body, tags, args.description, args.cover, args.canonical)

    if args.dry_run:
        print(f"[dry-run] title:       {args.title!r}")
        print(f"[dry-run] tags:        {tags}")
        print(f"[dry-run] description: {args.description!r}")
        print(f"[dry-run] cover:       {args.cover!r}")
        print(f"[dry-run] canonical:   {args.canonical!r}")
        print(f"[dry-run] body:        {len(body)} chars")
        if args.force:
            print("[dry-run] --force: cooldown bypassed.")
        else:
            check_cooldown(history)
            print("[dry-run] Cooldown check passed.")
        return

    if args.update:
        result = update_article(api_key, args.update, article)
        print(f"Updated: {result.get('url', '')}")
        return

    if args.force:
        print("WARNING: --force, bypassing cooldown.")
    else:
        check_cooldown(history)

    print("Publishing to dev.to...")
    result = publish(api_key, article)

    url = result.get("url", "")
    history["posts"].append({
        "date": datetime.now(timezone.utc).isoformat(),
        "title": args.title,
        "url": url,
        "id": result.get("id"),
    })
    save_history(history)

    print(f"Published: {url}")
    print("Reply to comments within 24 hours.")


if __name__ == "__main__":
    main()
