

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

pub fn contains_pesel(s: &str) -> bool {
    PESEL_RE.find_iter(s).any(|m| valid_pesel(m.as_str()))
}

pub fn extract_pesel(s: &str) -> Option<String> {
    PESEL_RE.find_iter(s).find(|m| valid_pesel(m.as_str())).map(|m| m.as_str().to_string())
}

pub fn mask_pesel_counted(s: &str) -> (String, u32) {
    crate::patterns::replace_counted(&PESEL_RE, s, |caps: &regex::Captures| {
        if !valid_pesel(&caps[0]) {
            return None;
        }
        Some("*".repeat(11))
    })
}

pub fn mask_pesel(s: &str) -> String {
    mask_pesel_counted(s).0
}

pub fn mask_pesel_fpe(s: &str, cipher: &crate::patterns::fpe::FpeCipher, claims: &crate::patterns::TokenClaims) -> String {
    crate::patterns::mask_family(&PESEL_RE, s, claims, &|t, _, _| valid_pesel(t), &|d| cipher.encrypt(d).ok())
}

pub fn mask_pesel_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher, claims: &crate::patterns::TokenClaims) -> String {
    crate::patterns::mask_family(&PESEL_RE, s, claims, &|t, _, _| valid_pesel(t), &|d| hasher.encrypt(d).ok())
}
