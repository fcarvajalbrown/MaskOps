//! Uruguayan cédula de identidad (CI) detection and masking.
//!
//! Format: D.DDD.DDD-D (7-digit body + 1-digit check, formatted with dots and dash).
//! Check digit: weights [2,9,8,7,6,3,4] applied to body digits; check = (10 − sum%10) % 10.
//!
//! Compliance: Ley 18.331 (Uruguay — Protección de Datos Personales).
//! EU adequacy bridge jurisdiction (Uruguay has GDPR adequacy status).
//! Digit-based PII — supports FPE and consistent masking.
//! GDPR Art. 4(5): FPE output is pseudonymization, not anonymization.
//! Note: only the formatted D.DDD.DDD-D pattern is matched to avoid collision
//! with Peruvian DNI (bare 8-digit sequences).

use once_cell::sync::Lazy;
use regex::Regex;

/// Matches the formatted Uruguayan CI: single-digit group, then 3+3 digit groups, then check.
static UY_CI_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d\.\d{3}\.\d{3}-\d)\b").unwrap()
});

const UY_WEIGHTS: [u32; 7] = [2, 9, 8, 7, 6, 3, 4];

fn valid_uy_ci(ci: &str) -> bool {
    let digits: String = ci.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() != 8 {
        return false;
    }
    let ds: Vec<u32> = digits.chars().map(|c| c as u32 - b'0' as u32).collect();
    let sum: u32 = ds[..7]
        .iter()
        .zip(UY_WEIGHTS.iter())
        .map(|(&d, &w)| d * w)
        .sum();
    (10 - (sum % 10)) % 10 == ds[7]
}

/// Returns true if the input contains a valid Uruguayan cédula (formatted form).
pub fn contains_uy_ci(s: &str) -> bool {
    UY_CI_RE.find_iter(s).any(|m| valid_uy_ci(m.as_str()))
}

/// Masks any valid Uruguayan cédula found (full redaction).
///
/// Example: `1.111.111-1` → `***********`
pub fn mask_uy_ci(s: &str) -> String {
    UY_CI_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_uy_ci(&caps[0]) {
                return caps[0].to_string();
            }
            "*".repeat(caps[0].len())
        })
        .into_owned()
}

/// Masks a valid Uruguayan cédula using FF3-1 FPE on the 8 digits.
///
/// Format preserved: D.DDD.DDD-D. Reversible with the same key and tweak.
pub fn mask_uy_ci_fpe(s: &str, cipher: &crate::patterns::fpe::Ff3Cipher) -> String {
    UY_CI_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_uy_ci(&caps[0]) {
                return caps[0].to_string();
            }
            let digits: String = caps[0].chars().filter(|c| c.is_ascii_digit()).collect();
            match cipher.encrypt(&digits) {
                Ok(enc) => format!("{}.{}.{}-{}",
                    &enc[..1], &enc[1..4], &enc[4..7], &enc[7..]),
                Err(_) => caps[0].to_string(),
            }
        })
        .into_owned()
}

/// Masks a valid Uruguayan cédula using HMAC-SHA256 consistent pseudonymization.
///
/// Same input → same output given the same salt. Not reversible without salt.
pub fn mask_uy_ci_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher) -> String {
    UY_CI_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_uy_ci(&caps[0]) {
                return caps[0].to_string();
            }
            let digits: String = caps[0].chars().filter(|c| c.is_ascii_digit()).collect();
            match hasher.encrypt(&digits) {
                Ok(hashed) => format!("{}.{}.{}-{}",
                    &hashed[..1], &hashed[1..4], &hashed[4..7], &hashed[7..]),
                Err(_) => caps[0].to_string(),
            }
        })
        .into_owned()
}
