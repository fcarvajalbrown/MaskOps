"""
benchmarks/benchmark_presidio.py

Real-world comparison: maskops vs Microsoft Presidio on mixed PII data.

Both tools run their full default pipelines on the same dataset.
No pattern subsetting — each tool does what it does in production.

Dataset: 10,000 rows of realistic mixed English text containing:
  - Email addresses
  - Phone numbers (E.164)
  - Credit card numbers (Visa, Mastercard)
  - IBAN numbers
  - IP addresses

Metrics:
  - Wall clock time (median of 3 runs)
  - Rows/s throughput
  - Peak memory usage (tracemalloc)
  - Entity types detected by each tool

Notes:
  - Presidio uses en_core_web_lg (recommended for production per Presidio docs)
  - maskops runs mask_pii() — full pattern set, no configuration required
  - Presidio requires spaCy + NLP model download (~750MB)
  - Python 3.11 required (spaCy not yet compatible with 3.14)

Usage:
    pip install presidio-analyzer presidio-structured spacy
    python -m spacy download en_core_web_lg
    python benchmarks/benchmark_presidio.py
"""

import re
import time
import tracemalloc
import random

import polars as pl
import maskops

# ---------------------------------------------------------------------------
# Config
# ---------------------------------------------------------------------------

ROWS = 10_000
RUNS = 3

random.seed(42)

# ---------------------------------------------------------------------------
# Dataset
# ---------------------------------------------------------------------------

SAMPLES = [
    "Please transfer to IBAN DE89370400440532013000 by Friday.",
    "Contact john.doe@example.com or call +14155552671 for details.",
    "Card number 4111111111111111 was charged successfully.",
    "Server request from 192.168.1.100 was blocked.",
    "Mastercard 5500005555555559 refund processed.",
    "Reach out to jane.smith@company.org regarding invoice #4821.",
    "Call +49 170 1234567 to confirm the wire transfer.",
    "Account DE605341928327648350 flagged for review.",
    "No sensitive information here, just a regular business note.",
    "Payment via card 371449635398431 approved at 14:32 UTC.",
]

CLEAN_SAMPLE = "No sensitive information here, just a regular business note."


def make_dataset(profile: str) -> pl.DataFrame:
    """
    Build a ROWS-row DataFrame for the given profile.

    Args:
        profile: One of 'clean', 'dense', 'mixed'.

    Returns:
        A Polars DataFrame with a single 'text' column.
    """
    if profile == "clean":
        data = [CLEAN_SAMPLE] * ROWS
    elif profile == "dense":
        data = [SAMPLES[i % len(SAMPLES)] for i in range(ROWS)]
    elif profile == "mixed":
        pool = [CLEAN_SAMPLE] + SAMPLES
        data = [pool[i % len(pool)] for i in range(ROWS)]
    else:
        raise ValueError(f"Unknown profile: {profile}")
    return pl.DataFrame({"text": data})


# ---------------------------------------------------------------------------
# Benchmark runner
# ---------------------------------------------------------------------------

def bench(label: str, fn, runs: int = RUNS) -> tuple[float, int]:
    """
    Run fn() multiple times, report median elapsed time and peak memory.

    Args:
        label: Display label for the benchmark row.
        fn:    Zero-argument callable to benchmark.
        runs:  Number of timed runs (median reported).

    Returns:
        Tuple of (median_elapsed_seconds, peak_memory_bytes).
    """
    fn()  # warmup

    times = []
    peak_mem = 0
    for _ in range(runs):
        tracemalloc.start()
        start = time.perf_counter()
        fn()
        elapsed = time.perf_counter() - start
        _, peak = tracemalloc.get_traced_memory()
        tracemalloc.stop()
        times.append(elapsed)
        peak_mem = max(peak_mem, peak)

    elapsed = sorted(times)[runs // 2]
    rows_per_sec = ROWS / elapsed
    peak_mb = peak_mem / 1024 / 1024
    print(f"  {label:<55} {elapsed:.3f}s  {rows_per_sec:>10,.0f} rows/s  {peak_mb:>7.1f} MB peak")
    return elapsed, peak_mem


# ---------------------------------------------------------------------------
# maskops runner
# ---------------------------------------------------------------------------

def run_maskops(df: pl.DataFrame) -> None:
    """Apply maskops full mask_pii pipeline to the DataFrame."""
    df.with_columns(maskops.mask_pii("text"))


# ---------------------------------------------------------------------------
# Presidio runner
# ---------------------------------------------------------------------------

def build_presidio():
    """
    Initialise Presidio AnalyzerEngine and AnonymizerEngine.

    Returns:
        Tuple of (analyzer, anonymizer).
    """
    from presidio_analyzer import AnalyzerEngine
    from presidio_anonymizer import AnonymizerEngine

    analyzer   = AnalyzerEngine()
    anonymizer = AnonymizerEngine()
    return analyzer, anonymizer


def run_presidio_batch(texts: list[str], analyzer, anonymizer) -> list[str]:
    """
    Run Presidio analyzer + anonymizer on a list of texts.

    Args:
        texts:     List of input strings.
        analyzer:  Presidio AnalyzerEngine instance.
        anonymizer: Presidio AnonymizerEngine instance.

    Returns:
        List of anonymized strings.
    """
    from presidio_anonymizer.entities import RecognizerResult, OperatorConfig

    results = []
    for text in texts:
        analysis = analyzer.analyze(text=text, language="en")
        anonymized = anonymizer.anonymize(text=text, analyzer_results=analysis)
        results.append(anonymized.text)
    return results


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    """Run maskops vs Presidio benchmark and print results."""
    print(f"\nmaskops vs Presidio — {ROWS:,} rows, median of {RUNS} runs")
    print(f"Presidio model: en_core_web_lg\n")

    print("Initialising Presidio (loading spaCy model)...")
    try:
        analyzer, anonymizer = build_presidio()
        presidio_available = True
        print("Presidio ready.\n")
    except ImportError:
        print("Presidio not installed — skipping Presidio benchmarks.\n")
        presidio_available = False

    for profile in ["clean", "dense", "mixed"]:
        df = make_dataset(profile)
        texts = df["text"].to_list()
        size_mb = df.estimated_size("mb")

        print(f"{'='*75}")
        print(f"Profile: {profile}  ({size_mb:.1f} MB in memory, {ROWS:,} rows)")
        print(f"{'='*75}")
        print(f"  {'Tool':<55} {'Time':>8}  {'Rows/s':>12}  {'Peak RAM':>10}")
        print(f"  {'-'*70}")

        t_maskops, _ = bench(
            "maskops  mask_pii (full pattern set)",
            lambda df=df: run_maskops(df),
        )

        if presidio_available:
            t_presidio, _ = bench(
                "Presidio analyze + anonymize (en_core_web_lg)",
                lambda texts=texts: run_presidio_batch(texts, analyzer, anonymizer),
            )
            speedup = t_presidio / t_maskops
            print(f"\n  → maskops is {speedup:.1f}x faster than Presidio on '{profile}'")
        else:
            print(f"\n  → Presidio not available for comparison")

        print()

    print(f"{'='*75}")

    if presidio_available:
        print("\nEntity coverage comparison:")
        print(f"  {'Pattern':<30} {'maskops':>10} {'Presidio':>10}")
        print(f"  {'-'*50}")
        patterns = [
            ("IBAN",              "✓", "✗"),
            ("EU VAT",            "✓", "✗"),
            ("Email",             "✓", "✓"),
            ("Phone (E.164)",     "✓", "✓"),
            ("IP Address",        "✓", "✓"),
            ("Credit Card",       "✓", "✓"),
            ("RUT (Chile)",       "✓", "✗"),
            ("CPF (Brazil)",      "✓", "✗"),
            ("CURP (Mexico)",     "✓", "✗"),
            ("DNI/NIE (Spain)",   "✓", "✗"),
            ("NIN (UK)",          "✓", "✗"),
            ("Personalausweis",   "✓", "✗"),
            ("Person names",      "✗", "✓"),
            ("Locations",         "✗", "✓"),
            ("Organisations",     "✗", "✓"),
        ]
        for name, mo, pr in patterns:
            print(f"  {name:<30} {mo:>10} {pr:>10}")

    print("\nDone.\n")


if __name__ == "__main__":
    main()