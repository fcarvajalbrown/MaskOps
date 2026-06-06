

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

pub fn contains_nhs(s: &str) -> bool {
    NHS_RE.find_iter(s).any(|m| valid_nhs(m.as_str()))
}

pub fn mask_nhs(s: &str) -> String {
    NHS_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_nhs(&caps[0]) {
                return caps[0].to_string();
            }
            caps[0].chars().map(|c| if c.is_ascii_digit() { '*' } else { c }).collect()
        })
        .into_owned()
}

pub fn mask_nhs_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher) -> String {
    NHS_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_nhs(&caps[0]) {
                return caps[0].to_string();
            }
            let digits: String = caps[0].chars().filter(|c| c.is_ascii_digit()).collect();
            match hasher.encrypt(&digits) {
                Ok(hashed) => hashed,
                Err(_)     => caps[0].to_string(),
            }
        })
        .into_owned()
}

pub fn mask_nhs_fpe(s: &str, cipher: &crate::patterns::fpe::Ff3Cipher) -> String {
    NHS_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_nhs(&caps[0]) {
                return caps[0].to_string();
            }
            let digits: String = caps[0].chars().filter(|c| c.is_ascii_digit()).collect();
            match cipher.encrypt(&digits) {
                Ok(enc) => enc,
                Err(_)  => caps[0].to_string(),
            }
        })
        .into_owned()
}
