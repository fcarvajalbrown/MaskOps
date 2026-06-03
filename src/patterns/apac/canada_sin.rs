//! Canadian Social Insurance Number (SIN) detection and masking.
//!
//! Format: NNN-NNN-NNN (formatted) or NNNNNNNNN (compact, 9 digits).
//! First digit: 1–9 (0 is unissued; 9 = temporary SINs for foreign workers/students).
//! Validation: Luhn algorithm on all 9 digits.
//!
//! Compliance: Personal Information Protection and Electronic Documents Act (PIPEDA),
//!   Privacy Act (Canada).
//! Digit-based PII — supports FPE and consistent masking.
//! GDPR Art. 4(5): FPE output is pseudonymization, not anonymization.
//! Note: compact regex excludes matches immediately followed by a dash, to avoid
//! colliding with NIT-style NNNNNNNNN-D sequences.

use once_cell::sync::Lazy;
use regex::Regex;

/// Formatted SIN: NNN-NNN-NNN, first digit 1-9.
static SIN_FMT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b([1-9]\d{2}-\d{3}-\d{3})\b").unwrap()
});

/// Compact SIN: 9 consecutive digits, first digit 1-9.
static SIN_COMPACT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b([1-9]\d{8})\b").unwrap()
});

fn luhn_valid(digits: &str) -> bool {
    if digits.len() != 9 {
        return false;
    }
    let sum: u32 = digits
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            let d = c as u32 - b'0' as u32;
            if i % 2 == 1 {
                let doubled = d * 2;
                if doubled > 9 { doubled - 9 } else { doubled }
            } else {
                d
            }
        })
        .sum();
    sum % 10 == 0
}

/// Returns true if the compact match at position `end` is not followed by a dash.
fn not_followed_by_dash(s: &str, end: usize) -> bool {
    s.as_bytes().get(end) != Some(&b'-')
}

/// Returns true if the input contains a valid Canadian SIN.
pub fn contains_sin(s: &str) -> bool {
    SIN_FMT_RE.find_iter(s).any(|m| {
        let d: String = m.as_str().chars().filter(|c| c.is_ascii_digit()).collect();
        luhn_valid(&d)
    }) || SIN_COMPACT_RE.find_iter(s).any(|m| {
        luhn_valid(m.as_str()) && not_followed_by_dash(s, m.end())
    })
}

/// Masks any valid Canadian SIN found (full redaction).
///
/// Formatted: `130-692-544` → `***-***-***`. Compact: `130692544` → `*********`.
pub fn mask_sin(s: &str) -> String {
    let s = SIN_FMT_RE
        .replace_all(s, |caps: &regex::Captures| {
            let raw = &caps[0];
            let d: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
            if !luhn_valid(&d) { return raw.to_string(); }
            "*".repeat(raw.len())
        })
        .into_owned();
    mask_sin_compact_asterisk(&s)
}

fn mask_sin_compact_asterisk(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut last = 0;
    for m in SIN_COMPACT_RE.find_iter(s) {
        if !luhn_valid(m.as_str()) || !not_followed_by_dash(s, m.end()) {
            result.push_str(&s[last..m.end()]);
            last = m.end();
            continue;
        }
        result.push_str(&s[last..m.start()]);
        result.push_str(&"*".repeat(9));
        last = m.end();
    }
    result.push_str(&s[last..]);
    result
}

/// Masks a valid Canadian SIN using FF3-1 FPE on the 9 digits.
///
/// Format preserved. Reversible with the same key and tweak.
pub fn mask_sin_fpe(s: &str, cipher: &crate::patterns::fpe::Ff3Cipher) -> String {
    let s = SIN_FMT_RE
        .replace_all(s, |caps: &regex::Captures| {
            let raw = &caps[0];
            let d: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
            if !luhn_valid(&d) { return raw.to_string(); }
            match cipher.encrypt(&d) {
                Ok(enc) => format!("{}-{}-{}", &enc[..3], &enc[3..6], &enc[6..]),
                Err(_) => raw.to_string(),
            }
        })
        .into_owned();
    mask_sin_compact_fpe(&s, cipher)
}

fn mask_sin_compact_fpe(s: &str, cipher: &crate::patterns::fpe::Ff3Cipher) -> String {
    let mut result = String::with_capacity(s.len());
    let mut last = 0;
    for m in SIN_COMPACT_RE.find_iter(s) {
        if !luhn_valid(m.as_str()) || !not_followed_by_dash(s, m.end()) {
            result.push_str(&s[last..m.end()]);
            last = m.end();
            continue;
        }
        result.push_str(&s[last..m.start()]);
        match cipher.encrypt(m.as_str()) {
            Ok(enc) => result.push_str(&enc),
            Err(_)  => result.push_str(m.as_str()),
        }
        last = m.end();
    }
    result.push_str(&s[last..]);
    result
}

/// Masks a valid Canadian SIN using HMAC-SHA256 consistent pseudonymization.
///
/// Same input → same output given the same salt. Not reversible without salt.
pub fn mask_sin_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher) -> String {
    let s = SIN_FMT_RE
        .replace_all(s, |caps: &regex::Captures| {
            let raw = &caps[0];
            let d: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
            if !luhn_valid(&d) { return raw.to_string(); }
            match hasher.encrypt(&d) {
                Ok(hashed) => format!("{}-{}-{}", &hashed[..3], &hashed[3..6], &hashed[6..]),
                Err(_) => raw.to_string(),
            }
        })
        .into_owned();
    mask_sin_compact_consistent(&s, hasher)
}

fn mask_sin_compact_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher) -> String {
    let mut result = String::with_capacity(s.len());
    let mut last = 0;
    for m in SIN_COMPACT_RE.find_iter(s) {
        if !luhn_valid(m.as_str()) || !not_followed_by_dash(s, m.end()) {
            result.push_str(&s[last..m.end()]);
            last = m.end();
            continue;
        }
        result.push_str(&s[last..m.start()]);
        match hasher.encrypt(m.as_str()) {
            Ok(hashed) => result.push_str(&hashed),
            Err(_)     => result.push_str(m.as_str()),
        }
        last = m.end();
    }
    result.push_str(&s[last..]);
    result
}
