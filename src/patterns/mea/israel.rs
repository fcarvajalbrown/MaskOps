use once_cell::sync::Lazy;
use regex::Regex;

use crate::patterns::mea::boundary_ok;

static IL_ID_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{9})\b").unwrap()
});

fn valid_il_id(digits: &str) -> bool {
    if digits.len() != 9 {
        return false;
    }
    let mut sum = 0u32;
    for (i, c) in digits.chars().enumerate() {
        let d = c as u32 - b'0' as u32;
        let weight = if i % 2 == 0 { 1 } else { 2 };
        let mut p = d * weight;
        if p > 9 {
            p -= 9;
        }
        sum += p;
    }
    sum % 10 == 0
}

fn matches(s: &str, m: &regex::Match) -> bool {
    valid_il_id(m.as_str()) && boundary_ok(s, m.start(), m.end())
}

pub fn extract_il_id(s: &str) -> Option<String> {
    IL_ID_RE
        .find_iter(s)
        .find(|m| matches(s, m))
        .map(|m| m.as_str().to_string())
}

pub fn contains_il_id(s: &str) -> bool {
    IL_ID_RE.find_iter(s).any(|m| matches(s, &m))
}

fn transform<F>(s: &str, render: F) -> (String, u32)
where
    F: Fn(&str) -> Option<String>,
{
    let mut result = String::with_capacity(s.len());
    let mut last = 0;
    let mut count = 0u32;
    for m in IL_ID_RE.find_iter(s) {
        if !matches(s, &m) {
            continue;
        }
        match render(m.as_str()) {
            Some(rep) => {
                result.push_str(&s[last..m.start()]);
                result.push_str(&rep);
                count += 1;
                last = m.end();
            }
            None => {}
        }
    }
    result.push_str(&s[last..]);
    (result, count)
}

pub fn mask_il_id_counted(s: &str) -> (String, u32) {
    transform(s, |_| Some("*".repeat(9)))
}

pub fn mask_il_id(s: &str) -> String {
    mask_il_id_counted(s).0
}

pub fn mask_il_id_fpe(s: &str, cipher: &crate::patterns::fpe::FpeCipher) -> String {
    transform(s, |d| Some(cipher.encrypt(d).unwrap_or_else(|_| d.to_string()))).0
}

pub fn mask_il_id_consistent(
    s: &str,
    hasher: &crate::patterns::consistent::ConsistentHasher,
) -> String {
    transform(s, |d| Some(hasher.encrypt(d).unwrap_or_else(|_| d.to_string()))).0
}
