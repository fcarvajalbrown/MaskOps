import base64
import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path

EDGE_CANDIDATES = [
    r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
    r"C:\Program Files\Microsoft\Edge\Application\msedge.exe",
]


def find_edge() -> str:
    for c in EDGE_CANDIDATES:
        if Path(c).exists():
            return c
    raise SystemExit("Edge not found; set the path in EDGE_CANDIDATES.")


def embed_fonts(svg_text: str, svg_dir: Path) -> str:
    def repl(m):
        rel = m.group(1)
        data = base64.b64encode((svg_dir / rel).read_bytes()).decode()
        return f"url(data:font/otf;base64,{data})"
    return re.sub(r"url\('([^']+\.otf)'\)", repl, svg_text)


def dims(svg_text: str) -> tuple[int, int]:
    w = int(re.search(r'width="(\d+)"', svg_text).group(1))
    h = int(re.search(r'height="(\d+)"', svg_text).group(1))
    return w, h


def main():
    args = [a for a in sys.argv[1:] if a != "--scale"]
    scale = 2
    if "--scale" in sys.argv:
        scale = int(sys.argv[sys.argv.index("--scale") + 1])
        args = [a for a in args if a != str(scale)]
    svg_path = Path(args[0]).resolve()
    out_path = Path(args[1]).resolve() if len(args) > 1 else svg_path.with_suffix(".png")

    svg_text = svg_path.read_text(encoding="utf-8")
    w, h = dims(svg_text)
    inline = embed_fonts(svg_text, svg_path.parent)
    html = f"<!doctype html><meta charset=utf-8><style>html,body{{margin:0;padding:0}}</style>{inline}"

    with tempfile.NamedTemporaryFile("w", suffix=".html", delete=False, encoding="utf-8") as f:
        f.write(html)
        html_path = f.name

    try:
        subprocess.run([
            find_edge(), "--headless=new", "--disable-gpu", "--hide-scrollbars",
            f"--force-device-scale-factor={scale}",
            f"--window-size={w},{h}",
            f"--screenshot={out_path}",
            Path(html_path).as_uri(),
        ], check=True, timeout=120)
    finally:
        os.unlink(html_path)

    if out_path.exists():
        print(f"wrote {out_path} ({out_path.stat().st_size} bytes, {w*scale}x{h*scale})")
    else:
        raise SystemExit("render failed: no PNG produced")


if __name__ == "__main__":
    main()
