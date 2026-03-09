"""
benchmarks/benchmark.py

Measures maskops throughput on 1M rows.
Tests mask_pii and contains_pii against three data profiles:
  - clean:    no PII (worst case for contains_pii, best for mask_pii)
  - dense:    every row contains PII
  - mixed:    50/50 mix (realistic)

Also benchmarks pure Python regex as a baseline comparison.

Usage:
    python benchmarks/benchmark.py
"""

import time
import re
import polars as pl
import maskops

# ---------------------------------------------------------------------------
# Config
# ---------------------------------------------------------------------------

ROWS = 1_000_000

IBAN_SAMPLE = "DE89370400440532013000"
VAT_SAMPLE = "DE123456789"
CLEAN_SAMPLE = "No sensitive information here, just a regular sentence."
MIXED_TEXTS = [
    f"Transfer to account {IBAN_SAMPLE} confirmed",
    CLEAN_SAMPLE,
]

# Pure Python baseline regex
IBAN_RE = re.compile(r"\b([A-Z]{2}\d{2}[A-Z0-9]{4}[0-9]{7}([A-Z0-9]?){0,16})\b")

# ---------------------------------------------------------------------------
# Dataset builders
# ---------------------------------------------------------------------------

def make_dataset(profile: str) -> pl.DataFrame:
    """Build a 1M row DataFrame for the given profile."""
    if profile == "clean":
        data = [CLEAN_SAMPLE] * ROWS
    elif profile == "dense":
        data = [f"Payment ref {IBAN_SAMPLE} from {VAT_SAMPLE}"] * ROWS
    elif profile == "mixed":
        data = [MIXED_TEXTS[i % 2] for i in range(ROWS)]
    else:
        raise ValueError(f"Unknown profile: {profile}")
    return pl.DataFrame({"text": data})

# ---------------------------------------------------------------------------
# Benchmark runner
# ---------------------------------------------------------------------------

def bench(label: str, fn, warmup: bool = True) -> float:
    """
    Run fn(), return elapsed seconds.
    Runs a warmup pass first to exclude JIT/lazy init costs.
    """
    if warmup:
        fn()  # warmup
    start = time.perf_counter()
    fn()
    elapsed = time.perf_counter() - start
    rows_per_sec = ROWS / elapsed
    mb_per_sec = (ROWS * len(IBAN_SAMPLE)) / elapsed / 1_000_000
    print(f"  {label:<45} {elapsed:.3f}s  {rows_per_sec:>12,.0f} rows/s  {mb_per_sec:>7.1f} MB/s")
    return elapsed

def python_regex_mask(df: pl.DataFrame) -> pl.Series:
    """Pure Python regex masking — used as baseline."""
    return pl.Series([
        IBAN_RE.sub(lambda m: m.group()[:4] + "*" * (len(m.group()) - 4), s)
        for s in df["text"].to_list()
    ])

# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    print(f"\nmaskops benchmark — {ROWS:,} rows\n")
    print(f"{'='*75}")

    for profile in ["clean", "dense", "mixed"]:
        df = make_dataset(profile)
        size_mb = df.estimated_size("mb")
        print(f"\nProfile: {profile}  ({size_mb:.1f} MB in memory)")
        print(f"{'-'*75}")

        bench("mask_pii (maskops)",
              lambda: df.with_columns(maskops.mask_pii("text")))

        bench("contains_pii (maskops)",
              lambda: df.with_columns(maskops.contains_pii("text")))

        bench("mask_pii baseline (pure Python re)",
              lambda: python_regex_mask(df))

    print(f"\n{'='*75}")
    print("Done.\n")

if __name__ == "__main__":
    main()