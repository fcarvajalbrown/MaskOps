//! US passport number detection and masking.
//!
//! Format: one uppercase letter followed by 8 digits (e.g., A12345678).
//! Current ICAO 9303 compliant US passport format.
//!
//! Non-digit PII (letter prefix makes it non-reversible via digit FPE):
//! always masked with asterisks regardless of mode.
//! GDPR: full redaction — no structural information preserved.

use once_cell::sync::Lazy;
use regex::Regex;

static PASSPORT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b([A-Z][0-9]{8})\b").unwrap()
});

/// Returns true if the input contains a US passport number.
pub fn contains_us_passport(s: &str) -> bool {
    PASSPORT_RE.is_match(s)
}

/// Masks any US passport number found with asterisks (full redaction).
///
/// Example: `A12345678` → `*********`
pub fn mask_us_passport(s: &str) -> String {
    PASSPORT_RE
        .replace_all(s, "*********")
        .into_owned()
}
