//! Ecuadorian cédula de identidad detection and masking.
//!
//! Format: 10 digits. First two digits = province code (01–24).
//! Validated using Módulo 10 (Luhn-like algorithm defined by the Registro Civil).
//!
//! Compliance: Ecuador LOPDP (Ley Orgánica de Protección de Datos Personales),
//! first enforcement actions began 2024. Digit-based PII — supports FPE.
//! GDPR Art. 4(5): FPE output is pseudonymization, not anonymization.

use once_cell::sync::Lazy;
use regex::Regex;

static EC_CEDULA_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{10})\b").unwrap()
});

/// Valid Ecuadorian province codes (01–24).
fn valid_province(d0: u32, d1: u32) -> bool {
    let province = d0 * 10 + d1;
    province >= 1 && province <= 24
}

/// Validates an Ecuadorian cédula using the Registro Civil Módulo 10 algorithm.
fn valid_cedula(s: &str) -> bool {
    let digits: Vec<u32> = s.chars().filter_map(|c| c.to_digit(10)).collect();
    if digits.len() != 10 {
        return false;
    }
    if !valid_province(digits[0], digits[1]) {
        return false;
    }
    // Third digit must be < 6 (natural persons; 6–9 are for juridical entities)
    if digits[2] >= 6 {
        return false;
    }
    let sum: u32 = digits[..9].iter().enumerate().map(|(i, &d)| {
        if i % 2 == 0 {
            let v = d * 2;
            if v >= 10 { v - 9 } else { v }
        } else {
            d
        }
    }).sum();
    let check = (10 - (sum % 10)) % 10;
    check == digits[9]
}

/// Returns true if the input contains a valid Ecuadorian cédula.
pub fn contains_ec_cedula(s: &str) -> bool {
    EC_CEDULA_RE.find_iter(s).any(|m| valid_cedula(m.as_str()))
}

/// Masks any valid Ecuadorian cédula found (full asterisk redaction).
///
/// Example: `1712345678` → `**********`
pub fn mask_ec_cedula(s: &str) -> String {
    EC_CEDULA_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_cedula(&caps[0]) {
                return caps[0].to_string();
            }
            "*".repeat(10)
        })
        .into_owned()
}

/// Masks a valid Ecuadorian cédula using HMAC-SHA256 consistent pseudonymization on all 10 digits.
///
/// Not reversible without salt.
pub fn mask_ec_cedula_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher) -> String {
    EC_CEDULA_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_cedula(&caps[0]) {
                return caps[0].to_string();
            }
            match hasher.encrypt(&caps[0]) {
                Ok(hashed) => hashed,
                Err(_)     => caps[0].to_string(),
            }
        })
        .into_owned()
}

/// Masks a valid Ecuadorian cédula using FF3-1 FPE on all 10 digits.
///
/// Reversible with the same key and tweak.
pub fn mask_ec_cedula_fpe(s: &str, cipher: &crate::patterns::fpe::Ff3Cipher) -> String {
    EC_CEDULA_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_cedula(&caps[0]) {
                return caps[0].to_string();
            }
            match cipher.encrypt(&caps[0]) {
                Ok(enc) => enc,
                Err(_)  => caps[0].to_string(),
            }
        })
        .into_owned()
}
