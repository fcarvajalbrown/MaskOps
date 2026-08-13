# Text-based PII (names, addresses): feasibility study

**Date:** 2026-08-13
**Status:** Study complete, no implementation. Informs Phase 4 Horizon roadmap item "Optional NER layer (free-text names/locations)".
**Version at time of study:** v2.1.0

## Question

Can MaskOps add detection of names, addresses and other text-based PII while still beating Presidio on throughput, accepting a much smaller margin (2x rather than the current ~350x) if necessary?

## Verdict

Speed is a non-issue and the margin does not need to shrink. A gazetteer layer costs **+2.4% of pipeline throughput** and lands at roughly **690x** Presidio's full pipeline. The 2x concession buys nothing.

The binding constraint is **accuracy**. An untuned gazetteer measures **precision 0.806, recall 0.892** on a favourable synthetic benchmark. Both figures are ceilings, not estimates. At that recall the layer cannot be described as anonymization under GDPR or Ley 21.719.

The engineering project hiding behind "add names to MaskOps" is not the matcher. It is a frequency-derived common-word blocklist per language, which is what actually bounds precision.

## Part 1 — Speed, measured

1,000,000 rows, 76.6 MiB corpus, generated from the same Faker templates as `benchmarks/gen_benchmark_data.py` (which already embed `{name}` and `{city}`). Commodity Windows desktop, release build with `lto = "fat"`, `codegen-units = 1`.

| Stage | 1M rows | rows/s |
|---|---|---|
| Current `mask_pii()` full | 10.676s | 93,667 |
| Gazetteer pass, 675K patterns, scan + mask | 0.260s | 3,839,737 |
| Combined | 10.936s | 91,442 |

The gazetteer layer runs 41x faster than the pipeline it would attach to. Marginal cost is 2.4%.

Against the 132 rows/s Presidio full-pipeline figure recorded in `benchmarks/RESULTS.md`, the combined pipeline is approximately **690x** faster. That Presidio number was carried over from the existing benchmark and was not re-measured for this study; the conclusion is robust to an order of magnitude of error in it.

Harness validity check: the `mask_pii()` full run of 10.676s tracks the 10.28s recorded in `RESULTS.md`, so the comparison base is sound. The like-for-like path measured 2.405s (415,821 rows/s) against a published 1.73s, but that path is not under test here.

### Automaton choice

Measured at 675K patterns, leftmost-longest, over the same corpus:

| | aho-corasick 1.1 | daachorse 1.0.1 |
|---|---|---|
| Scan | 0.383s (2,609,176 rows/s) | 0.217s (4,610,347 rows/s) |
| Build | 2.078s | 2.058s |
| Memory | 66.1 MiB | 67.5 MiB |

`daachorse` is 1.77x faster and uses 2% more memory.

This **does not reproduce the published claims**. Kanda et al. report 3.0-5.1x faster and 45-55% smaller memory against the `aho-corasick` crate at 675K patterns. The speedup here is under half that, and the memory claim inverts. The likely cause is workload shape: Latin-script leftmost-longest matching over short lines (~77 bytes) compresses the gap relative to the paper's dictionary benchmark, and per-call overhead dominates.

Practical consequence: `aho-corasick` is **already in `Cargo.lock`** as a transitive dependency of `regex`. Using it costs zero new dependencies at roughly +4.3% pipeline time instead of +2.4%. The entire benefit of adopting `daachorse` is about 1.9% of total runtime, against a new direct dependency and its supply-chain review.

Build cost at 675K patterns is ~2s and would be paid once per process, not per column or per batch.

## Part 2 — Accuracy, measured

Eval set: 40,000 rows, 74,530 gold name/city tokens. 20,000 rows carrying Faker-generated names and cities with exact ground-truth character spans, plus 20,000 distractor rows containing no person names or cities but including capitalized non-names, English surname-homographs (`Price`, `Rose`, `Hall`, `Cook`, `Baker`, `Young`, `Long`), and Spanish equivalents (`Cruz`, `Flores`, `Santos`, `Rio`).

Gazetteer built from `en_US`, `es_CL`, `pt_BR`, `es_MX`, `es_ES` under seed 7; eval set drawn under seed 99, so coverage gaps are present rather than assumed away. Token-level scoring: a predicted token is a true positive if it falls inside a gold span.

| Strategy | Precision | Recall | F1 | FP |
|---|---|---|---|---|
| Case-insensitive | 0.756 | 0.920 | 0.830 | 22,172 |
| Capitalized only | 0.798 | 0.893 | 0.843 | 16,868 |
| **+ generic function words only (untuned)** | **0.806** | **0.892** | **0.847** | 15,952 |
| + hand-written stoplist (tuned to test set) | 0.984 | 0.883 | 0.931 | 1,072 |
| + sentence-initial gate | 0.984 | 0.865 | 0.921 | 1,072 |

**The 0.984 precision row is not a valid result.** That gain came from content words placed in the blocklist because they were known to appear in the distractor sentences. Restricting the blocklist to genuine function words (`the`, `of`, `de`, `la`, `por`) returns precision to 0.806. The tuned row is retained here only to document the size of the leakage: it is the difference between a shippable-looking number and the real one.

Secondary findings:

- **The sentence-initial gate is dead weight.** It costs 1.8 points of recall and buys zero precision. Do not build it.
- **Case-sensitivity is worth having but minor**, 0.756 to 0.798.
- **Precision, not recall, is the failure mode.** Recall barely moves across all five strategies (0.865-0.920); precision moves 0.756-0.806 untuned. Gating cannot fix what the dictionary gets wrong.

### Why both numbers are generous

Recall is optimistic because Faker draws from a small closed name pool, so the seed-99 eval set overlaps the seed-7 gazetteer far more than real long-tail surnames would. Precision is optimistic because the distractor set is 20 sentence templates repeated 20,000 times, a much narrower collision surface than real prose. Treat 0.806 / 0.892 as an upper bound.

## Part 3 — Literature survey

24 searches, August 2026. Summary of what is and is not relevant.

**The existing regex core is already near state of the art and the literature does not offer an easy win there.** The Rust `regex` crate beats RE2 on most of `rebar`'s benchmarks and already ports Teddy out of Hyperscan. The genuinely faster engines in the 2023-2026 literature are multi-literal DPI engines solving a different problem:

- **Harry** (INFOCOM 2023) — AVX-512 column-vector multi-literal matching, 30-70 Gbit/s, 2.09x FDR, 52x Aho-Corasick.
- **Hyperflex** (arXiv 2512.07123, Dec 2025) — SIMD-based DFA, 8.89 Gbit/s, 2.27x over Hyperscan's default McClellan DFA; deployed into Hyperscan.
- **Teddy** (ICPP 2021) — SIMD literal matching, up to 35x naive Aho-Corasick, 2.16x FDR. Already present in the Rust regex crate as a prefilter.

These matter for the gazetteer layer, which is a multi-literal problem, not for the 36 structured check-digit-validated patterns in `mask_all`.

**RE#** (POPL 2025, arXiv 2407.20479) reports 71% faster than the Rust regex crate on baseline benchmarks via symbolic derivatives, with intersection, complement and restricted lookarounds. Not actionable: it is .NET, and its advantage derives from extensions MaskOps' patterns do not use.

**Counting-set automata** (POPL 2020) and **cFRA** (TRETS 2025, 97.5% state compression) solve bounded-repetition state explosion. Not a MaskOps problem.

**The vectorization ceiling is real.** Columnar-engine literature is consistent that regex evaluation forces SIMD lane divergence because each row takes a different code path. The only two levers are within-row SIMD (Teddy-style) and across-row threading (`rayon`, already roadmap v2.5). There is no third.

### Competitive landscape has shifted

Multilingual name detection is no longer the weak spot of ML tools, which invalidates any positioning built on out-recalling them on Spanish or Portuguese names:

- **GLiNER2-PII** (arXiv 2605.09973, May 2026) — 0.3B params, 42 PII entity types across seven categories, explicitly designed for locale-dependent context-sensitive PII, highest span-level F1 of five systems on the SPY benchmark.
- **Piiranha** — mdeberta-v3-base, 98.27% PII token catch rate, 6 languages including Spanish and Portuguese.
- **GLiNER Guard** (arXiv 2605.05277, 2026) — 145-147M bi-encoder, 193.6 req/s under ONNX TensorRT; bi-encoder loses only 5.2% throughput scaling from 1 to 1024 labels where uni-encoders lose 98.7%.
- **pii-vault** — Presidio-compatible PII detection and reversible tokenization with a Rust implementation, MIT licensed.
- **PII Engineer** now publishes head-to-head accuracy and latency benchmarks across 9 languages.

A 0.892-recall gazetteer is not competitive with these on detection quality. It competes on being roughly 690x faster, deterministic, and fully air-gappable.

## Part 4 — Architectural constraints

**The byte short-circuit does not extend to names.** `has_pii_candidate` (`src/patterns/mod.rs:203`) skips any row containing neither a digit nor `@`, and that is what keeps `mask_all` fast on PII-free rows. Every row of natural prose contains capitalized words, so a name layer has no equivalent cheap escape and fires on 100% of rows. The roadmap's existing instruction that the NER layer stay "outside the deterministic fast core, never bundled into the core masking path" (`ROADMAP.md:81`) is correct, and this is the concrete reason.

**A separate, larger opportunity exists in the core.** `mask_all` (`src/patterns/mod.rs:284`) chains 36 sequential regex passes, each allocating a fresh `String`. Hyperscan's Chimera architecture is the alternative: a fast literal prefilter over the haystack, invoking the expensive engine only on candidates. Applying that shape to `mask_all` would speed up what MaskOps already ships, independent of any text-PII work, and requires no dependency beyond the `aho-corasick` already in the lock file. Given the full pipeline is 10.7s against the gazetteer layer's 0.26s, this is where the time actually is.

## Part 5 — Compliance framing

Over-masking destroys utility but leaks nothing. Under-masking leaks PII. At 0.892 recall roughly 11% of name tokens survive.

A gazetteer name layer therefore **cannot be sold or documented as anonymization**. It is defensible as a pre-filter, or as defense in depth alongside a schema-driven structured pass. This is the same trap that `RESULTS.md` caveat 1 currently protects against by disclaiming NER scope entirely.

This does not conflict with the existing hard rules in `CLAUDE.md`: the layer would be asterisk-only and irreversible, requires no network access, and adds no key material.

## Part 6 — Consequences for existing claims

Shipping this would require rewriting statements that currently direct users to Presidio for names:

- `README.md:170` — "Unstructured free text, need NER for names, places, organisations -> Presidio"
- `README.md:315-317` — Person names / Locations / Organisations marked as not supported
- `README.md:319` — "Presidio's strength is unstructured text with named entities"
- `benchmarks/RESULTS.md` caveat 1 — currently disclaims NER scope, which is what keeps the ~348x like-for-like headline honest

A third benchmark mode would be needed so the text-inclusive number is reported separately rather than folded into the existing headline.

## Part 7 — What would have to be true to build this

1. A genuine frequency-derived common-word blocklist for English, Spanish and Portuguese lifts untuned precision from 0.806 to something shippable. **This is the open question that decides the feature.** It was not tested.
2. The layer is opt-in and outside `mask_all`, so no existing user pays the 2.4%.
3. It is documented as a pre-filter, never as anonymization.
4. Addresses are scoped separately from names. Spanish and Portuguese street lines are templated (`Av.`, `Avenida`, `Calle`, `Pasaje`, `Rua`, `Rodovia` followed by titlecase and a number) and are a high-precision regex problem with none of the gazetteer's collision issues. Addresses may be worth shipping independently of PERSON detection.

## Methodology and caveats

- Corpus and gazetteers generated with Faker 40.8.0 under fixed seeds; corpus templates copied from `benchmarks/gen_benchmark_data.py`.
- Throughput measured in a standalone Rust binary reading a plain-text corpus, release profile matching the crate's own. Word-boundary checking on match offsets included in the timed section.
- Gazetteers at 50K, 300K and 675K patterns were built from ~26,000 real Faker tokens padded with synthetic entries to reach scale. Padding contributes almost no matches (1,258,034 hits at 50K rising only to 1,342,152 at 675K), confirming real tokens drive matching. Padding affects automaton size only, not match quality.
- Accuracy measured in Python at token level; language-independent, so the Rust implementation would not change it.
- Presidio was not re-measured. Its 132 rows/s figure is carried from `benchmarks/RESULTS.md`.
- Precision and recall were measured on synthetic templated text, not real customer data. No real PII was used at any point.
- The benchmark and eval harnesses were written as throwaway scratchpad code and are not in the repository. The methodology above is sufficient to reproduce them.

## Sources

- [Hyperflex: A SIMD-based DFA Model for Deep Packet Inspection](https://arxiv.org/abs/2512.07123)
- [Harry: A Scalable SIMD-based Multi-literal Pattern Matching Engine](https://ieeexplore.ieee.org/document/10229022/)
- [Teddy: An Efficient SIMD-based Literal Matching Engine](https://dl.acm.org/doi/abs/10.1145/3472456.3473512)
- [Hyperscan: A Fast Multi-pattern Regex Matcher for Modern CPUs (NSDI 19)](https://www.usenix.org/system/files/nsdi19-wang-xiang.pdf)
- [Engineering faster double-array Aho-Corasick automata](https://onlinelibrary.wiley.com/doi/10.1002/spe.3190?af=R)
- [RE#: High Performance Derivative-Based Regex Matching (POPL 2025)](https://dl.acm.org/doi/10.1145/3704837)
- [Regex matching with counting-set automata (POPL 2020)](https://dl.acm.org/doi/10.1145/3428286)
- [On-Demand Regular Expression Matching on FPGAs (TRETS 2025)](https://dl.acm.org/doi/10.1145/3774651)
- [rebar regex benchmark](https://github.com/BurntSushi/rebar)
- [GLiNER2-PII: A Multilingual Model for PII Extraction](https://arxiv.org/abs/2605.09973)
- [GLiNER Guard: Unified Encoder Family for Production LLM Safety and Privacy](https://arxiv.org/abs/2605.05277)
- [Best Open Source Models for PII Redaction (Piiranha)](https://grepture.com/blog/best-open-source-models-pii-redaction)
- [pii-vault](https://github.com/jiansen/pii-vault)
- [PII Engineer benchmarks](https://pii.engineer/benchmarks)
- [Chimera, Hyperscan documentation](https://intel.github.io/hyperscan/dev-reference/chimera.html)
