

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

pub fn mask_npi(s: &str) -> String {
    NPI_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_npi(&caps[0]) {
                return caps[0].to_string();
            }
            "*".repeat(10)
        })
        .into_owned()
}

pub fn mask_npi_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher) -> String {
    NPI_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_npi(&caps[0]) {
                return caps[0].to_string();
            }
            match hasher.encrypt(&caps[0]) {
                Ok(hashed) => hashed,
                Err(_)     => caps[0].to_string(),
            }
        })
        .into_owned()
}

pub fn mask_npi_fpe(s: &str, cipher: &crate::patterns::fpe::Ff3Cipher) -> String {
    NPI_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_npi(&caps[0]) {
                return caps[0].to_string();
            }
            match cipher.encrypt(&caps[0]) {
                Ok(enc) => enc,
                Err(_)  => caps[0].to_string(),
            }
        })
        .into_owned()
}
