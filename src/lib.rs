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
use patterns::{mask_all, mask_all_fpe, contains_any_pii};
use patterns::{Ff3Cipher, KEY_LEN, TWEAK_LEN};
pub use patterns::fpe::Ff3Cipher as MaskopsFpe;

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
// Expression: mask_pii_fpe
// ---------------------------------------------------------------------------
/// Polars expression: masks digit PII (cards, phones, RUT, CPF) with FF3-1 FPE.
/// Non-digit PII (IBAN, VAT, email, IP, EU IDs) is still asterisked.
///
/// inputs[0]: Utf8 column to mask
/// inputs[1]: Binary scalar — 32-byte AES-256 key
/// inputs[2]: Binary scalar — 7-byte tweak
#[polars_expr(output_type=String)]
fn mask_pii_fpe(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].str()?;

    let key_series   = inputs[1].cast(&DataType::Binary)?;
    let tweak_series = inputs[2].cast(&DataType::Binary)?;

    let key_bytes = key_series.binary()?
        .get(0)
        .ok_or_else(|| PolarsError::ComputeError("mask_pii_fpe: missing key".into()))?;
    let tweak_bytes = tweak_series.binary()?
        .get(0)
        .ok_or_else(|| PolarsError::ComputeError("mask_pii_fpe: missing tweak".into()))?;

    if key_bytes.len() != KEY_LEN {
        return Err(PolarsError::ComputeError(
            format!("mask_pii_fpe: key must be {} bytes, got {}", KEY_LEN, key_bytes.len()).into()
        ));
    }
    if tweak_bytes.len() != TWEAK_LEN {
        return Err(PolarsError::ComputeError(
            format!("mask_pii_fpe: tweak must be {} bytes, got {}", TWEAK_LEN, tweak_bytes.len()).into()
        ));
    }

    let key:   [u8; KEY_LEN]   = key_bytes.try_into().unwrap();
    let tweak: [u8; TWEAK_LEN] = tweak_bytes.try_into().unwrap();
    let cipher = Ff3Cipher::new(&key, &tweak);

    let out: StringChunked = ca.apply(|opt_val: Option<&str>| {
        opt_val.map(|s| std::borrow::Cow::Owned(mask_all_fpe(s, &cipher)))
    });
    Ok(out.into_series())
}

// ---------------------------------------------------------------------------
// PyO3 module — entry point for maturin
// ---------------------------------------------------------------------------

#[pymodule]
fn _maskops(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", "0.1.5")?;
    Ok(())
}