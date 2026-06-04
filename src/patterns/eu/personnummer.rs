//! Swedish personnummer detection and masking.
//!
//! Formats:
//!   - YYMMDD-NNNN (short, 11 chars) — most common
//!   - YYYYMMDD-NNNN (long, 13 chars) — official long form
//!   - YYMMDD+NNNN (centenarian, 100+ years old) — treated same as dash form
//!
//! Validation: Luhn on 10 digits (YYMMDDNNNN for short; MMDDNNNN prefixed with
//! last-2 of century for long). Double even-indexed digits (0-indexed, left-to-right);
//! sum mod 10 = 0. Equivalent to standard right-to-left Luhn for 10-digit inputs.
//!
//! Compliance: Swedish GDPR implementation (Dataskyddslagen 2018:218),
//!   Skatteverket (Swedish Tax Agency). GDPR Art. 4(1).
//! Digit-based PII — supports FPE and consistent masking.
//! GDPR Art. 4(5): FPE output is pseudonymization, not anonymization.

use once_cell::sync::Lazy;
use regex::Regex;

/// Short form: YYMMDD[-+]NNNN
static SHORT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{6}[-+]\d{4})\b").unwrap()
});

/// Long form: YYYYMMDD-NNNN
static LONG_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{8}-\d{4})\b").unwrap()
});

fn luhn_valid(digits: &str) -> bool {
    if digits.len() != 10 {
        return false;
    }
    let sum: u32 = digits.chars().enumerate().map(|(i, c)| {
        let d = c as u32 - b'0' as u32;
        if i % 2 == 0 {
            let doubled = d * 2;
            if doubled > 9 { doubled - 9 } else { doubled }
        } else {
            d
        }
    }).sum();
    sum % 10 == 0
}

/// Extracts the 10 Luhn digits from a short-form match (YYMMDD[-+]NNNN).
fn short_digits(raw: &str) -> String {
    raw.chars().filter(|c| c.is_ascii_digit()).collect()
}

/// Extracts the 10 Luhn digits from a long-form match (YYYYMMDD-NNNN):
/// last 10 digits of the 12-digit string (MMDDNNNN + YY prefix → YYMMDDNNNN).
fn long_digits(raw: &str) -> String {
    let all: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
    all[2..].to_string()
}

/// Returns true if the input contains a valid Swedish personnummer.
pub fn contains_personnummer(s: &str) -> bool {
    SHORT_RE.find_iter(s).any(|m| luhn_valid(&short_digits(m.as_str())))
        || LONG_RE.find_iter(s).any(|m| luhn_valid(&long_digits(m.as_str())))
}

/// Masks any valid personnummer found, replacing digits with `*` and preserving separators.
///
/// Short: `811228-9874` → `******-****`. Long: `19811228-9874` → `********-****`.
pub fn mask_personnummer(s: &str) -> String {
    let s = SHORT_RE
        .replace_all(s, |caps: &regex::Captures| {
            let raw = &caps[0];
            if !luhn_valid(&short_digits(raw)) { return raw.to_string(); }
            raw.chars().map(|c| if c.is_ascii_digit() { '*' } else { c }).collect()
        })
        .into_owned();
    LONG_RE
        .replace_all(&s, |caps: &regex::Captures| {
            let raw = &caps[0];
            if !luhn_valid(&long_digits(raw)) { return raw.to_string(); }
            raw.chars().map(|c| if c.is_ascii_digit() { '*' } else { c }).collect()
        })
        .into_owned()
}

/// Masks a valid personnummer using FF3-1 FPE.
///
/// Short: encrypts 10 digits, outputs `XXXXXX-XXXX` (separator preserved).
/// Long: encrypts 12 digits, outputs `XXXXXXXX-XXXX`.
pub fn mask_personnummer_fpe(s: &str, cipher: &crate::patterns::fpe::Ff3Cipher) -> String {
    let s = SHORT_RE
        .replace_all(s, |caps: &regex::Captures| {
            let raw = &caps[0];
            let sep = if raw.contains('+') { '+' } else { '-' };
            let d = short_digits(raw);
            if !luhn_valid(&d) { return raw.to_string(); }
            match cipher.encrypt(&d) {
                Ok(enc) => format!("{}{}{}", &enc[..6], sep, &enc[6..]),
                Err(_)  => raw.to_string(),
            }
        })
        .into_owned();
    LONG_RE
        .replace_all(&s, |caps: &regex::Captures| {
            let raw = &caps[0];
            let all: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
            if !luhn_valid(&long_digits(raw)) { return raw.to_string(); }
            match cipher.encrypt(&all) {
                Ok(enc) => format!("{}-{}", &enc[..8], &enc[8..]),
                Err(_)  => raw.to_string(),
            }
        })
        .into_owned()
}

/// Masks a valid personnummer using HMAC-SHA256 consistent pseudonymization.
///
/// Short: same input → same output given same salt. Output is `XXXXXX-XXXX`.
/// Long: output is `XXXXXXXX-XXXX`.
pub fn mask_personnummer_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher) -> String {
    let s = SHORT_RE
        .replace_all(s, |caps: &regex::Captures| {
            let raw = &caps[0];
            let sep = if raw.contains('+') { '+' } else { '-' };
            let d = short_digits(raw);
            if !luhn_valid(&d) { return raw.to_string(); }
            match hasher.encrypt(&d) {
                Ok(hashed) => format!("{}{}{}", &hashed[..6], sep, &hashed[6..]),
                Err(_)     => raw.to_string(),
            }
        })
        .into_owned();
    LONG_RE
        .replace_all(&s, |caps: &regex::Captures| {
            let raw = &caps[0];
            let all: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
            if !luhn_valid(&long_digits(raw)) { return raw.to_string(); }
            match hasher.encrypt(&all) {
                Ok(hashed) => format!("{}-{}", &hashed[..8], &hashed[8..]),
                Err(_)     => raw.to_string(),
            }
        })
        .into_owned()
}
