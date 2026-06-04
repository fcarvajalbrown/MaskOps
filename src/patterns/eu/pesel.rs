//! Polish PESEL (Powszechny Elektroniczny System Ewidencji Ludności) detection and masking.
//!
//! Format: 11 digits YYMMDDSSSSCC.
//! YY=year, MM=month (century encoded: 01–12=1900s, 21–32=2000s), DD=day,
//! SSSS=serial (last digit odd=male, even=female), C=check digit.
//! Check: weights [1,3,7,9,1,3,7,9,1,3] on digits 0–9; check=(10−sum%10)%10.
//!
//! Compliance: UODO (Polish data protection authority), GDPR Art. 4(1).
//! Digit-based PII — supports FPE and consistent masking.
//! GDPR Art. 4(5): FPE output is pseudonymization, not anonymization.
//! Note: CPF (Brazilian) is also 11 digits; both validations rarely pass simultaneously.

use once_cell::sync::Lazy;
use regex::Regex;

static PESEL_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{11})\b").unwrap()
});

const WEIGHTS: [u32; 10] = [1, 3, 7, 9, 1, 3, 7, 9, 1, 3];

fn valid_pesel(s: &str) -> bool {
    if s.len() != 11 {
        return false;
    }
    let digits: Vec<u32> = s.chars().filter_map(|c| c.to_digit(10)).collect();
    if digits.len() != 11 {
        return false;
    }
    let sum: u32 = digits[..10].iter().zip(WEIGHTS.iter()).map(|(d, w)| d * w).sum();
    (10 - sum % 10) % 10 == digits[10]
}

/// Returns true if the input contains a valid Polish PESEL.
pub fn contains_pesel(s: &str) -> bool {
    PESEL_RE.find_iter(s).any(|m| valid_pesel(m.as_str()))
}

/// Masks any valid PESEL found with asterisks.
///
/// Example: `91010112346` → `***********`
pub fn mask_pesel(s: &str) -> String {
    PESEL_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_pesel(&caps[0]) {
                return caps[0].to_string();
            }
            "*".repeat(11)
        })
        .into_owned()
}

/// Masks a valid PESEL using FF3-1 FPE on the 11 digits.
///
/// Output is 11 digits. Reversible with the same key and tweak.
pub fn mask_pesel_fpe(s: &str, cipher: &crate::patterns::fpe::Ff3Cipher) -> String {
    PESEL_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_pesel(&caps[0]) {
                return caps[0].to_string();
            }
            match cipher.encrypt(&caps[0]) {
                Ok(enc) => enc,
                Err(_)  => caps[0].to_string(),
            }
        })
        .into_owned()
}

/// Masks a valid PESEL using HMAC-SHA256 consistent pseudonymization.
///
/// Same input → same output given the same salt. Not reversible without salt.
pub fn mask_pesel_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher) -> String {
    PESEL_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_pesel(&caps[0]) {
                return caps[0].to_string();
            }
            match hasher.encrypt(&caps[0]) {
                Ok(hashed) => hashed,
                Err(_)     => caps[0].to_string(),
            }
        })
        .into_owned()
}
