# PII masking in Polars: MaskOps 2.0, and two metrics that lied to me

> Published 2026-06-11 to dev.to — https://dev.to/fcarvajalbrown/pii-masking-in-polars-maskops-20-and-two-metrics-that-lied-to-me-2bh3
> Tags: rust, python, polars, privacy
> Cover: covers/maskops-2.0-benchmarks.png
> Description: A Rust Polars plugin for high-speed PII masking - RUT, CPF, cards, IBAN. Honest benchmarks vs pure-Python regex, air-gapped, GDPR-ready. MaskOps 2.0.

---

MaskOps 2.0 shipped this week. Before I told anyone, I looked at my own numbers. Two of them were lying to me — in opposite directions.

MaskOps is a Rust plugin for [Polars](https://pola.rs) that does PII masking inside the dataframe — RUT, CPF, credit cards, IBANs, and twenty-odd more families — air-gapped, with no network call, ever. If you have reached for Microsoft Presidio and found it carries no Latin American identifiers, that is the gap MaskOps fills: check-digit-validated RUT, CPF, and CURP detection alongside the EU, US, and APAC families, as a native Polars expression. Version 2.0 is the enterprise line — configurable patterns, structured extraction, an audit pass that counts what it masked, and format-preserving encryption (GDPR Art. 4(5) pseudonymization) for the reversible cases. That part I was sure of. The numbers around it, less so.

## The first number lied against me — the benchmark

The last thing I checked was the benchmark table in my own README. It said MaskOps ran at 0.4× to 0.7× the speed of plain Python `re`. Slower than the language I wrote it to replace.

I almost opened the profiler. Instead I read the benchmark harness. I should have read it first.

Here is what it did. For every family — "Credit Card", "EU", "LatAm" — it ran the full masker. All thirty-five pattern families at once. Then it compared the time against a Python baseline that ran one regex for that family.

So the "Credit Card" row timed MaskOps scanning for cards, phones, IBANs, Korean RRNs, and thirty others — against Python scanning for cards. The proof sat in the table the whole time: every MaskOps row took the same ~2.3 seconds regardless of family, because it always did all the work. Only the Python column moved.

I was timing my engine doing thirty-five times the work and calling it slow.

Two fixes. The first was the benchmark, not the code: compare like-for-like. When the row says "Credit Card", mask credit cards — the same job the baseline does. MaskOps already supports selection, so it was one argument: `mask_pii("text", patterns=["credit_card"])`.

The second was real. Most rows in real data contain no PII. Every pattern MaskOps detects needs a digit, or an `@`. A row with neither cannot match anything. So before any regex, walk the bytes:

```rust
pub fn has_pii_candidate(value: &str) -> bool {
    value.bytes().any(|b| b.is_ascii_digit() || b == b'@')
}
```

If false, return the string untouched. On clean text this skips all thirty-five scans for the price of one pass over the bytes. Output does not change. The same 394 tests pass.

PII masking in Polars, measured fairly — one million rows, median of three, against a pure-Python `re` baseline with matching coverage:

| Data profile | Speedup vs Python |
|---|---|
| clean (no PII) | 11×–163× |
| mixed (50% PII) | 1.2×–3.2× |
| dense (every row) | 1.3×–2.7× |

One family still loses on dense data: the European ID set runs four separate regex passes, and a single combined Python regex edges it out, 0.9×. I left that in the README. A table with no losses is a table someone tuned until it lied.

## The second number lied for me — the downloads

The other number was downloads. I shipped the 1.7 through 2.0 releases in one short burst, and the PyPI counter jumped from about ten a day to three and a half thousand on release day. A hundredfold, overnight.

It would be easy to write "downloads are exploding." It would also be false.

That spike sits exactly on the days I pushed releases. It is CI building wheels across the OS and Python matrix, mirrors syncing, bots crawling each new version. PyPI counts all of it. None of it is a person deciding to use the thing. Strip the release days and the real line is flat and small — single digits, which is the honest state of a young project.

So I am not going to tell you adoption is taking off. The download number is real and it is mostly noise, and pretending otherwise insults anyone who can open the same pypistats page I did.

## What I take from this

Two instruments. One read low because it measured the wrong thing. One read high because it counted the wrong things. A metric is not a verdict. It is a measurement, and a measurement can be miscalibrated in your favor or against it, and you owe it to yourself to know which.

Read the harness before the flame graph. Strip the release days before you celebrate the downloads. Then trust what is left.

MaskOps is open source, MPL-2.0, on [PyPI](https://pypi.org/project/maskops/). It does PII masking inside Polars, air-gapped, with check-digit validation so a random nine-digit number is not mistaken for an ID. It does not do named-entity recognition. The [source and the benchmark code](https://github.com/fcarvajalbrown/MaskOps) are on GitHub — run it. If your machine disagrees with mine, I want to know.
