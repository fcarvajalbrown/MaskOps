# MaskOps vs Presidio benchmark

Reproducible throughput comparison of MaskOps and Microsoft Presidio on PII masking. Numbers are machine-relative; reproduce them yourself with the commands below.

## Headline

On **like-for-like** masking of the same structured PII (email, phone, IP, IBAN, credit card), MaskOps is roughly **two to three orders of magnitude faster** than Presidio. On a commodity Windows desktop:

| Mode | MaskOps @ 1M rows | Presidio @ 1M rows | Speedup |
|---|---|---|---|
| **Like-for-like** (same 5 entities, regex vs regex) | **1.73s** (578,830 rows/s) | 10.0 min (1,665 rows/s) | **~348x** |
| Full pipeline (Presidio NER on, scope-different) | 10.28s (97,283 rows/s) | 2.11h (132 rows/s) | ~737x |

The like-for-like number is the honest speed claim. The full-pipeline number is **scope-different and should not be read as pure speed** (see Caveats).

## Why MaskOps is faster (structural, not a tuning artifact)

- **Vectorized vs per-row.** MaskOps runs one compiled-Rust regex pass over an entire Polars column with zero per-row Python overhead. Presidio loops row by row in the Python interpreter, building result objects per text. This architectural gap is permanent; a faster CPU speeds up both and the ratio roughly holds.
- **Byte short-circuit.** MaskOps skips rows that cannot contain a given pattern before running the full regex.

## Caveats (read before quoting any number)

1. **Scope, not just speed.** MaskOps detects structured PII (regex + check-digit validation). Presidio additionally detects PERSON / LOCATION / ORGANIZATION via an ML model, which MaskOps does not attempt. "Faster" means faster at the subset of work both tools do. MaskOps does not replace Presidio's entity detection.
2. **Speed is not accuracy.** This benchmark measures throughput only. It does not measure detection recall or precision.
3. **Data-dependent.** The ratio shifts with text length and PII density. Short records widen the gap; long documents narrow it. The data here is templated, not real customer text.
4. **Like-for-like Presidio config.** For the like-for-like mode, Presidio's five matching regex recognizers (EmailRecognizer, PhoneRecognizer, IpRecognizer, IbanRecognizer, CreditCardRecognizer) are run directly with the spaCy NER disabled, because those structured recognizers do not use it. This gives Presidio its best case (no ML tax) and isolates regex-engine speed.
5. **Single machine, one run structure.** MaskOps times are the median of 3 runs. The full-pipeline Presidio number is projected linearly from a 1,500-row sample; the like-for-like Presidio number is a measured full 1M-row pass.

## Methodology

- **Data:** 1,000,000 rows, 98.7% unique, generated from realistic Faker templates embedding emails, phones, credit cards, IBANs, IPs, names, and cities. Both tools read the identical Parquet file, so they process the exact same rows.
- **Like-for-like entities:** email, phone, ip, iban, credit_card.
- **MaskOps:** `mask_pii(patterns=[...])` (like-for-like) or `mask_pii()` (full).
- **Presidio:** like-for-like = the five regex recognizers, NER off; full = `AnalyzerEngine` + `AnonymizerEngine` with `en_core_web_lg` and default recognizers.
- **Environment:** MaskOps built for CPython 3.14; Presidio on CPython 3.12 (its spaCy stack does not support 3.14). Both use polars 1.38.1. They run in separate virtualenvs but on the identical dataset, so the two timings measure the same workload.

## Reproduce

```bash
python benchmarks/gen_benchmark_data.py --rows 1000000 --pool 100000 --out target/bench_data.parquet

python benchmarks/benchmark_1m.py --tool maskops  --mode lfl  --data target/bench_data.parquet
python benchmarks/benchmark_1m.py --tool presidio --mode lfl  --data target/bench_data.parquet --presidio-full-rows
python benchmarks/benchmark_1m.py --tool maskops  --mode full --data target/bench_data.parquet
python benchmarks/benchmark_1m.py --tool presidio --mode full --data target/bench_data.parquet
```

Presidio requires `pip install presidio-analyzer presidio-anonymizer spacy click` and `python -m spacy download en_core_web_lg`.
