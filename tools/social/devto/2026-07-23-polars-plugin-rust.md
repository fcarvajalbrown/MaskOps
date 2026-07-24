# Writing a native Polars plugin in Rust: PII masking with no per-row Python

- Published: https://dev.to/fcarvajalbrown/writing-a-native-polars-plugin-in-rust-pii-masking-with-no-per-row-python-2co8
- Date: 2026-07-23
- Tags: rust, polars, python, dataengineering
- Cover: tools/social/covers/polars-plugin-rust.png
- Description: How to build a native Polars expression plugin in Rust with pyo3-polars: register mask_pii-style expressions that run on Arrow buffers, with no per-row Python overhead.

---

A column of payment notes will teach you where Python spends its time. Mine held free text: "charged card 4111 1111 1111 1111", "cliente RUT 12.345.678-5 activo", the ordinary residue of a support desk. The task was small. Redact the numbers, keep the sentence. In pure Python the shape is familiar:

```python
df.with_columns(
    pl.col("notes").map_elements(mask, return_dtype=pl.String)
)
```

`map_elements` does exactly what it says. It calls a Python function once per row. At a thousand rows you never notice. At a million, the interpreter is now the workload: one function call, one `str` object, one set of `re` allocations per row, repeated a million times. Polars ran the scan in Rust and then handed every value back across the boundary so Python could look at it alone. The vectorized engine underneath went idle.

Polars has a way out that is not "vectorize your regex in NumPy." You write the per-value work once, in Rust, and register it as an expression. The function receives the whole column as an Arrow-backed `Series` and returns a `Series`. Python is called once, not once per row. This is the expression-plugin interface, and the boilerplate is smaller than its reputation.

## The two halves of the bridge

A plugin is a compiled dynamic library plus a thin Python function that points Polars at it.

On the Rust side you write a function over `Series` and annotate it. `pyo3-polars` supplies the macro:

```rust
use pyo3_polars::derive::polars_expr;
use pyo3_polars::export::polars_core::prelude::*;

#[polars_expr(output_type=String)]
fn mask_digits(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].str()?;
    let out: StringChunked =
        ca.apply(|opt| opt.map(|s| std::borrow::Cow::Owned(mask_cell(s))));
    Ok(out.into_series())
}
```

On the Python side you declare the same function by name and tell Polars how it behaves:

```python
from pathlib import Path
import polars as pl
from polars.plugins import register_plugin_function

PLUGIN = Path(__file__).parent

def mask_digits(expr):
    col = pl.col(expr) if isinstance(expr, str) else expr
    return register_plugin_function(
        plugin_path=PLUGIN,
        function_name="mask_digits",
        args=[col],
        is_elementwise=True,
    )
```

That is the entire contract. `function_name` matches the Rust function. `plugin_path` is the directory holding the compiled library. `is_elementwise=True` is a promise about the function, and I will come back to how load-bearing that promise is.

Called from Python, it reads like any other expression:

```python
df.with_columns(mask_digits("notes"))
```

The difference is underneath. The closure inside `mask_digits` runs in compiled code, across the column, with the interpreter uninvolved until the `Series` comes back.

## The crate that makes it a plugin

The build side is three lines of intent. The crate compiles to a C-compatible dynamic library, and it depends on `pyo3-polars` with the derive feature:

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
pyo3 = { version = "0.25", features = ["extension-module"] }
pyo3-polars = { version = "0.23", features = ["derive"] }
polars-core = "0.50"
regex = "1"
once_cell = "1"
```

`crate-type = ["cdylib"]` is the load-bearing choice. It produces a `.so` on Linux, a `.pyd` on Windows, a `.dylib` on macOS: an artifact the CPython process can `dlopen` and Polars can call into with no per-row marshalling. `maturin develop` compiles it and drops it beside the Python package, ready to import. There is no separate build step to remember once it is wired.

One constraint deserves a line of its own, because it fails loudly and confusingly when ignored. `pyo3` and `pyo3-polars` are versioned together. MaskOps pins `pyo3 = 0.25` against `pyo3-polars = 0.23`, and bumping one without the other produces link errors that read like compiler bugs and are not. Treat the pair as a single dependency with two names.

## What `inputs[0].str()` actually hands you

A `Series` is Polars' typed column. `inputs[0].str()?` narrows it to a `StringChunked`, the UTF-8 variant, backed by Arrow buffers: a contiguous block of bytes and an offsets array, not a vector of heap strings. Iterating it yields `Option<&str>` borrowed straight from those buffers. The `?` is the type check. Hand the expression a non-string column and it returns a clean Polars error instead of misbehaving.

`apply` walks the values and hands back a new chunked array. The `Cow` in the closure earns its place. `Cow::Owned` says "I built a new string here." Its sibling `Cow::Borrowed` says "I changed nothing, reuse the original bytes." A masker that returns most rows untouched can hand back borrows and skip the allocation entirely on those rows. On a column that is mostly clean, which is most real text, that is the difference between allocating a million strings and allocating a few thousand.

## The optimization worth stealing

Here is the part that carries over to any plugin doing pattern work, whatever the patterns are.

Regex is not free, and most rows do not match. A note that reads "customer called back, resolved" contains no card, no national ID, no email. Running a battery of compiled patterns against it is wasted work, and it is the common case: production text is mostly PII-free. So before any regex runs, MaskOps asks a cheaper question:

```rust
#[inline]
pub fn has_pii_candidate(value: &str) -> bool {
    value.bytes().any(|b| b.is_ascii_digit() || b == b'@')
}
```

Every identifier the library detects contains at least one digit or an `@`. A row with neither cannot contain any of them, so it skips the whole pattern battery and returns untouched. This is a byte scan with no allocation and no Unicode decoding: it reads raw bytes and stops at the first digit or `@`. It costs almost nothing, and on clean text it replaces everything. The regex engine, the check-digit validation, the per-family passes: none of it runs on a row that had no candidate byte to begin with.

The lesson generalizes past masking. A plugin that runs expensive per-value work should first ask the cheapest possible question that can rule the work out. Cheap byte-level rejection in front of expensive structured matching is most of why the Rust column pass stays flat while a Python `re` loop degrades with every pattern you add. Nothing about the engine got cleverer. It stopped doing work it could prove was pointless.

## `is_elementwise`, and why the flag is a promise

`is_elementwise=True` tells Polars that row *i* of the output depends only on row *i* of the input. That licenses the engine to do what it likes with the boundaries: split the column across threads, run it inside a `group_by`, stream it in chunks. The optimizer trusts the flag outright and never re-derives it.

Masking is genuinely elementwise, so the promise holds. A rolling mean is not, and marking one elementwise would produce wrong answers at every chunk edge, silently, with no error to catch. The rule is flat: set the flag to what is true, never to what is convenient.

## Beyond a single string column

Two needs show up the moment the toy grows.

The output type varies. `mask_pii` returns text, but `extract_pii` returns one field per identifier family, and its declared type is a `Struct`. Polars needs the schema before execution, so a fixed `output_type=String` gives way to a function that computes the output `Field`:

```rust
#[polars_expr(output_type_func=extract_pii_output)]
fn extract_pii(inputs: &[Series]) -> PolarsResult<Series> { /* ... */ }
```

And a plugin only ever receives `Series`, never loose Python values. Configuration has to arrive as data. MaskOps passes an FPE key as a `Binary` literal and the selected pattern list as a comma-joined string literal, each wrapped in `pl.lit` on the Python side and read back as `inputs[1]`, `inputs[2]`, and so on:

```python
args = [col, pl.lit(",".join(patterns))]
```

Inside the kernel that literal is just another `Series`; you read element zero and parse it. It is a small ceremony, and it keeps the boundary honest. Everything crossing into Rust is columnar.

## What a plugin does not fix

The plugin boundary buys throughput on structured, per-value work. It does nothing for problems of a different shape.

Named-entity recognition is the clear line. Finding a person's name in a sentence needs a model and context, not a pattern over one cell, and that is Presidio's job, not a regex plugin's. A plugin also does not make a bad pattern correct: a card regex with no Luhn check will still mask phone numbers that happen to have sixteen digits, in Rust, quickly. Speed is not accuracy. The plugin only removes the interpreter from the hot path. What runs on that path is still yours to get right.

## The one idea

Cross the language boundary once per column, in compiled code, and the per-row Python tax is not reduced. It is not paid at all. Everything else here (the Arrow buffers, the `Cow`, the byte short-circuit, the honest `is_elementwise`) is detail in service of that single move. You are not rewriting Polars. You are adding one compiled column operation and letting the engine keep the row loop where it belongs, which is out of Python.

MaskOps is the worked example, built exactly this way: [github.com/fcarvajalbrown/MaskOps](https://github.com/fcarvajalbrown/MaskOps), on PyPI as [`maskops`](https://pypi.org/project/maskops/). The full plugin skeleton, including the `maturin` configuration that binds the compiled module to its Python package, is in the Polars [plugin documentation](https://docs.pola.rs/user-guide/plugins/).
