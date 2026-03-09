//! Mexican CURP (Clave Única de Registro de Población) detection and masking.
//!
//! Format: 18-character alphanumeric code
//! No check digit validation — format-only matching.
//!
//! Example:
//!   BADD110313HCMLNS09  →  ******************

use once_cell::sync::Lazy;
use regex::Regex;

static CURP_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b([A-Z][AEIOU][A-Z]{2}\d{6}[HM][A-Z]{2}[B-DF-HJ-NP-TV-Z]{3}[A-Z0-9]\d)\b").unwrap()
});

/// Returns true if the input contains a CURP.
pub fn contains_curp(s: &str) -> bool {
    CURP_RE.is_match(s)
}

/// Masks any CURP found in the input.
pub fn mask_curp(s: &str) -> String {
    CURP_RE.replace_all(s, |caps: &regex::Captures| {
        "*".repeat(caps[0].len())
    }).into_owned()
}