"""
benchmarks/benchmark.py

Measures maskops throughput on 1M rows, broken down by pattern family:

  - EU:        IBAN, VAT, Email, Phone
  - LatAm:     RUT (Chile), CPF (Brazil), CURP (Mexico)
  - Network:   IP address
  - Card:      Visa, Mastercard, Amex, Discover, Maestro
  - EU ID:     DNI/NIE (Spain), NIN (UK), Personalausweis (Germany)
  - All:       every pattern active (worst case)

Each family is tested across three data profiles:
  - clean:  no PII (baseline, no masking work done)
  - dense:  every row contains PII from that family
  - mixed:  50/50 mix (realistic production workload)

Timing: median of 3 runs per benchmark to reduce noise.
Baseline: pure Python regex per family, equivalent pattern coverage to maskops.

Usage:
    python benchmarks/benchmark.py
"""

import re
import time

import polars as pl
import maskops

# ---------------------------------------------------------------------------
# Config
# ---------------------------------------------------------------------------

ROWS = 1_000_000
RUNS = 3
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

CARD_SAMPLES = [
    "Payment charged to card 4111111111111111 approved",
    "Refund issued to 371449635398431",
    "Card 5500005555555559 declined",
    "Maestro 6304000000000000 accepted",
]

EU_ID_SAMPLES = [
    "DNI del cliente: 12345678Z registrado",
    "NIE registrado: X1234567L",
    "NIN on file: AB 12 34 56 C",
    "Ausweis-Nr: T220001293",
]

ALL_SAMPLES = EU_SAMPLES + LATAM_SAMPLES + NETWORK_SAMPLES + CARD_SAMPLES + EU_ID_SAMPLES

# ---------------------------------------------------------------------------
# Pure Python regex baselines
# Each baseline covers the same patterns as maskops for that family.
# ---------------------------------------------------------------------------

EU_RE = re.compile(
    # IBAN
    r"\b([A-Z]{2}\d{2}[A-Z0-9]{4}[0-9]{7}([A-Z0-9]?){0,16})\b"
    # VAT (major EU countries)
    r"|ATU[0-9]{8}|BE[01][0-9]{9}|BG[0-9]{9,10}|DE[0-9]{9}"
    r"|ES[A-Z0-9][0-9]{7}[A-Z0-9]|FR[A-Z0-9]{2}[0-9]{9}"
    r"|IT[0-9]{11}|NL[0-9]{9}B[0-9]{2}|PL[0-9]{10}"
    # Email
    r"|[a-zA-Z0-9._%+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,}"
    # Phone (E.164-ish)
    r"|\+\d{1,3}\d{6,12}\b"
)

LATAM_RE = re.compile(
    # RUT
    r"\b\d{1,2}\.?\d{3}\.?\d{3}-[\dKk]\b"
    # CPF
    r"|\b\d{3}\.?\d{3}\.?\d{3}-?\d{2}\b"
    # CURP
    r"|[A-Z][AEIOU][A-Z]{2}\d{6}[HM][A-Z]{2}[B-DF-HJ-NP-TV-Z]{3}[A-Z0-9]\d"
)

NETWORK_RE = re.compile(
    # IPv4
    r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b"
    # IPv6 (simplified)
    r"|[0-9a-fA-F]{1,4}(?::[0-9a-fA-F]{1,4}){7}"
)

CARD_RE = re.compile(
    # Visa/MC/Discover/Maestro 16-digit
    r"\b4[0-9]{15}\b"
    r"|\b5[1-5][0-9]{14}\b"
    r"|\b(?:6011|65[0-9]{2}|64[4-9][0-9])[0-9]{12}\b"
    r"|\b(?:6304|6759|676[1-3])[0-9]{12}\b"
    # Amex 15-digit
    r"|\b3[47][0-9]{13}\b"
)

EU_ID_RE = re.compile(
    # DNI
    r"\b\d{8}[A-HJ-NP-TV-Z]\b"
    # NIE
    r"|\b[XYZ]\d{7}[A-HJ-NP-TV-Z]\b"
    # NIN
    r"\b[A-CEGHJ-PR-TW-Z][A-CEGHJ-NPR-TW-Z]\s?\d{2}\s?\d{2}\s?\d{2}\s?[ABCD]\b"
    # Personalausweis
    r"|\b[A-Z][A-Z0-9]{8}[0-9]\b"
)

ALL_RE = re.compile(
    EU_RE.pattern + r"|" + LATAM_RE.pattern + r"|" + NETWORK_RE.pattern
    + r"|" + CARD_RE.pattern + r"|" + EU_ID_RE.pattern
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

def bench(label: str, fn, runs: int = RUNS) -> float:
    """
    Run fn() multiple times and report the median elapsed time.

    Args:
        label: Display label for the benchmark row.
        fn:    Zero-argument callable to benchmark.
        runs:  Number of timed runs (median is reported).

    Returns:
        Median elapsed time in seconds.
    """
    # warmup — triggers Arrow buffer allocation and any lazy init
    fn()

    times = []
    for _ in range(runs):
        start = time.perf_counter()
        fn()
        times.append(time.perf_counter() - start)

    elapsed = sorted(times)[runs // 2]  # median
    rows_per_sec = ROWS / elapsed
    # Use actual DataFrame size for accurate MB/s
    print(f"  {label:<45} {elapsed:.3f}s  {rows_per_sec:>12,.0f} rows/s")
    return elapsed

def python_regex_mask(df: pl.DataFrame, pattern: re.Pattern) -> pl.Series:
    """
    Pure Python regex masking baseline — equivalent coverage to maskops.

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
    ("EU (IBAN, VAT, Email, Phone)",           EU_SAMPLES,     EU_RE),
    ("LatAm (RUT, CPF, CURP)",                 LATAM_SAMPLES,  LATAM_RE),
    ("Network (IP)",                            NETWORK_SAMPLES, NETWORK_RE),
    ("Credit Card (Visa/MC/Amex/Discover/Maestro)", CARD_SAMPLES, CARD_RE),
    ("European ID (DNI/NIE/NIN/Personalausweis)",   EU_ID_SAMPLES, EU_ID_RE),
    ("All patterns",                            ALL_SAMPLES,    ALL_RE),
]

def main():
    """Run all benchmarks and print results to stdout."""
    print(f"\nmaskops benchmark — {ROWS:,} rows, median of {RUNS} runs")

    for family_name, samples, baseline_re in FAMILIES:
        print(f"\n{'='*75}")
        print(f"Family: {family_name}")
        print(f"{'='*75}")

        for profile in ["clean", "dense", "mixed"]:
            df = make_dataset(samples, profile)
            size_mb = df.estimated_size("mb")
            print(f"\n  Profile: {profile}  ({size_mb:.1f} MB in memory)")
            print(f"  {'-'*70}")

            t_mask    = bench("mask_pii (maskops)",
                              lambda df=df: df.with_columns(maskops.mask_pii("text")))
            t_contains = bench("contains_pii (maskops)",
                               lambda df=df: df.with_columns(maskops.contains_pii("text")))
            t_baseline = bench("mask_pii baseline (pure Python re)",
                               lambda df=df, r=baseline_re: python_regex_mask(df, r))

            speedup = t_baseline / t_mask
            print(f"\n  → maskops is {speedup:.1f}x faster than pure Python on '{profile}'")

    print(f"\n{'='*75}")
    print("Done.\n")

if __name__ == "__main__":
    main()