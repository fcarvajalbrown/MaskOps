

use once_cell::sync::Lazy;
use regex::Regex;

static SHORT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{6}[-+]\d{4})\b").unwrap()
});

static LONG_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{8}-\d{4})\b").unwrap()
});

fn luhn_valid(digits: &str) -> bool {
    if digits.len() != 10 {
        return false;
    }
    let sum: u32 = digits.chars().enumerate().map(|(i, c)| {
        let d = c as u32 - b'0' as u32;
        if i % 2 == 0 {
            let doubled = d * 2;
            if doubled > 9 { doubled - 9 } else { doubled }
        } else {
            d
        }
    }).sum();
    sum % 10 == 0
}

fn short_digits(raw: &str) -> String {
    raw.chars().filter(|c| c.is_ascii_digit()).collect()
}

fn long_digits(raw: &str) -> String {
    let all: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
    all[2..].to_string()
}

pub fn contains_personnummer(s: &str) -> bool {
    SHORT_RE.find_iter(s).any(|m| luhn_valid(&short_digits(m.as_str())))
        || LONG_RE.find_iter(s).any(|m| luhn_valid(&long_digits(m.as_str())))
}

pub fn extract_personnummer(s: &str) -> Option<String> {
    SHORT_RE.find_iter(s).find(|m| luhn_valid(&short_digits(m.as_str())))
        .or_else(|| LONG_RE.find_iter(s).find(|m| luhn_valid(&long_digits(m.as_str()))))
        .map(|m| m.as_str().to_string())
}

pub fn mask_personnummer_counted(s: &str) -> (String, u32) {
    let (s, n_short) = crate::patterns::replace_counted(&SHORT_RE, s, |caps: &regex::Captures| {
        let raw = &caps[0];
        if !luhn_valid(&short_digits(raw)) { return None; }
        Some(raw.chars().map(|c| if c.is_ascii_digit() { '*' } else { c }).collect())
    });
    let (s, n_long) = crate::patterns::replace_counted(&LONG_RE, &s, |caps: &regex::Captures| {
        let raw = &caps[0];
        if !luhn_valid(&long_digits(raw)) { return None; }
        Some(raw.chars().map(|c| if c.is_ascii_digit() { '*' } else { c }).collect())
    });
    (s, n_short + n_long)
}

pub fn mask_personnummer(s: &str) -> String {
    mask_personnummer_counted(s).0
}

pub fn mask_personnummer_fpe(s: &str, cipher: &crate::patterns::fpe::FpeCipher) -> String {
    let s = SHORT_RE
        .replace_all(s, |caps: &regex::Captures| {
            let raw = &caps[0];
            let sep = if raw.contains('+') { '+' } else { '-' };
            let d = short_digits(raw);
            if !luhn_valid(&d) { return raw.to_string(); }
            match cipher.encrypt(&d) {
                Ok(enc) => format!("{}{}{}", &enc[..6], sep, &enc[6..]),
                Err(_)  => raw.to_string(),
            }
        })
        .into_owned();
    LONG_RE
        .replace_all(&s, |caps: &regex::Captures| {
            let raw = &caps[0];
            let all: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
            if !luhn_valid(&long_digits(raw)) { return raw.to_string(); }
            match cipher.encrypt(&all) {
                Ok(enc) => format!("{}-{}", &enc[..8], &enc[8..]),
                Err(_)  => raw.to_string(),
            }
        })
        .into_owned()
}

pub fn mask_personnummer_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher) -> String {
    let s = SHORT_RE
        .replace_all(s, |caps: &regex::Captures| {
            let raw = &caps[0];
            let sep = if raw.contains('+') { '+' } else { '-' };
            let d = short_digits(raw);
            if !luhn_valid(&d) { return raw.to_string(); }
            match hasher.encrypt(&d) {
                Ok(hashed) => format!("{}{}{}", &hashed[..6], sep, &hashed[6..]),
                Err(_)     => raw.to_string(),
            }
        })
        .into_owned();
    LONG_RE
        .replace_all(&s, |caps: &regex::Captures| {
            let raw = &caps[0];
            let all: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
            if !luhn_valid(&long_digits(raw)) { return raw.to_string(); }
            match hasher.encrypt(&all) {
                Ok(hashed) => format!("{}-{}", &hashed[..8], &hashed[8..]),
                Err(_)     => raw.to_string(),
            }
        })
        .into_owned()
}
