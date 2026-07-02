

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

pub fn contains_bsn(s: &str) -> bool {
    BSN_RE.find_iter(s).any(|m| valid_bsn(m.as_str()))
}

pub fn extract_bsn(s: &str) -> Option<String> {
    BSN_RE.find_iter(s).find(|m| valid_bsn(m.as_str())).map(|m| m.as_str().to_string())
}

pub fn mask_bsn_counted(s: &str) -> (String, u32) {
    crate::patterns::replace_counted(&BSN_RE, s, |caps: &regex::Captures| {
        if !valid_bsn(&caps[0]) {
            return None;
        }
        Some("*".repeat(9))
    })
}

pub fn mask_bsn(s: &str) -> String {
    mask_bsn_counted(s).0
}

pub fn mask_bsn_fpe(s: &str, cipher: &crate::patterns::fpe::FpeCipher, claims: &crate::patterns::TokenClaims) -> String {
    crate::patterns::mask_family(&BSN_RE, s, claims, &|t, _, _| valid_bsn(t), &|d| cipher.encrypt(d).ok())
}

pub fn mask_bsn_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher, claims: &crate::patterns::TokenClaims) -> String {
    crate::patterns::mask_family(&BSN_RE, s, claims, &|t, _, _| valid_bsn(t), &|d| hasher.encrypt(d).ok())
}
