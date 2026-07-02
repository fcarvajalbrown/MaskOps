

use once_cell::sync::Lazy;
use regex::Regex;

static NPI_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{10})\b").unwrap()
});

fn valid_npi(s: &str) -> bool {
    let digits: Vec<u32> = s.chars().filter_map(|c| c.to_digit(10)).collect();
    if digits.len() != 10 {
        return false;
    }
    
    
    let sum: u32 = digits[..9].iter().rev().enumerate().map(|(i, &d)| {
        if i % 2 == 0 { 
            let v = d * 2;
            if v > 9 { v - 9 } else { v }
        } else {
            d
        }
    }).sum::<u32>() + 24;
    (10 - (sum % 10)) % 10 == digits[9]
}

pub fn extract_npi(s: &str) -> Option<String> {
    NPI_RE.find_iter(s).find(|m| valid_npi(m.as_str())).map(|m| m.as_str().to_string())
}

pub fn contains_npi(s: &str) -> bool {
    NPI_RE.find_iter(s).any(|m| valid_npi(m.as_str()))
}

pub fn mask_npi_counted(s: &str) -> (String, u32) {
    crate::patterns::replace_counted(&NPI_RE, s, |caps: &regex::Captures| {
        if !valid_npi(&caps[0]) {
            return None;
        }
        Some("*".repeat(10))
    })
}

pub fn mask_npi(s: &str) -> String {
    mask_npi_counted(s).0
}

pub fn mask_npi_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher, claims: &crate::patterns::TokenClaims) -> String {
    crate::patterns::mask_family(&NPI_RE, s, claims, &|t, _, _| valid_npi(t), &|d| hasher.encrypt(d).ok())
}

pub fn mask_npi_fpe(s: &str, cipher: &crate::patterns::fpe::FpeCipher, claims: &crate::patterns::TokenClaims) -> String {
    crate::patterns::mask_family(&NPI_RE, s, claims, &|t, _, _| valid_npi(t), &|d| cipher.encrypt(d).ok())
}
