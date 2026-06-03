//! UK NHS number detection and masking.
//!
//! Format: 10 digits. Validated using Modulus 11 (weights 10 down to 2).
//! Sum of (digit × weight) must be divisible by 11. Check digit is position 10.
//!
//! Compliance: NHS Data Dictionary. Digit-based — supports FPE.
//! GDPR Art. 4(5): FPE output is pseudonymization, not anonymization.

use once_cell::sync::Lazy;
use regex::Regex;

/// Matches 10 consecutive digits with optional spaces every 3 digits (999 999 9999 format).
static NHS_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{3}[ ]?\d{3}[ ]?\d{4})\b").unwrap()
});

/// Validates an NHS number using Modulus 11.
fn valid_nhs(s: &str) -> bool {
    let digits: Vec<u32> = s.chars().filter_map(|c| c.to_digit(10)).collect();
    if digits.len() != 10 {
        return false;
    }
    let sum: u32 = digits[..9].iter().enumerate().map(|(i, &d)| d * (10 - i as u32)).sum();
    let remainder = sum % 11;
    if remainder == 1 {
        return false; // invalid by NHS rules
    }
    let check = (11 - remainder) % 11;
    check == digits[9]
}

/// Returns true if the input contains a valid NHS number.
pub fn contains_nhs(s: &str) -> bool {
    NHS_RE.find_iter(s).any(|m| valid_nhs(m.as_str()))
}

/// Masks any valid NHS number found, preserving spaces.
///
/// Example: `943 476 5919` → `*** *** ****`
pub fn mask_nhs(s: &str) -> String {
    NHS_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_nhs(&caps[0]) {
                return caps[0].to_string();
            }
            caps[0].chars().map(|c| if c.is_ascii_digit() { '*' } else { c }).collect()
        })
        .into_owned()
}

/// Masks a valid NHS number using HMAC-SHA256 consistent pseudonymization on the 10 digits.
///
/// Spaces stripped on output. Not reversible without salt.
pub fn mask_nhs_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher) -> String {
    NHS_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_nhs(&caps[0]) {
                return caps[0].to_string();
            }
            let digits: String = caps[0].chars().filter(|c| c.is_ascii_digit()).collect();
            match hasher.encrypt(&digits) {
                Ok(hashed) => hashed,
                Err(_)     => caps[0].to_string(),
            }
        })
        .into_owned()
}

/// Masks a valid NHS number using FF3-1 FPE on the 10 digits.
///
/// Spaces stripped on output. Reversible with the same key and tweak.
pub fn mask_nhs_fpe(s: &str, cipher: &crate::patterns::fpe::Ff3Cipher) -> String {
    NHS_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_nhs(&caps[0]) {
                return caps[0].to_string();
            }
            let digits: String = caps[0].chars().filter(|c| c.is_ascii_digit()).collect();
            match cipher.encrypt(&digits) {
                Ok(enc) => enc,
                Err(_)  => caps[0].to_string(),
            }
        })
        .into_owned()
}
