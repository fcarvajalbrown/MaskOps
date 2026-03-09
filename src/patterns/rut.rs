//! Chilean RUT (Rol Único Tributario) detection and masking.
//!
//! Format: 12.345.678-9 or 12345678-9
//! Check digit: Módulo 11 (0-9 or K)
//!
//! Examples:
//!   12.345.678-9  →  ********-9
//!   7654321-K     →  *******-K

use once_cell::sync::Lazy;
use regex::Regex;

static RUT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{1,2}\.?\d{3}\.?\d{3}-[\dKk])\b").unwrap()
});

/// Validates RUT check digit using Módulo 11.
fn valid_rut(rut: &str) -> bool {
    let clean: String = rut.chars().filter(|c| c.is_alphanumeric()).collect();
    if clean.len() < 2 {
        return false;
    }
    let (body, dv) = clean.split_at(clean.len() - 1);
    let dv = dv.to_uppercase();

    let digits: Vec<u32> = body.chars().rev()
        .filter_map(|c| c.to_digit(10))
        .collect();

    let factors = [2, 3, 4, 5, 6, 7];
    let sum: u32 = digits.iter().enumerate()
        .map(|(i, &d)| d * factors[i % 6])
        .sum();

    let remainder = 11 - (sum % 11);
    let expected = match remainder {
        11 => "0".to_string(),
        10 => "K".to_string(),
        n  => n.to_string(),
    };

    dv == expected
}

/// Returns true if the input contains a valid RUT.
pub fn contains_rut(s: &str) -> bool {
    RUT_RE.find_iter(s).any(|m| valid_rut(m.as_str()))
}

/// Masks the body of any valid RUT found in the input, preserving the check digit.
pub fn mask_rut(s: &str) -> String {
    RUT_RE.replace_all(s, |caps: &regex::Captures| {
        let rut = &caps[0];
        if !valid_rut(rut) {
            return rut.to_string();
        }
        let dv = &rut[rut.len()-1..];
        let body_len = rut.len() - 2; // exclude dash and check digit
        format!("{}-{}", "*".repeat(body_len), dv)
    }).into_owned()
}