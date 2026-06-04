# Rust Performance Review — MaskOps

You are reviewing a diff for MaskOps, a native Polars plugin for high-speed PII masking. The Rust core compiles to a cdylib. Per-row performance matters — this runs on DataFrames with millions of rows.

## Benchmark context

{{BENCH_CONTEXT}}

## What to look for

1. **Unnecessary heap allocations** — `.to_string()`, `.collect::<Vec<_>>()`, `.clone()` that could be avoided with borrowed types or in-place mutation
2. **Unparallelized hot loops** — iteration over Series or ChunkedArray rows that does not use `rayon::par_iter()` or `par_bridge()`
3. **Regex compiled inside loops** — `Regex::new(...)` called per-row instead of via `once_cell::sync::Lazy` or compiled once at the call site
4. **Inefficient iterator chains** — multiple `.map().filter().collect()` passes where a single pass would do
5. **String copies in masking** — `replace()` or `format!()` allocating new strings where byte-level in-place replacement would work

## Diff to review

```diff
{{DIFF}}
```

## Output format

For each suggestion:

**[IMPACT: high/medium/low]** `file.rs:line`

Before:
```rust
// existing code
```

After:
```rust
// optimized code
```

Reason: one sentence

Skip anything that requires unsafe, changes correctness, or modifies the public API signatures (mask_pii, contains_pii, mask_pii_fpe).

If no opportunities found, respond with exactly: `No optimization opportunities found.`
