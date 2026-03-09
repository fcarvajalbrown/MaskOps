"""
benchmarks/benchmark.py

Measures maskops throughput on 1M rows, broken down by pattern family:

  - EU:      IBAN, VAT, Email, Phone
  - LatAm:   RUT (Chile), CPF (Brazil), CURP (Mexico)
  - Network: IP address
  - All:     every pattern active (worst case)

Each family is tested across three data profiles:
  - clean:  no PII (baseline, no masking work done)
  - dense:  every row contains PII from that family
  - mixed:  50/50 mix (realistic production workload)

Also benchmarks pure Python regex per family as a comparison baseline.

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
CLEAN_SAMPLE = "No sensitive information here, just a regular sentence."

# ---------------------------------------------------------------------------
# PII samples per family
# ---------------------------------------------------------------------------

EU_SAMPLES = [
    "Transfer to DE89370400440532013000 confirmed",
    "VAT number DE123456789 on invoice",
    "Contact john.doe@example.com for details",
    "Call us at +14155552671 anytime",
]

LATAM_SAMPLES = [
    "Cliente RUT 76.354.771-K registrado",
    "CPF do cliente: 529.982.247-25 confirmado",
    "CURP: BADD110313HCMLNS09 registrado",
]

NETWORK_SAMPLES = [
    "Server at 192.168.1.100 is down",
    "Request from 10.0.0.254 blocked",
    "IPv6 client 2001:db8:0:0:1:2:3:4 connected",
]

ALL_SAMPLES = EU_SAMPLES + LATAM_SAMPLES + NETWORK_SAMPLES

# ---------------------------------------------------------------------------
# Pure Python regex baselines (one per family)
# ---------------------------------------------------------------------------

EU_RE = re.compile(
    r"\b([A-Z]{2}\d{2}[A-Z0-9]{4}[0-9]{7}([A-Z0-9]?){0,16})\b"
    r"|[a-zA-Z0-9._%+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,}"
    r"|\+?\d[\d\s\-().]{7,}\d"
)

LATAM_RE = re.compile(
    r"\b\d{1,2}\.?\d{3}\.?\d{3}-[\dKk]\b"
    r"|\b\d{3}\.?\d{3}\.?\d{3}-?\d{2}\b"
    r"|[A-Z][AEIOU][A-Z]{2}\d{6}[HM][A-Z]{2}[B-DF-HJ-NP-TV-Z]{3}[A-Z0-9]\d"
)

NETWORK_RE = re.compile(
    r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b"
)

ALL_RE = re.compile(
    EU_RE.pattern + r"|" + LATAM_RE.pattern + r"|" + NETWORK_RE.pattern
)

# ---------------------------------------------------------------------------
# Dataset builders
# ---------------------------------------------------------------------------

def make_dataset(samples: list, profile: str) -> pl.DataFrame:
    """
    Build a 1M row DataFrame for the given profile.

    Args:
        samples: List of PII-containing strings for this family.
        profile: One of 'clean', 'dense', 'mixed'.

    Returns:
        A Polars DataFrame with a single 'text' column.
    """
    if profile == "clean":
        data = [CLEAN_SAMPLE] * ROWS
    elif profile == "dense":
        data = [samples[i % len(samples)] for i in range(ROWS)]
    elif profile == "mixed":
        pool = [CLEAN_SAMPLE] + samples
        data = [pool[i % len(pool)] for i in range(ROWS)]
    else:
        raise ValueError(f"Unknown profile: {profile}")
    return pl.DataFrame({"text": data})

# ---------------------------------------------------------------------------
# Benchmark runner
# ---------------------------------------------------------------------------

def bench(label: str, fn, warmup: bool = True) -> float:
    """
    Run fn() and return elapsed seconds.

    Args:
        label:  Display label for the benchmark row.
        fn:     Zero-argument callable to benchmark.
        warmup: If True, runs fn() once before timing to exclude init costs.

    Returns:
        Elapsed time in seconds.
    """
    if warmup:
        fn()
    start = time.perf_counter()
    fn()
    elapsed = time.perf_counter() - start
    rows_per_sec = ROWS / elapsed
    mb_per_sec = (ROWS * 40) / elapsed / 1_000_000
    print(f"  {label:<45} {elapsed:.3f}s  {rows_per_sec:>12,.0f} rows/s  {mb_per_sec:>7.1f} MB/s")
    return elapsed

def python_regex_mask(df: pl.DataFrame, pattern: re.Pattern) -> pl.Series:
    """
    Pure Python regex masking baseline.

    Args:
        df:      DataFrame with 'text' column.
        pattern: Compiled regex to apply.

    Returns:
        Masked Polars Series.
    """
    return pl.Series([
        pattern.sub(lambda m: "*" * len(m.group()), s)
        for s in df["text"].to_list()
    ])

# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

FAMILIES = [
    ("EU (IBAN, VAT, Email, Phone)", EU_SAMPLES, EU_RE),
    ("LatAm (RUT, CPF, CURP)",       LATAM_SAMPLES, LATAM_RE),
    ("Network (IP)",                  NETWORK_SAMPLES, NETWORK_RE),
    ("All patterns",                  ALL_SAMPLES, ALL_RE),
]

def main():
    """Run all benchmarks and print results to stdout."""
    print(f"\nmaskops benchmark — {ROWS:,} rows")

    for family_name, samples, baseline_re in FAMILIES:
        print(f"\n{'='*75}")
        print(f"Family: {family_name}")
        print(f"{'='*75}")

        for profile in ["clean", "dense", "mixed"]:
            df = make_dataset(samples, profile)
            size_mb = df.estimated_size("mb")
            print(f"\n  Profile: {profile}  ({size_mb:.1f} MB in memory)")
            print(f"  {'-'*70}")
            bench("mask_pii (maskops)",
                  lambda: df.with_columns(maskops.mask_pii("text")))
            bench("contains_pii (maskops)",
                  lambda: df.with_columns(maskops.contains_pii("text")))
            bench("mask_pii baseline (pure Python re)",
                  lambda: python_regex_mask(df, baseline_re))

    print(f"\n{'='*75}")
    print("Done.\n")

if __name__ == "__main__":
    main()