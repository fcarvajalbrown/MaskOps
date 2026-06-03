//! Medicare Beneficiary Identifier (MBI) detection and masking.
//!
//! Format: 11 characters. CMS pattern (positions 1-indexed):
//!   1:    digit 1-9
//!   2:    alpha (not S, L, O, I, B, Z)
//!   3:    digit or alpha (not S, L, O, I, B, Z)
//!   4:    digit
//!   5:    alpha (not S, L, O, I, B, Z)
//!   6:    alpha (not S, L, O, I, B, Z)
//!   7:    digit
//!   8:    digit or alpha (not S, L, O, I, B, Z)
//!   9:    digit or alpha (not S, L, O, I, B, Z)
//!   10:   digit
//!   11:   digit
//!
//! No check digit. Format-only validation.
//! Compliance: CMS MBI standard (effective April 2018). Non-digit — asterisk only.

use once_cell::sync::Lazy;
use regex::Regex;

// CMS allowed alpha chars: A-Z minus S, L, O, I, B, Z → ACDEFGHJKMNPQRTUVWXY
const ALPHA: &str = "ACDEFGHJKMNPQRTUVWXY";

static MBI_RE: Lazy<Regex> = Lazy::new(|| {
    // Pre-built from the CMS character classes above.
    // Position 1: [1-9]
    // Position 2: [ACDEFGHJKMNPQRTUVWXY]
    // Position 3: [0-9ACDEFGHJKMNPQRTUVWXY]
    // Position 4: [0-9]
    // Position 5-6: [ACDEFGHJKMNPQRTUVWXY]
    // Position 7: [0-9]
    // Position 8-9: [0-9ACDEFGHJKMNPQRTUVWXY]
    // Position 10-11: [0-9]
    Regex::new(
        r"\b([1-9][ACDEFGHJKMNPQRTUVWXY][0-9ACDEFGHJKMNPQRTUVWXY][0-9][ACDEFGHJKMNPQRTUVWXY]{2}[0-9][0-9ACDEFGHJKMNPQRTUVWXY]{2}[0-9]{2})\b"
    ).unwrap()
});

/// Returns true if the input contains an MBI.
pub fn contains_mbi(s: &str) -> bool {
    MBI_RE.is_match(s)
}

/// Masks any MBI found (full redaction).
///
/// Example: `1EG4-TE5-MK72` would be normalized and masked.
/// Example: `1EG4TE5MK72` → `***********`
pub fn mask_mbi(s: &str) -> String {
    MBI_RE
        .replace_all(s, "*".repeat(11).as_str())
        .into_owned()
}

// Suppress unused import warning
#[allow(dead_code)]
const _ALPHA_CHECK: &str = ALPHA;
