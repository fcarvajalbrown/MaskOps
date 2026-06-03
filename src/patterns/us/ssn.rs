//! US Social Security Number (SSN) detection and masking.
//!
//! Format: AAA-GG-SSSS (dashed). Area 001–899 (excluding 000 and 666).
//! Group 01–99. Serial 0001–9999. Area 900–999 are ITINs, not SSNs.
//! Excludes two historically leaked invalid numbers.
//!
//! Digit-based PII: supports asterisk masking and FF3-1 FPE.
//! GDPR Art. 4(5): FPE output is pseudonymization, not anonymization.

use once_cell::sync::Lazy;
use regex::Regex;

static SSN_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{3})-(\d{2})-(\d{4})\b").unwrap()
});

// Woolworth wallet (078-05-1120) and promotional misuse (219-09-9999).
const KNOWN_INVALID: &[&str] = &["078051120", "219099999"];

fn valid_ssn(area: &str, group: &str, serial: &str) -> bool {
    let a: u32 = match area.parse() { Ok(v) => v, Err(_) => return false };
    let g: u32 = match group.parse() { Ok(v) => v, Err(_) => return false };
    let s: u32 = match serial.parse() { Ok(v) => v, Err(_) => return false };
    if a == 0 || a == 666 || a >= 900 { return false; }
    if g == 0 { return false; }
    if s == 0 { return false; }
    let compact = format!("{:03}{:02}{:04}", a, g, s);
    !KNOWN_INVALID.contains(&compact.as_str())
}

/// Returns true if the input contains a valid SSN.
pub fn contains_ssn(s: &str) -> bool {
    SSN_RE.captures_iter(s).any(|caps| valid_ssn(&caps[1], &caps[2], &caps[3]))
}

/// Masks any SSN found with asterisks, preserving dashes.
///
/// Example: `123-45-6789` → `***-**-****`
pub fn mask_ssn(s: &str) -> String {
    SSN_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_ssn(&caps[1], &caps[2], &caps[3]) {
                return caps[0].to_string();
            }
            "***-**-****".to_string()
        })
        .into_owned()
}

/// Masks any SSN using HMAC-SHA256 consistent pseudonymization on the 9 digits.
///
/// Dashes preserved in output. Not reversible without salt.
pub fn mask_ssn_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher) -> String {
    SSN_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_ssn(&caps[1], &caps[2], &caps[3]) {
                return caps[0].to_string();
            }
            let digits = format!("{}{}{}", &caps[1], &caps[2], &caps[3]);
            match hasher.encrypt(&digits) {
                Ok(hashed) => format!("{}-{}-{}", &hashed[..3], &hashed[3..5], &hashed[5..]),
                Err(_)     => caps[0].to_string(),
            }
        })
        .into_owned()
}

/// Masks any SSN using FF3-1 format-preserving encryption on the 9 digits.
///
/// Dashes are preserved in the output. Reversible with the same key and tweak.
///
/// Example: `123-45-6789` → `361-98-4203`  (same structure, reversible)
pub fn mask_ssn_fpe(s: &str, cipher: &crate::patterns::fpe::Ff3Cipher) -> String {
    SSN_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_ssn(&caps[1], &caps[2], &caps[3]) {
                return caps[0].to_string();
            }
            let digits = format!("{}{}{}", &caps[1], &caps[2], &caps[3]);
            match cipher.encrypt(&digits) {
                Ok(enc) => format!("{}-{}-{}", &enc[..3], &enc[3..5], &enc[5..]),
                Err(_) => caps[0].to_string(),
            }
        })
        .into_owned()
}
