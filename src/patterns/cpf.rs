//! Brazilian CPF (Cadastro de Pessoas Físicas) detection and masking.
//!
//! Format: 123.456.789-09 or 12345678909
//! Check digits: Módulo 11 (two check digits)
//!
//! Examples:
//!   529.982.247-25  →  *********-25
//!   52998224725     →  *********25

use once_cell::sync::Lazy;
use regex::Regex;

static CPF_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{3}\.?\d{3}\.?\d{3}-?\d{2})\b").unwrap()
});

/// Validates CPF check digits using Módulo 11.
fn valid_cpf(cpf: &str) -> bool {
    let digits: Vec<u32> = cpf.chars()
        .filter(|c| c.is_ascii_digit())
        .filter_map(|c| c.to_digit(10))
        .collect();

    if digits.len() != 11 {
        return false;
    }

    // Reject all-same-digit CPFs (e.g. 111.111.111-11)
    if digits.windows(2).all(|w| w[0] == w[1]) {
        return false;
    }

    // First check digit
    let sum1: u32 = digits[..9].iter().enumerate()
        .map(|(i, &d)| d * (10 - i as u32))
        .sum();
    let r1 = (sum1 * 10) % 11;
    let d1 = if r1 == 10 { 0 } else { r1 };

    // Second check digit
    let sum2: u32 = digits[..10].iter().enumerate()
        .map(|(i, &d)| d * (11 - i as u32))
        .sum();
    let r2 = (sum2 * 10) % 11;
    let d2 = if r2 == 10 { 0 } else { r2 };

    digits[9] == d1 && digits[10] == d2
}

/// Returns true if the input contains a valid CPF.
pub fn contains_cpf(s: &str) -> bool {
    CPF_RE.find_iter(s).any(|m| valid_cpf(m.as_str()))
}

/// Masks the body of any valid CPF found, preserving the two check digits.
pub fn mask_cpf(s: &str) -> String {
    CPF_RE.replace_all(s, |caps: &regex::Captures| {
        let cpf = &caps[0];
        if !valid_cpf(cpf) {
            return cpf.to_string();
        }
        let digits: String = cpf.chars().filter(|c| c.is_ascii_digit()).collect();
        let check = &digits[9..];
        format!("{}-{}", "*".repeat(9), check)
    }).into_owned()
}