#!/usr/bin/env python3
import json
from datetime import datetime, timezone
from pathlib import Path

HISTORY_FILE = Path(__file__).parent / "devto_history.json"
REMIND_AFTER_DAYS = 14

def is_posting_day():
    return datetime.now(timezone.utc).astimezone().weekday() <= 2

def main():
    if not is_posting_day():
        return

    if not HISTORY_FILE.exists():
        msg = "dev.to: no post history on this machine — run devto_post.py --history or consider publishing."
        print(json.dumps({"systemMessage": msg}))
        return

    history = json.loads(HISTORY_FILE.read_text())
    posts = history.get("posts", [])

    if not posts:
        msg = "dev.to: no posts yet — consider publishing your first article."
        print(json.dumps({"systemMessage": msg}))
        return

    last = posts[-1]
    last_date = datetime.fromisoformat(last["date"])
    if last_date.tzinfo is None:
        last_date = last_date.replace(tzinfo=timezone.utc)
    days_ago = (datetime.now(timezone.utc) - last_date).days

    if days_ago >= REMIND_AFTER_DAYS:
        msg = (
            f"dev.to reminder: last post was {days_ago} days ago ('{last['title']}')."
            " Today is a good day to post — best window: 12:00–18:00 UTC."
        )
        print(json.dumps({"systemMessage": msg}))

if __name__ == "__main__":
    main()
