

use once_cell::sync::Lazy;
use regex::Regex;

static CNPJ_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{2}\.?\d{3}\.?\d{3}/?\d{4}-?\d{2})\b").unwrap()
});

const FIRST_WEIGHTS: [u32; 12] = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
const SECOND_WEIGHTS: [u32; 13] = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];

fn check_digit(digits: &[u32], weights: &[u32]) -> u32 {
    let sum: u32 = digits.iter().zip(weights.iter()).map(|(d, w)| d * w).sum();
    let remainder = sum % 11;
    if remainder < 2 {
        0
    } else {
        11 - remainder
    }
}

fn valid_cnpj(cnpj: &str) -> bool {
    let digits: Vec<u32> = cnpj
        .chars()
        .filter(|c| c.is_ascii_digit())
        .filter_map(|c| c.to_digit(10))
        .collect();

    if digits.len() != 14 {
        return false;
    }

    if digits.windows(2).all(|w| w[0] == w[1]) {
        return false;
    }

    let d1 = check_digit(&digits[..12], &FIRST_WEIGHTS);
    let d2 = check_digit(&digits[..13], &SECOND_WEIGHTS);

    digits[12] == d1 && digits[13] == d2
}

pub fn extract_cnpj(s: &str) -> Option<String> {
    CNPJ_RE.find_iter(s).find(|m| valid_cnpj(m.as_str())).map(|m| m.as_str().to_string())
}

pub fn contains_cnpj(s: &str) -> bool {
    CNPJ_RE.find_iter(s).any(|m| valid_cnpj(m.as_str()))
}

pub fn mask_cnpj_counted(s: &str) -> (String, u32) {
    crate::patterns::replace_counted(&CNPJ_RE, s, |caps: &regex::Captures| {
        let cnpj = &caps[0];
        if !valid_cnpj(cnpj) {
            return None;
        }
        let digits: String = cnpj.chars().filter(|c| c.is_ascii_digit()).collect();
        let check = &digits[12..];
        Some(format!("{}-{}", "*".repeat(12), check))
    })
}

pub fn mask_cnpj(s: &str) -> String {
    mask_cnpj_counted(s).0
}

pub fn mask_cnpj_fpe(s: &str, cipher: &crate::patterns::fpe::FpeCipher, claims: &crate::patterns::TokenClaims) -> String {
    crate::patterns::mask_family(&CNPJ_RE, s, claims, &|t, _, _| valid_cnpj(t), &|d| cipher.encrypt(d).ok())
}

pub fn mask_cnpj_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher, claims: &crate::patterns::TokenClaims) -> String {
    crate::patterns::mask_family(&CNPJ_RE, s, claims, &|t, _, _| valid_cnpj(t), &|d| hasher.encrypt(d).ok())
}
