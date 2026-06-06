

use once_cell::sync::Lazy;
use regex::Regex;

static EC_CEDULA_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{10})\b").unwrap()
});

fn valid_province(d0: u32, d1: u32) -> bool {
    let province = d0 * 10 + d1;
    province >= 1 && province <= 24
}

fn valid_cedula(s: &str) -> bool {
    let digits: Vec<u32> = s.chars().filter_map(|c| c.to_digit(10)).collect();
    if digits.len() != 10 {
        return false;
    }
    if !valid_province(digits[0], digits[1]) {
        return false;
    }
    
    if digits[2] >= 6 {
        return false;
    }
    let sum: u32 = digits[..9].iter().enumerate().map(|(i, &d)| {
        if i % 2 == 0 {
            let v = d * 2;
            if v >= 10 { v - 9 } else { v }
        } else {
            d
        }
    }).sum();
    let check = (10 - (sum % 10)) % 10;
    check == digits[9]
}

pub fn extract_ec_cedula(s: &str) -> Option<String> {
    EC_CEDULA_RE.find_iter(s).find(|m| valid_cedula(m.as_str())).map(|m| m.as_str().to_string())
}

pub fn contains_ec_cedula(s: &str) -> bool {
    EC_CEDULA_RE.find_iter(s).any(|m| valid_cedula(m.as_str()))
}

pub fn mask_ec_cedula(s: &str) -> String {
    EC_CEDULA_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_cedula(&caps[0]) {
                return caps[0].to_string();
            }
            "*".repeat(10)
        })
        .into_owned()
}

pub fn mask_ec_cedula_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher) -> String {
    EC_CEDULA_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_cedula(&caps[0]) {
                return caps[0].to_string();
            }
            match hasher.encrypt(&caps[0]) {
                Ok(hashed) => hashed,
                Err(_)     => caps[0].to_string(),
            }
        })
        .into_owned()
}

pub fn mask_ec_cedula_fpe(s: &str, cipher: &crate::patterns::fpe::Ff3Cipher) -> String {
    EC_CEDULA_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_cedula(&caps[0]) {
                return caps[0].to_string();
            }
            match cipher.encrypt(&caps[0]) {
                Ok(enc) => enc,
                Err(_)  => caps[0].to_string(),
            }
        })
        .into_owned()
}
