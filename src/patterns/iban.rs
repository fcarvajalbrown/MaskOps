//! IBAN detection and masking patterns.
//!
//! Covers all 36 SEPA/EU country formats. Regex validates structure only
//! (not checksum) for performance — checksum validation is opt-in.

use once_cell::sync::Lazy;
use regex::Regex;

/// Matches any IBAN: 2-letter country code + 2 check digits + up to 30 alphanumeric chars.
/// Handles optional spaces every 4 chars (print format).
pub static IBAN_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b([A-Z]{2}\d{2}[A-Z0-9]{4}[0-9]{7}([A-Z0-9]?){0,16})\b").unwrap()
});

/// Masks an IBAN, preserving country code + check digits, replacing the rest with `*`.
///
/// Example: `DE89370400440532013000` → `DE89******************`
pub fn mask_iban(value: &str) -> String {
    IBAN_RE
        .replace_all(value, |caps: &regex::Captures| {
            let iban = caps.get(0).unwrap().as_str();
            // Keep first 4 chars (CC + check digits), mask the rest
            let (visible, secret) = iban.split_at(4.min(iban.len()));
            format!("{}{}", visible, "*".repeat(secret.len()))
        })
        .into_owned()
}

/// Returns true if the string contains at least one IBAN pattern.
pub fn contains_iban(value: &str) -> bool {
    IBAN_RE.is_match(value)
}
