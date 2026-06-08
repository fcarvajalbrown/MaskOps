#!/usr/bin/env python3
"""
Usage:
    python tools/social/devto_post.py --title "..." --body article.md --tags polars,rust,python
    python tools/social/devto_post.py --dry-run --title "..." --body article.md
    python tools/social/devto_post.py --history
    python tools/social/devto_post.py --rules

Credentials via env var:
    DEVTO_API_KEY=your_api_key  (dev.to Settings > Account > API Keys)

Body can be a path to a .md file or inline markdown passed as a string.
Tags: comma-separated, max 4, lowercase. Good defaults: polars, rust, python, privacy.
"""

import argparse
import json
import os
import sys
from datetime import datetime, timezone
from pathlib import Path

import requests

DEVTO_API = "https://dev.to/api/articles"
HISTORY_FILE = Path(__file__).parent / "devto_history.json"
MIN_DAYS_BETWEEN_POSTS = 14

RULES = """
dev.to posting rules
====================
1. WAIT      At least 14 days since the last post.
2. NO DUPE   Never republish the same article. Update the existing one instead.
3. TAGS      Max 4 tags. Use: polars, rust, python, privacy, datascience, dataengineering.
4. TYPE      Mix it up — announcements, tutorials, benchmarks, deep-dives.
             Don't only post release notes.
5. TITLE     Clear and factual. dev.to is friendlier than HN but clickbait still hurts reach.
6. ENGAGE    Reply to comments within 24 hours.
7. CANONICAL If the content lives elsewhere first (GitHub, your site), set canonical_url.

WRITING STYLE
=============
Hemingway + Dijkstra: short declarative sentences, no adjectives that don't earn their place,
no hedging, no filler. State what the thing does. State what it doesn't do. Stop.
No "excited to share", no "thrilled to announce", no "journey". Say the thing.
"""


def load_history() -> dict:
    if HISTORY_FILE.exists():
        return json.loads(HISTORY_FILE.read_text())
    return {"posts": []}


def save_history(history: dict) -> None:
    HISTORY_FILE.write_text(json.dumps(history, indent=2))


def check_cooldown(history: dict) -> None:
    if not history["posts"]:
        return
    last = history["posts"][-1]
    last_date = datetime.fromisoformat(last["date"])
    days_ago = (datetime.now(timezone.utc) - last_date).days
    if days_ago < MIN_DAYS_BETWEEN_POSTS:
        wait = MIN_DAYS_BETWEEN_POSTS - days_ago
        sys.exit(
            f"BLOCKED: Last post was {days_ago} days ago ('{last['title']}').\n"
            f"Wait {wait} more day(s)."
        )


def resolve_body(body_arg: str) -> str:
    path = Path(body_arg)
    if path.exists():
        return path.read_text(encoding="utf-8")
    return body_arg


def publish(api_key: str, title: str, body: str, tags: list[str]) -> dict:
    resp = requests.post(
        DEVTO_API,
        headers={"api-key": api_key, "Content-Type": "application/json"},
        json={"article": {"title": title, "body_markdown": body, "published": True, "tags": tags}},
    )
    if resp.status_code not in (200, 201):
        sys.exit(f"dev.to API error {resp.status_code}: {resp.text}")
    return resp.json()


def main():
    parser = argparse.ArgumentParser(description="Publish an article to dev.to.")
    parser.add_argument("--title", help="Article title")
    parser.add_argument("--body", help="Path to a .md file or inline markdown string")
    parser.add_argument("--tags", default="polars,rust,python,privacy", help="Comma-separated tags (max 4)")
    parser.add_argument("--dry-run", action="store_true", help="Print payload without publishing")
    parser.add_argument("--rules", action="store_true", help="Print posting rules and exit")
    parser.add_argument("--history", action="store_true", help="Print post history and exit")
    args = parser.parse_args()

    if args.rules:
        print(RULES)
        return

    history = load_history()

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

    if args.dry_run:
        print(f"[dry-run] title: {args.title!r}")
        print(f"[dry-run] tags:  {tags}")
        print(f"[dry-run] body:  {len(body)} chars")
        check_cooldown(history)
        print("[dry-run] Cooldown check passed.")
        return

    check_cooldown(history)

    print("Publishing to dev.to...")
    result = publish(api_key, args.title, body, tags)

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
