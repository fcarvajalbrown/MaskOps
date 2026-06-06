

use once_cell::sync::Lazy;
use regex::Regex;

static TFN_SPACED_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{3} \d{3} \d{3})\b").unwrap()
});

static TFN_COMPACT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b([1-9]\d{8})\b").unwrap()
});

const TFN_WEIGHTS: [u32; 9] = [1, 4, 3, 7, 5, 8, 6, 9, 10];

fn valid_tfn(digits: &str) -> bool {
    if digits.len() != 9 {
        return false;
    }
    let sum: u32 = digits
        .chars()
        .zip(TFN_WEIGHTS.iter())
        .map(|(c, &w)| (c as u32 - b'0' as u32) * w)
        .sum();
    sum % 11 == 0
}

pub fn contains_tfn(s: &str) -> bool {
    TFN_SPACED_RE.find_iter(s).any(|m| {
        let d: String = m.as_str().chars().filter(|c| c.is_ascii_digit()).collect();
        valid_tfn(&d)
    }) || TFN_COMPACT_RE.find_iter(s).any(|m| valid_tfn(m.as_str()))
}

pub fn mask_tfn(s: &str) -> String {
    let s = TFN_SPACED_RE
        .replace_all(s, |caps: &regex::Captures| {
            let raw = &caps[0];
            let d: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
            if !valid_tfn(&d) { return raw.to_string(); }
            "*** *** ***".to_string()
        })
        .into_owned();
    TFN_COMPACT_RE
        .replace_all(&s, |caps: &regex::Captures| {
            if !valid_tfn(&caps[0]) { return caps[0].to_string(); }
            "*".repeat(9)
        })
        .into_owned()
}

pub fn mask_tfn_fpe(s: &str, cipher: &crate::patterns::fpe::Ff3Cipher) -> String {
    let s = TFN_SPACED_RE
        .replace_all(s, |caps: &regex::Captures| {
            let raw = &caps[0];
            let d: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
            if !valid_tfn(&d) { return raw.to_string(); }
            match cipher.encrypt(&d) {
                Ok(enc) => format!("{} {} {}", &enc[..3], &enc[3..6], &enc[6..]),
                Err(_) => raw.to_string(),
            }
        })
        .into_owned();
    TFN_COMPACT_RE
        .replace_all(&s, |caps: &regex::Captures| {
            if !valid_tfn(&caps[0]) { return caps[0].to_string(); }
            match cipher.encrypt(&caps[0]) {
                Ok(enc) => enc,
                Err(_) => caps[0].to_string(),
            }
        })
        .into_owned()
}

pub fn mask_tfn_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher) -> String {
    let s = TFN_SPACED_RE
        .replace_all(s, |caps: &regex::Captures| {
            let raw = &caps[0];
            let d: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
            if !valid_tfn(&d) { return raw.to_string(); }
            match hasher.encrypt(&d) {
                Ok(hashed) => format!("{} {} {}", &hashed[..3], &hashed[3..6], &hashed[6..]),
                Err(_) => raw.to_string(),
            }
        })
        .into_owned();
    TFN_COMPACT_RE
        .replace_all(&s, |caps: &regex::Captures| {
            if !valid_tfn(&caps[0]) { return caps[0].to_string(); }
            match hasher.encrypt(&caps[0]) {
                Ok(hashed) => hashed,
                Err(_) => caps[0].to_string(),
            }
        })
        .into_owned()
}
