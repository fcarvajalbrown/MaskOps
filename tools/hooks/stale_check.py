#!/usr/bin/env python3
import json
import re
import sys
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
STALE_DRAFT_DAYS = 7


def read(path):
    try:
        return (ROOT / path).read_text(encoding="utf-8")
    except OSError:
        return ""


def first(pattern, text):
    m = re.search(pattern, text, re.MULTILINE)
    return m.group(1) if m else None


def version_sources():
    sources = {}
    sources["Cargo.toml"] = first(r'^version\s*=\s*"([^"]+)"', read("Cargo.toml"))
    sources["pyproject.toml"] = first(r'^version\s*=\s*"([^"]+)"', read("pyproject.toml"))
    manifest = read(".release-please-manifest.json")
    try:
        sources[".release-please-manifest.json"] = json.loads(manifest).get(".")
    except json.JSONDecodeError:
        sources[".release-please-manifest.json"] = None
    lock = read("Cargo.lock")
    sources["Cargo.lock"] = first(
        r'name\s*=\s*"maskops"\s*\nversion\s*=\s*"([^"]+)"', lock
    )
    sources["src/lib.rs"] = first(r'__version__"\s*,\s*"([^"]+)"', read("src/lib.rs"))
    return sources


def version_drift():
    sources = version_sources()
    present = {k: v for k, v in sources.items() if v}
    if len(set(present.values())) <= 1:
        return None
    detail = ", ".join(f"{k}={v}" for k, v in sources.items())
    return f"Version files disagree: {detail}. Reconcile to the shipped roadmap milestone before any release."


def stale_drafts(now):
    cutoff = STALE_DRAFT_DAYS * 86400
    stale = []
    for draft in (ROOT / "tools" / "social").glob("draft_*.md"):
        age = now - draft.stat().st_mtime
        if age > cutoff:
            stale.append((draft.name, int(age // 86400)))
    if not stale:
        return None
    listed = ", ".join(f"{name} ({days}d)" for name, days in sorted(stale))
    return f"Stale scratch drafts in tools/social: {listed}. Archive and publish, or delete."


def main():
    try:
        json.load(sys.stdin)
    except (json.JSONDecodeError, ValueError):
        pass
    now = datetime.now(timezone.utc).timestamp()
    warnings = [w for w in (version_drift(), stale_drafts(now)) if w]
    if warnings:
        print(json.dumps({"systemMessage": " | ".join(warnings)}))


if __name__ == "__main__":
    main()
