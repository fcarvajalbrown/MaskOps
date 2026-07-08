import json
import re
import sys

STRING_LITERAL = re.compile(r'"(?:\\.|[^"\\])*"')
CHAR_LITERAL = re.compile(r"'(?:\\.|[^'\\])*'")
COMMENT_TOKEN = re.compile(r"//|/\*|\*/")


def added_text(tool_name, tool_input):
    if tool_name == "Write":
        return tool_input.get("content", "")
    if tool_name == "Edit":
        return tool_input.get("new_string", "")
    if tool_name == "MultiEdit":
        return "\n".join(e.get("new_string", "") for e in tool_input.get("edits", []))
    return ""


def offending_lines(text):
    hits = []
    for i, line in enumerate(text.splitlines(), 1):
        stripped = CHAR_LITERAL.sub("''", STRING_LITERAL.sub('""', line))
        if COMMENT_TOKEN.search(stripped):
            hits.append((i, line.strip()))
    return hits


def main():
    try:
        payload = json.load(sys.stdin)
    except (json.JSONDecodeError, ValueError):
        sys.exit(0)

    tool_input = payload.get("tool_input", {})
    path = tool_input.get("file_path", "")
    if not path.endswith(".rs"):
        sys.exit(0)

    hits = offending_lines(added_text(payload.get("tool_name", ""), tool_input))
    if not hits:
        sys.exit(0)

    preview = "; ".join(f"L{n}: {t}" for n, t in hits[:5])
    print(
        f"Blocked: this edit adds Rust comments to {path}. "
        f"MaskOps forbids all comments in .rs files (names and types are the only documentation). "
        f"Remove them and re-apply. Offending lines: {preview}",
        file=sys.stderr,
    )
    sys.exit(2)


main()
