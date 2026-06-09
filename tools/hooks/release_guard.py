import sys
import json
import re

GUARDS = [
    (re.compile(r"^\s*git\s+push\b.*(--force-with-lease|--force|(?<!\w)-f(?!\w))"),
     "Force push blocked — tell me first and wait for explicit confirmation."),
    (re.compile(r"^\s*git\s+push\b.*(--tags|origin\s+v)"),
     "Pushing tags triggers the PyPI publish workflow — production action. Confirm the exact tag and that a roadmap stage actually shipped."),
    (re.compile(r"^\s*git\s+tag\s+(-a|-s|v)"),
     "Creating a tag is a production action — v* tags trigger the PyPI publish workflow. Confirm the exact tag explicitly first."),
    (re.compile(r"^\s*gh\s+release\s+create\b"),
     "Creating a GitHub Release is a production release action. It must be a deliberate roadmap release — confirm explicitly first."),
]

PR_CREATE = re.compile(r"^\s*gh\s+pr\s+create\b")
PR_REMINDER = (
    "PR opened. ONLY if this PR ships a roadmap feature/fix: after merge, confirm the "
    "changelog, roadmap, and all five version files are bumped, then run "
    "python tools/release/release.py X.Y.Z --yes. For chore/docs/ci/tooling PRs, do "
    "nothing release-related — no tag, no release."
)


def main():
    try:
        data = json.load(sys.stdin)
    except Exception:
        return
    command = data.get("tool_input", {}).get("command", "") or ""
    lines = command.splitlines()
    first_line = lines[0] if lines else ""
    for pattern, reason in GUARDS:
        if pattern.search(first_line):
            print(json.dumps({"continue": False, "stopReason": reason}))
            return
    if PR_CREATE.search(first_line):
        print(json.dumps({"systemMessage": PR_REMINDER}))
        return


if __name__ == "__main__":
    main()
