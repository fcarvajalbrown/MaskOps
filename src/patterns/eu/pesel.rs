

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

pub fn mask_pesel(s: &str) -> String {
    PESEL_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_pesel(&caps[0]) {
                return caps[0].to_string();
            }
            "*".repeat(11)
        })
        .into_owned()
}

pub fn mask_pesel_fpe(s: &str, cipher: &crate::patterns::fpe::Ff3Cipher) -> String {
    PESEL_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_pesel(&caps[0]) {
                return caps[0].to_string();
            }
            match cipher.encrypt(&caps[0]) {
                Ok(enc) => enc,
                Err(_)  => caps[0].to_string(),
            }
        })
        .into_owned()
}

pub fn mask_pesel_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher) -> String {
    PESEL_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_pesel(&caps[0]) {
                return caps[0].to_string();
            }
            match hasher.encrypt(&caps[0]) {
                Ok(hashed) => hashed,
                Err(_)     => caps[0].to_string(),
            }
        })
        .into_owned()
}
