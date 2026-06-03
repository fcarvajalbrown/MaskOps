//! US National Provider Identifier (NPI) detection and masking.
//!
//! Format: 10 digits, Luhn-validated with a constant prefix factor of 24
//! (HIPAA NPI Luhn check adds 24 to the sum before computing check digit).
//!
//! Compliance: HIPAA 45 CFR §162.406. Digit-based PII — supports FPE.
//! GDPR Art. 4(5): FPE output is pseudonymization, not anonymization.

use once_cell::sync::Lazy;
use regex::Regex;

static NPI_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{10})\b").unwrap()
});

/// Validates a US NPI using the HIPAA Luhn check (prefix factor 24).
fn valid_npi(s: &str) -> bool {
    let digits: Vec<u32> = s.chars().filter_map(|c| c.to_digit(10)).collect();
    if digits.len() != 10 {
        return false;
    }
    // Double every other digit starting from the second-to-last, going left.
    // NPI prefix constant: 24 added to the running sum.
    let sum: u32 = digits[..9].iter().rev().enumerate().map(|(i, &d)| {
        if i % 2 == 0 { // positions 9,7,5,3,1 from right (even index in rev)
            let v = d * 2;
            if v > 9 { v - 9 } else { v }
        } else {
            d
        }
    }).sum::<u32>() + 24;
    (10 - (sum % 10)) % 10 == digits[9]
}

/// Returns true if the input contains a valid NPI.
pub fn contains_npi(s: &str) -> bool {
    NPI_RE.find_iter(s).any(|m| valid_npi(m.as_str()))
}

/// Masks any valid NPI found (full redaction).
///
/// Example: `1234567893` → `**********`
pub fn mask_npi(s: &str) -> String {
    NPI_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_npi(&caps[0]) {
                return caps[0].to_string();
            }
            "*".repeat(10)
        })
        .into_owned()
}

/// Masks a valid NPI using HMAC-SHA256 consistent pseudonymization on all 10 digits.
///
/// Not reversible without salt.
pub fn mask_npi_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher) -> String {
    NPI_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_npi(&caps[0]) {
                return caps[0].to_string();
            }
            match hasher.encrypt(&caps[0]) {
                Ok(hashed) => hashed,
                Err(_)     => caps[0].to_string(),
            }
        })
        .into_owned()
}

/// Masks a valid NPI using FF3-1 FPE on all 10 digits.
///
/// Reversible with the same key and tweak.
pub fn mask_npi_fpe(s: &str, cipher: &crate::patterns::fpe::Ff3Cipher) -> String {
    NPI_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_npi(&caps[0]) {
                return caps[0].to_string();
            }
            match cipher.encrypt(&caps[0]) {
                Ok(enc) => enc,
                Err(_)  => caps[0].to_string(),
            }
        })
        .into_owned()
}
