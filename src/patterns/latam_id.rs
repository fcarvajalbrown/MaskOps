//! Latin American national ID detection and masking.
//!
//! Covers:
//!   - Chilean RUT (Rol Único Tributario): 12.345.678-9 / 12345678-9
//!   - Brazilian CPF (Cadastro de Pessoas Físicas): 123.456.789-09 / 12345678909
//!   - Mexican CURP (Clave Única de Registro de Población): 18-char alphanumeric
//!
//! RUT and CPF include check-digit validation (Módulo 11).
//! CURP uses format-only matching — no check digit defined in the standard.

use once_cell::sync::Lazy;
use regex::Regex;

// ── Regexes ───────────────────────────────────────────────────────────────────

static RUT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{1,2}\.?\d{3}\.?\d{3}-[\dKk])\b").unwrap()
});

static CPF_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{3}\.?\d{3}\.?\d{3}-?\d{2})\b").unwrap()
});

static CURP_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b([A-Z][AEIOU][A-Z]{2}\d{6}[HM][A-Z]{2}[B-DF-HJ-NP-TV-Z]{3}[A-Z0-9]\d)\b")
        .unwrap()
});

// ── Validation ────────────────────────────────────────────────────────────────

/// Validates a Chilean RUT check digit using Módulo 11.
fn valid_rut(rut: &str) -> bool {
    let clean: String = rut.chars().filter(|c| c.is_alphanumeric()).collect();
    if clean.len() < 2 {
        return false;
    }
    let (body, dv) = clean.split_at(clean.len() - 1);
    let dv = dv.to_uppercase();

    let digits: Vec<u32> = body.chars().rev().filter_map(|c| c.to_digit(10)).collect();
    let factors = [2, 3, 4, 5, 6, 7];
    let sum: u32 = digits.iter().enumerate().map(|(i, &d)| d * factors[i % 6]).sum();

    let expected = match 11 - (sum % 11) {
        11 => "0".to_string(),
        10 => "K".to_string(),
        n => n.to_string(),
    };
    dv == expected
}

/// Validates a Brazilian CPF using Módulo 11 (two check digits).
fn valid_cpf(cpf: &str) -> bool {
    let digits: Vec<u32> = cpf
        .chars()
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

    let sum1: u32 = digits[..9].iter().enumerate().map(|(i, &d)| d * (10 - i as u32)).sum();
    let r1 = (sum1 * 10) % 11;
    let d1 = if r1 == 10 { 0 } else { r1 };

    let sum2: u32 = digits[..10].iter().enumerate().map(|(i, &d)| d * (11 - i as u32)).sum();
    let r2 = (sum2 * 10) % 11;
    let d2 = if r2 == 10 { 0 } else { r2 };

    digits[9] == d1 && digits[10] == d2
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Returns true if the input contains a valid Chilean RUT.
pub fn contains_rut(s: &str) -> bool {
    RUT_RE.find_iter(s).any(|m| valid_rut(m.as_str()))
}

/// Masks the body of any valid RUT found, preserving the check digit.
///
/// Example: `12.345.678-9` → `********-9`
pub fn mask_rut(s: &str) -> String {
    RUT_RE
        .replace_all(s, |caps: &regex::Captures| {
            let rut = &caps[0];
            if !valid_rut(rut) {
                return rut.to_string();
            }
            let dv = &rut[rut.len() - 1..];
            let body_len = rut.len() - 2; // exclude dash and check digit
            format!("{}-{}", "*".repeat(body_len), dv)
        })
        .into_owned()
}

/// Returns true if the input contains a valid Brazilian CPF.
pub fn contains_cpf(s: &str) -> bool {
    CPF_RE.find_iter(s).any(|m| valid_cpf(m.as_str()))
}

/// Masks the body of any valid CPF found, preserving the two check digits.
///
/// Example: `529.982.247-25` → `*********-25`
pub fn mask_cpf(s: &str) -> String {
    CPF_RE
        .replace_all(s, |caps: &regex::Captures| {
            let cpf = &caps[0];
            if !valid_cpf(cpf) {
                return cpf.to_string();
            }
            let digits: String = cpf.chars().filter(|c| c.is_ascii_digit()).collect();
            let check = &digits[9..];
            format!("{}-{}", "*".repeat(9), check)
        })
        .into_owned()
}

/// Returns true if the input contains a Mexican CURP.
pub fn contains_curp(s: &str) -> bool {
    CURP_RE.is_match(s)
}

/// Masks any CURP found in the input (full redaction — no visible portion).
///
/// Example: `BADD110313HCMLNS09` → `******************`
pub fn mask_curp(s: &str) -> String {
    CURP_RE
        .replace_all(s, |caps: &regex::Captures| "*".repeat(caps[0].len()))
        .into_owned()
}

/// Masks the digit body of a valid Chilean RUT using FF3-1 FPE.
///
/// The check digit is preserved (it's a single char, too short for FPE).
/// Separators are stripped — output is compact digits + check digit.
/// Reversible with the same key and tweak.
///
/// Example: `12.345.678-9` → `87263401-9`
pub fn mask_rut_fpe(s: &str, cipher: &crate::patterns::fpe::Ff3Cipher) -> String {
    RUT_RE
        .replace_all(s, |caps: &regex::Captures| {
            let rut = &caps[0];
            if !valid_rut(rut) {
                return rut.to_string();
            }
            let clean: String = rut.chars().filter(|c| c.is_alphanumeric()).collect();
            let body = &clean[..clean.len() - 1];
            let dv   = &clean[clean.len() - 1..];

            match cipher.encrypt(body) {
                Ok(encrypted) => format!("{}-{}", encrypted, dv),
                Err(_)        => rut.to_string(),
            }
        })
        .into_owned()
}

/// Masks the digit body of a valid Brazilian CPF using FF3-1 FPE.
///
/// All 11 digits are encrypted as a unit — separators stripped on output.
/// Reversible with the same key and tweak.
///
/// Example: `529.982.247-25` → `73614052891`
pub fn mask_cpf_fpe(s: &str, cipher: &crate::patterns::fpe::Ff3Cipher) -> String {
    CPF_RE
        .replace_all(s, |caps: &regex::Captures| {
            let cpf = &caps[0];
            if !valid_cpf(cpf) {
                return cpf.to_string();
            }
            let digits: String = cpf.chars().filter(|c| c.is_ascii_digit()).collect();

            match cipher.encrypt(&digits) {
                Ok(encrypted) => encrypted,
                Err(_)        => cpf.to_string(),
            }
        })
        .into_owned()
}