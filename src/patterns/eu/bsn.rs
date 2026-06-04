//! Dutch BSN (Burgerservicenummer) detection and masking.
//!
//! Format: 9 consecutive digits.
//! Validation: 11-proof (elfproef). Weights [9,8,7,6,5,4,3,2,−1].
//! Sum = Σ(digit[i] × weight[i]) must be divisible by 11.
//!
//! Compliance: Dutch AVG (GDPR implementation), WBP, GDPR Art. 4(1).
//! Digit-based PII — supports FPE and consistent masking.
//! GDPR Art. 4(5): FPE output is pseudonymization, not anonymization.
//! Note: ~9% of random 9-digit numbers pass the 11-proof. Some valid SINs (Luhn)
//! also pass elfproef — whichever detector runs first in the pipeline wins.

use once_cell::sync::Lazy;
use regex::Regex;

static BSN_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{9})\b").unwrap()
});

const BSN_WEIGHTS: [i32; 9] = [9, 8, 7, 6, 5, 4, 3, 2, -1];

fn valid_bsn(s: &str) -> bool {
    let digits: Vec<i32> = s.chars().filter_map(|c| c.to_digit(10).map(|d| d as i32)).collect();
    if digits.len() != 9 {
        return false;
    }
    let sum: i32 = digits.iter().zip(BSN_WEIGHTS.iter()).map(|(d, w)| d * w).sum();
    sum > 0 && sum % 11 == 0
}

/// Returns true if the input contains a valid Dutch BSN.
pub fn contains_bsn(s: &str) -> bool {
    BSN_RE.find_iter(s).any(|m| valid_bsn(m.as_str()))
}

/// Masks any valid BSN found with asterisks.
///
/// Example: `123456782` → `*********`
pub fn mask_bsn(s: &str) -> String {
    BSN_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_bsn(&caps[0]) {
                return caps[0].to_string();
            }
            "*".repeat(9)
        })
        .into_owned()
}

/// Masks a valid BSN using FF3-1 FPE on the 9 digits.
///
/// Output is 9 digits. Reversible with the same key and tweak.
pub fn mask_bsn_fpe(s: &str, cipher: &crate::patterns::fpe::Ff3Cipher) -> String {
    BSN_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_bsn(&caps[0]) {
                return caps[0].to_string();
            }
            match cipher.encrypt(&caps[0]) {
                Ok(enc) => enc,
                Err(_)  => caps[0].to_string(),
            }
        })
        .into_owned()
}

/// Masks a valid BSN using HMAC-SHA256 consistent pseudonymization.
///
/// Same input → same output given the same salt. Not reversible without salt.
pub fn mask_bsn_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher) -> String {
    BSN_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_bsn(&caps[0]) {
                return caps[0].to_string();
            }
            match hasher.encrypt(&caps[0]) {
                Ok(hashed) => hashed,
                Err(_)     => caps[0].to_string(),
            }
        })
        .into_owned()
}
