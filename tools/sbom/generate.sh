#!/usr/bin/env bash
set -euo pipefail

# Regenerates the MaskOps Software Bill of Materials (CycloneDX).
# Anyone can run this to verify the published SBOMs match the source tree.
#
# Prerequisites:
#   cargo install cargo-cyclonedx
#   pip install cyclonedx-bom
#
# Usage:  bash tools/sbom/generate.sh

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
OUT="$ROOT/docs/security"
mkdir -p "$OUT"

echo "[1/2] Rust crate SBOM (compiled cdylib supply chain)..."
( cd "$ROOT" && cargo cyclonedx --format json --all-features )
mv "$ROOT/maskops.cdx.json" "$OUT/maskops.cdx.json"

echo "[2/2] Python runtime SBOM (declared runtime dependencies)..."
REQS="$(mktemp)"
printf 'polars>=0.46\npyyaml>=6.0\ntomli>=2.0; python_version < "3.11"\n' > "$REQS"
python -m cyclonedx_py requirements "$REQS" --of JSON --sv 1.5 --output-reproducible -o "$OUT/maskops-python.cdx.json"
rm -f "$REQS"

echo "Done. SBOMs written to docs/security/"
