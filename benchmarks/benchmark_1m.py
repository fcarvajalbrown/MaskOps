import argparse
import statistics
import time

import polars as pl

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
PRESIDIO_MODEL = "en_core_web_lg"

LFL_PATTERNS = ["email", "phone", "ip", "iban", "credit_card"]
LFL_ENTITIES = ["EMAIL_ADDRESS", "PHONE_NUMBER", "IP_ADDRESS", "IBAN_CODE", "CREDIT_CARD"]


def make_texts(profile, rows):
    if profile == "clean":
        pool = [CLEAN_SAMPLE]
    elif profile == "dense":
        pool = SAMPLES
    elif profile == "mixed":
        pool = [CLEAN_SAMPLE] + SAMPLES
    else:
        raise ValueError(f"Unknown profile: {profile}")
    return [pool[i % len(pool)] for i in range(rows)]


def time_maskops(texts, runs, mode):
    import maskops

    df = pl.DataFrame({"text": texts})
    if mode == "lfl":
        expr = maskops.mask_pii("text", patterns=LFL_PATTERNS)
    else:
        expr = maskops.mask_pii("text")
    df.with_columns(expr)
    times = []
    for _ in range(runs):
        start = time.perf_counter()
        df.with_columns(expr)
        times.append(time.perf_counter() - start)
    return statistics.median(times)


def build_presidio_lfl():
    from presidio_analyzer.predefined_recognizers import (
        EmailRecognizer,
        PhoneRecognizer,
        IpRecognizer,
        IbanRecognizer,
        CreditCardRecognizer,
    )
    from presidio_anonymizer import AnonymizerEngine

    recognizers = [
        EmailRecognizer(),
        PhoneRecognizer(),
        IpRecognizer(),
        IbanRecognizer(),
        CreditCardRecognizer(),
    ]
    return recognizers, AnonymizerEngine()


def time_presidio_lfl(texts, recognizers, anonymizer):
    start = time.perf_counter()
    for text in texts:
        results = []
        for rec in recognizers:
            results.extend(rec.analyze(text, rec.supported_entities, nlp_artifacts=None))
        anonymizer.anonymize(text=text, analyzer_results=results)
    return time.perf_counter() - start


def build_presidio_full():
    from presidio_analyzer import AnalyzerEngine
    from presidio_anonymizer import AnonymizerEngine

    return AnalyzerEngine(), AnonymizerEngine()


def time_presidio_full(texts, analyzer, anonymizer):
    start = time.perf_counter()
    for text in texts:
        analysis = analyzer.analyze(text=text, language="en")
        anonymizer.anonymize(text=text, analyzer_results=analysis)
    return time.perf_counter() - start


def fmt_seconds(s):
    if s < 90:
        return f"{s:.2f}s"
    if s < 5400:
        return f"{s/60:.1f}min"
    return f"{s/3600:.2f}h"


def main():
    parser = argparse.ArgumentParser(description="MaskOps vs Presidio at scale")
    parser.add_argument("--rows", type=int, default=1_000_000)
    parser.add_argument("--presidio-rows", type=int, default=20_000)
    parser.add_argument("--profile", choices=["clean", "dense", "mixed"], default="mixed")
    parser.add_argument("--maskops-runs", type=int, default=3)
    parser.add_argument("--mode", choices=["lfl", "full"], default="lfl")
    parser.add_argument("--presidio-full-rows", action="store_true")
    parser.add_argument("--tool", choices=["both", "maskops", "presidio"], default="both")
    parser.add_argument("--maskops-time", type=float, default=None)
    parser.add_argument("--data", default=None)
    args = parser.parse_args()

    def load_texts(n):
        if args.data:
            return pl.read_parquet(args.data)["text"].head(n).to_list()
        return make_texts(args.profile, n)

    if args.mode == "lfl":
        scope = f"like-for-like: {', '.join(LFL_PATTERNS)} (Presidio NER off)"
    else:
        scope = "full pipeline: all MaskOps patterns vs Presidio AnalyzerEngine + NER (scope-different)"

    print("=" * 74)
    print("MaskOps vs Presidio benchmark")
    print("=" * 74)
    print(f"Mode:               {args.mode}")
    print(f"Scope:              {scope}")
    print(f"Rows:               {args.rows:,}")
    print(f"Profile:            {args.profile}")
    print(f"Tool(s):            {args.tool}")
    print("=" * 74)

    mo_time = args.maskops_time
    pr_rate = None
    pr_projected = None
    measured = None

    if args.tool in ("both", "maskops"):
        texts = load_texts(args.rows)
        df_mb = pl.DataFrame({"text": texts}).estimated_size("mb")
        print(f"\nMaskOps dataset: {args.rows:,} rows, {df_mb:.1f} MB")
        print(f"Running MaskOps ({args.mode})...")
        mo_time = time_maskops(texts, args.maskops_runs, args.mode)
        print(f"  MaskOps:  {fmt_seconds(mo_time)}  ({args.rows/mo_time:,.0f} rows/s, median of {args.maskops_runs})")

    if args.tool in ("both", "presidio"):
        full_rows = args.presidio_full_rows
        n = args.rows if full_rows else args.presidio_rows
        texts = load_texts(n)
        if args.mode == "lfl":
            print(f"\nBuilding Presidio like-for-like recognizers (no spaCy NER)...")
            recognizers, anonymizer = build_presidio_lfl()
            runner = lambda: time_presidio_lfl(texts, recognizers, anonymizer)
        else:
            print(f"\nInitialising Presidio AnalyzerEngine ({PRESIDIO_MODEL}, NER on)...")
            analyzer, anonymizer = build_presidio_full()
            runner = lambda: time_presidio_full(texts, analyzer, anonymizer)
        if full_rows:
            print(f"Running Presidio on full {args.rows:,} rows...")
        else:
            print(f"Running Presidio on {args.presidio_rows:,}-row sample...")
        pr_time = runner()
        pr_rate = n / pr_time
        pr_projected = pr_time if full_rows else args.rows / pr_rate
        measured = "measured (full)" if full_rows else f"projected from {args.presidio_rows:,}-row sample"
        print(f"  Presidio: {pr_rate:,.0f} rows/s ({measured})")

    print("\n" + "=" * 74)
    print(f"Result ({args.mode})")
    print("=" * 74)
    if mo_time is not None:
        print(f"  MaskOps  @ {args.rows:,} rows:  {fmt_seconds(mo_time):>10}   ({args.rows/mo_time:,.0f} rows/s)")
    if pr_projected is not None:
        print(f"  Presidio @ {args.rows:,} rows:  {fmt_seconds(pr_projected):>10}   ({pr_rate:,.0f} rows/s, {measured})")
    if mo_time is not None and pr_projected is not None:
        print(f"\n  MaskOps is {pr_projected/mo_time:,.0f}x faster on '{args.profile}' ({args.mode}).")
    print("=" * 74)
    print(f"\nMethodology ({args.mode}):")
    if args.mode == "lfl":
        print(f"  Both tools detect+mask only: {', '.join(LFL_PATTERNS)}.")
        print(f"  MaskOps: mask_pii(patterns={LFL_PATTERNS}).")
        print(f"  Presidio: {', '.join(LFL_ENTITIES)} regex recognizers run directly,")
        print(f"  spaCy NER disabled (it is not used for these structured types).")
        print(f"  This compares regex engine to regex engine on the identical task.")
    else:
        print(f"  MaskOps runs all patterns; Presidio runs AnalyzerEngine + {PRESIDIO_MODEL}")
        print(f"  with NER. NOTE: Presidio also detects PERSON/LOCATION/ORG that MaskOps")
        print(f"  does not. This number reflects scope difference, not pure engine speed.")
    if pr_projected is not None and not args.presidio_full_rows:
        print(f"  Presidio time at {args.rows:,} is projected linearly from the sample.")
    print()


if __name__ == "__main__":
    main()
