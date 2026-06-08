import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent

VERSION_PATTERNS = [
    ("pyproject.toml", r'(?m)^version = "(.+?)"'),
    ("Cargo.toml", r'(?m)^version = "(.+?)"'),
    (".release-please-manifest.json", r'"\.": "(.+?)"'),
    ("src/lib.rs", r'__version__", "(.+?)"'),
    ("Cargo.lock", r'name = "maskops"\nversion = "(.+?)"'),
]


def run(args, capture=False):
    return subprocess.run(args, cwd=ROOT, text=True, check=True,
                          capture_output=capture)


def fail(message):
    print(f"release: {message}")
    sys.exit(1)


def read_version(rel_path, pattern):
    text = (ROOT / rel_path).read_text(encoding="utf-8")
    match = re.search(pattern, text)
    return match.group(1) if match else None


def changelog_section(version):
    text = (ROOT / "docs" / "CHANGELOG.md").read_text(encoding="utf-8")
    pattern = re.compile(
        r"^## \[" + re.escape(version) + r"\].*?(?=^## \[|\Z)",
        re.MULTILINE | re.DOTALL,
    )
    match = pattern.search(text)
    return match.group(0).strip() if match else None


def roadmap_mentions(version):
    return version in (ROOT / "ROADMAP.md").read_text(encoding="utf-8")


def main():
    flags = sys.argv[1:]
    do_release = "--yes" in flags
    positional = [a for a in flags if a != "--yes"]
    if len(positional) != 1:
        fail("usage: python tools/release/release.py X.Y.Z [--yes]")
    version = positional[0].lstrip("v")
    tag = f"v{version}"

    branch = run(["git", "rev-parse", "--abbrev-ref", "HEAD"],
                 capture=True).stdout.strip()
    if branch != "main":
        fail(f"must be on main, currently on {branch}")

    if run(["git", "status", "--porcelain"], capture=True).stdout.strip():
        fail("working tree not clean — commit or stash first")

    mismatches = []
    for rel_path, pattern in VERSION_PATTERNS:
        found = read_version(rel_path, pattern)
        if found != version:
            mismatches.append((rel_path, found))
    if mismatches:
        for rel_path, found in mismatches:
            print(f"  {rel_path}: {found}")
        fail(f"version files do not all equal {version} (above) — "
             "bump every version file first")

    notes = changelog_section(version)
    if not notes:
        fail(f"no '## [{version}]' section in docs/CHANGELOG.md — "
             "add the changelog entry first")

    if run(["git", "tag", "-l", tag], capture=True).stdout.strip():
        fail(f"tag {tag} already exists")

    print(f"Pre-flight for {tag}:")
    print("  - on main, working tree clean")
    print(f"  - all five version files == {version}")
    print("  - changelog section present")
    print("  - roadmap mentions version: "
          + ("yes" if roadmap_mentions(version) else "NO (check ROADMAP.md)"))
    print(f"  - tag {tag} does not exist yet")

    if not do_release:
        print()
        print(f"Dry run. To publish: python tools/release/release.py {version} --yes")
        return

    run(["git", "tag", "-a", tag, "-m", tag])
    run(["git", "push", "origin", tag])
    run(["gh", "release", "create", tag, "--title", tag, "--notes", notes])
    print(f"Released {tag}. publish.yml is building wheels and uploading to PyPI.")


if __name__ == "__main__":
    main()
