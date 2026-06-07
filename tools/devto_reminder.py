#!/usr/bin/env python3
import json
import sys
from datetime import datetime, timezone
from pathlib import Path

HISTORY_FILE = Path(__file__).parent / "devto_history.json"
REMIND_AFTER_DAYS = 21

def main():
    if not HISTORY_FILE.exists():
        msg = "dev.to: no posts yet — consider publishing your first article."
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
    days_ago = (datetime.now(timezone.utc) - last_date).days

    if days_ago >= REMIND_AFTER_DAYS:
        msg = f"dev.to reminder: last post was {days_ago} days ago ('{last['title']}'). Good time for a new article."
        print(json.dumps({"systemMessage": msg}))

if __name__ == "__main__":
    main()
