use once_cell::sync::Lazy;
use regex::Regex;

use crate::patterns::mea::boundary_ok;

static ZA_ID_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{13})\b").unwrap()
});

fn luhn_valid(digits: &str) -> bool {
    let sum: u32 = digits
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            let d = c as u32 - b'0' as u32;
            if i % 2 == 1 {
                let doubled = d * 2;
                if doubled > 9 { doubled - 9 } else { doubled }
            } else {
                d
            }
        })
        .sum();
    sum % 10 == 0
}

fn valid_za_id(digits: &str) -> bool {
    if digits.len() != 13 {
        return false;
    }
    let ds: Vec<u32> = digits.chars().map(|c| c as u32 - b'0' as u32).collect();
    let mm = ds[2] * 10 + ds[3];
    let dd = ds[4] * 10 + ds[5];
    if mm < 1 || mm > 12 || dd < 1 || dd > 31 {
        return false;
    }
    if ds[10] > 2 {
        return false;
    }
    luhn_valid(digits)
}

fn matches(s: &str, m: &regex::Match) -> bool {
    valid_za_id(m.as_str()) && boundary_ok(s, m.start(), m.end())
}

pub fn extract_za_id(s: &str) -> Option<String> {
    ZA_ID_RE
        .find_iter(s)
        .find(|m| matches(s, m))
        .map(|m| m.as_str().to_string())
}

pub fn contains_za_id(s: &str) -> bool {
    ZA_ID_RE.find_iter(s).any(|m| matches(s, &m))
}

fn transform<F>(s: &str, render: F) -> (String, u32)
where
    F: Fn(&str) -> Option<String>,
{
    let mut result = String::with_capacity(s.len());
    let mut last = 0;
    let mut count = 0u32;
    for m in ZA_ID_RE.find_iter(s) {
        if !matches(s, &m) {
            continue;
        }
        if let Some(rep) = render(m.as_str()) {
            result.push_str(&s[last..m.start()]);
            result.push_str(&rep);
            count += 1;
            last = m.end();
        }
    }
    result.push_str(&s[last..]);
    (result, count)
}

pub fn mask_za_id_counted(s: &str) -> (String, u32) {
    transform(s, |_| Some("*".repeat(13)))
}

pub fn mask_za_id(s: &str) -> String {
    mask_za_id_counted(s).0
}

pub fn mask_za_id_fpe(s: &str, cipher: &crate::patterns::fpe::FpeCipher, claims: &crate::patterns::TokenClaims) -> String {
    crate::patterns::mask_family(&ZA_ID_RE, s, claims,
        &|t, start, end| valid_za_id(t) && boundary_ok(s, start, end),
        &|d| cipher.encrypt(d).ok())
}

pub fn mask_za_id_consistent(
    s: &str,
    hasher: &crate::patterns::consistent::ConsistentHasher,
    claims: &crate::patterns::TokenClaims,
) -> String {
    crate::patterns::mask_family(&ZA_ID_RE, s, claims,
        &|t, start, end| valid_za_id(t) && boundary_ok(s, start, end),
        &|d| hasher.encrypt(d).ok())
}
