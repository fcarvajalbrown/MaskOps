

use once_cell::sync::Lazy;
use regex::Regex;

static NHS_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{3}[ ]?\d{3}[ ]?\d{4})\b").unwrap()
});

fn valid_nhs(s: &str) -> bool {
    let digits: Vec<u32> = s.chars().filter_map(|c| c.to_digit(10)).collect();
    if digits.len() != 10 {
        return false;
    }
    let sum: u32 = digits[..9].iter().enumerate().map(|(i, &d)| d * (10 - i as u32)).sum();
    let remainder = sum % 11;
    if remainder == 1 {
        return false; 
    }
    let check = (11 - remainder) % 11;
    check == digits[9]
}

pub fn extract_nhs(s: &str) -> Option<String> {
    NHS_RE.find_iter(s).find(|m| valid_nhs(m.as_str())).map(|m| m.as_str().to_string())
}

pub fn contains_nhs(s: &str) -> bool {
    NHS_RE.find_iter(s).any(|m| valid_nhs(m.as_str()))
}

pub fn mask_nhs_counted(s: &str) -> (String, u32) {
    crate::patterns::replace_counted(&NHS_RE, s, |caps: &regex::Captures| {
        if !valid_nhs(&caps[0]) {
            return None;
        }
        Some(caps[0].chars().map(|c| if c.is_ascii_digit() { '*' } else { c }).collect())
    })
}

pub fn mask_nhs(s: &str) -> String {
    mask_nhs_counted(s).0
}

pub fn mask_nhs_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher, claims: &crate::patterns::TokenClaims) -> String {
    crate::patterns::mask_family(&NHS_RE, s, claims, &|t, _, _| valid_nhs(t), &|d| hasher.encrypt(d).ok())
}

pub fn mask_nhs_fpe(s: &str, cipher: &crate::patterns::fpe::FpeCipher, claims: &crate::patterns::TokenClaims) -> String {
    crate::patterns::mask_family(&NHS_RE, s, claims, &|t, _, _| valid_nhs(t), &|d| cipher.encrypt(d).ok())
}
