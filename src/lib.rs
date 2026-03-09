//! maskops — High-speed PII masking as a native Polars plugin.
//!
//! Exposes two Polars expressions:
//! - `mask_pii(col)`: replaces all detected PII with `*` characters
//! - `contains_pii(col)`: returns a boolean column, true if PII was found

mod patterns;

use pyo3::prelude::*;
use pyo3::types::PyModuleMethods;
use pyo3_polars::derive::polars_expr;
use pyo3_polars::export::polars_core::prelude::*;
use patterns::{mask_all, contains_any_pii};

// ---------------------------------------------------------------------------
// Expression: mask_pii
// ---------------------------------------------------------------------------

/// Polars expression: replaces all PII in a Utf8 column with masked equivalents.
#[polars_expr(output_type=String)]
fn mask_pii(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].str()?;
    let out: StringChunked = ca.apply(|opt_val: Option<&str>| {
        opt_val.map(|s| std::borrow::Cow::Owned(mask_all(s)))
    });
    Ok(out.into_series())
}

// ---------------------------------------------------------------------------
// Expression: contains_pii
// ---------------------------------------------------------------------------

/// Polars expression: returns a boolean Series — true where PII was detected.
#[polars_expr(output_type=Boolean)]
fn contains_pii(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].str()?;
    let out: BooleanChunked = ca.apply_nonnull_values_generic(DataType::Boolean, |s| {
        contains_any_pii(s)
    });
    Ok(out.into_series())
}

// ---------------------------------------------------------------------------
// PyO3 module — entry point for maturin
// ---------------------------------------------------------------------------

#[pymodule]
fn _maskops(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", "0.1.4")?;
    Ok(())
}