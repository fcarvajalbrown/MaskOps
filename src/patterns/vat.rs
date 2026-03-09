//! EU VAT number detection and masking patterns.
//!
//! Covers all 27 EU member states. Each country has its own format;
//! this module uses a unified regex that captures the common structure.

use once_cell::sync::Lazy;
use regex::Regex;

/// Matches EU VAT numbers across all member states.
/// Format: 2-letter country prefix + 8–12 alphanumeric chars.
pub static VAT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"\b(ATU[0-9]{8}|BE[01][0-9]{9}|BG[0-9]{9,10}|CY[0-9]{8}[A-Z]|CZ[0-9]{8,10}|DE[0-9]{9}|DK[0-9]{8}|EE[0-9]{9}|EL[0-9]{9}|ES[A-Z0-9][0-9]{7}[A-Z0-9]|FI[0-9]{8}|FR[A-Z0-9]{2}[0-9]{9}|HR[0-9]{11}|HU[0-9]{8}|IE([0-9]{7}[A-Z]{1,2}|[0-9][A-Z][0-9]{5}[A-Z])|IT[0-9]{11}|LT([0-9]{9}|[0-9]{12})|LU[0-9]{8}|LV[0-9]{11}|MT[0-9]{8}|NL[0-9]{9}B[0-9]{2}|PL[0-9]{10}|PT[0-9]{9}|RO[0-9]{2,10}|SE[0-9]{12}|SI[0-9]{8}|SK[0-9]{10})\b"
    ).unwrap()
});

/// Masks a VAT number, preserving the 2-char country prefix only.
///
/// Example: `DE123456789` → `DE*********`
pub fn mask_vat(value: &str) -> String {
    VAT_RE
        .replace_all(value, |caps: &regex::Captures| {
            let vat = caps.get(0).unwrap().as_str();
            let (prefix, rest) = vat.split_at(2.min(vat.len()));
            format!("{}{}", prefix, "*".repeat(rest.len()))
        })
        .into_owned()
}

/// Returns true if the string contains at least one EU VAT number.
pub fn contains_vat(value: &str) -> bool {
    VAT_RE.is_match(value)
}
