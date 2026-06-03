//! Australian Tax File Number (TFN) detection and masking.
//!
//! Format: NNN NNN NNN (space-separated) or NNNNNNNNN (compact, 9 digits).
//! Validation: weighted sum mod 11 = 0.
//! Weights: [1, 4, 3, 7, 5, 8, 6, 9, 10].
//!
//! Compliance: Privacy Act 1988 (Australia), Tax File Number Guidelines (ATO).
//! Digit-based PII — supports FPE and consistent masking.
//! GDPR Art. 4(5): FPE output is pseudonymization, not anonymization.

use once_cell::sync::Lazy;
use regex::Regex;

/// Space-separated TFN: NNN NNN NNN.
static TFN_SPACED_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{3} \d{3} \d{3})\b").unwrap()
});

/// Compact TFN: 9 consecutive digits, first digit 1-9.
static TFN_COMPACT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b([1-9]\d{8})\b").unwrap()
});

const TFN_WEIGHTS: [u32; 9] = [1, 4, 3, 7, 5, 8, 6, 9, 10];

fn valid_tfn(digits: &str) -> bool {
    if digits.len() != 9 {
        return false;
    }
    let sum: u32 = digits
        .chars()
        .zip(TFN_WEIGHTS.iter())
        .map(|(c, &w)| (c as u32 - b'0' as u32) * w)
        .sum();
    sum % 11 == 0
}

/// Returns true if the input contains a valid Australian TFN.
pub fn contains_tfn(s: &str) -> bool {
    TFN_SPACED_RE.find_iter(s).any(|m| {
        let d: String = m.as_str().chars().filter(|c| c.is_ascii_digit()).collect();
        valid_tfn(&d)
    }) || TFN_COMPACT_RE.find_iter(s).any(|m| valid_tfn(m.as_str()))
}

/// Masks any valid Australian TFN found (full redaction).
///
/// Space-separated: `123 456 782` → `*** *** ***`. Compact: `123456782` → `*********`.
pub fn mask_tfn(s: &str) -> String {
    let s = TFN_SPACED_RE
        .replace_all(s, |caps: &regex::Captures| {
            let raw = &caps[0];
            let d: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
            if !valid_tfn(&d) { return raw.to_string(); }
            "*** *** ***".to_string()
        })
        .into_owned();
    TFN_COMPACT_RE
        .replace_all(&s, |caps: &regex::Captures| {
            if !valid_tfn(&caps[0]) { return caps[0].to_string(); }
            "*".repeat(9)
        })
        .into_owned()
}

/// Masks a valid Australian TFN using FF3-1 FPE on the 9 digits.
///
/// Format preserved. Reversible with the same key and tweak.
pub fn mask_tfn_fpe(s: &str, cipher: &crate::patterns::fpe::Ff3Cipher) -> String {
    let s = TFN_SPACED_RE
        .replace_all(s, |caps: &regex::Captures| {
            let raw = &caps[0];
            let d: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
            if !valid_tfn(&d) { return raw.to_string(); }
            match cipher.encrypt(&d) {
                Ok(enc) => format!("{} {} {}", &enc[..3], &enc[3..6], &enc[6..]),
                Err(_) => raw.to_string(),
            }
        })
        .into_owned();
    TFN_COMPACT_RE
        .replace_all(&s, |caps: &regex::Captures| {
            if !valid_tfn(&caps[0]) { return caps[0].to_string(); }
            match cipher.encrypt(&caps[0]) {
                Ok(enc) => enc,
                Err(_) => caps[0].to_string(),
            }
        })
        .into_owned()
}

/// Masks a valid Australian TFN using HMAC-SHA256 consistent pseudonymization.
///
/// Same input → same output given the same salt. Not reversible without salt.
pub fn mask_tfn_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher) -> String {
    let s = TFN_SPACED_RE
        .replace_all(s, |caps: &regex::Captures| {
            let raw = &caps[0];
            let d: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
            if !valid_tfn(&d) { return raw.to_string(); }
            match hasher.encrypt(&d) {
                Ok(hashed) => format!("{} {} {}", &hashed[..3], &hashed[3..6], &hashed[6..]),
                Err(_) => raw.to_string(),
            }
        })
        .into_owned();
    TFN_COMPACT_RE
        .replace_all(&s, |caps: &regex::Captures| {
            if !valid_tfn(&caps[0]) { return caps[0].to_string(); }
            match hasher.encrypt(&caps[0]) {
                Ok(hashed) => hashed,
                Err(_) => caps[0].to_string(),
            }
        })
        .into_owned()
}
