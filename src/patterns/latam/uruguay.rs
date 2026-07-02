

use once_cell::sync::Lazy;
use regex::Regex;

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

pub fn extract_uy_ci(s: &str) -> Option<String> {
    UY_CI_RE.find_iter(s).find(|m| valid_uy_ci(m.as_str())).map(|m| m.as_str().to_string())
}

pub fn contains_uy_ci(s: &str) -> bool {
    UY_CI_RE.find_iter(s).any(|m| valid_uy_ci(m.as_str()))
}

pub fn mask_uy_ci_counted(s: &str) -> (String, u32) {
    crate::patterns::replace_counted(&UY_CI_RE, s, |caps: &regex::Captures| {
        if !valid_uy_ci(&caps[0]) {
            return None;
        }
        Some("*".repeat(caps[0].len()))
    })
}

pub fn mask_uy_ci(s: &str) -> String {
    mask_uy_ci_counted(s).0
}

pub fn mask_uy_ci_fpe(s: &str, cipher: &crate::patterns::fpe::FpeCipher, claims: &crate::patterns::TokenClaims) -> String {
    crate::patterns::mask_family(&UY_CI_RE, s, claims, &|t, _, _| valid_uy_ci(t), &|d| cipher.encrypt(d).ok())
}

pub fn mask_uy_ci_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher, claims: &crate::patterns::TokenClaims) -> String {
    crate::patterns::mask_family(&UY_CI_RE, s, claims, &|t, _, _| valid_uy_ci(t), &|d| hasher.encrypt(d).ok())
}
