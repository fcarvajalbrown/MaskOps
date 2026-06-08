

use once_cell::sync::Lazy;
use regex::Regex;

static CF_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b([A-Z]{6}\d{2}[A-Z]\d{2}[A-Z]\d{3}[A-Z])\b").unwrap()
});

const ODD_VALUES: [u32; 36] = [
    
    1, 0, 5, 7, 9, 13, 15, 17, 19, 21,
    
    1, 0, 5, 7, 9, 13, 15, 17, 19, 21, 2, 4, 18, 20, 11, 3, 6, 8, 12, 14, 16, 10, 22, 25, 24, 23,
];

fn cf_odd_value(c: char) -> u32 {
    let idx = if c.is_ascii_digit() {
        (c as u8 - b'0') as usize
    } else {
        (c as u8 - b'A') as usize + 10
    };
    ODD_VALUES[idx]
}

fn cf_even_value(c: char) -> u32 {
    if c.is_ascii_digit() {
        c as u32 - b'0' as u32
    } else {
        c as u32 - b'A' as u32
    }
}

fn valid_cf(cf: &str) -> bool {
    if cf.len() != 16 {
        return false;
    }
    let upper = cf.to_uppercase();
    let chars: Vec<char> = upper.chars().collect();
    
    
    let sum: u32 = chars[..15]
        .iter()
        .enumerate()
        .map(|(i, &c)| {
            if i % 2 == 0 {
                cf_odd_value(c)
            } else {
                cf_even_value(c)
            }
        })
        .sum();
    let expected = (b'A' + (sum % 26) as u8) as char;
    chars[15] == expected
}

pub fn contains_cf(s: &str) -> bool {
    CF_RE.find_iter(s).any(|m| valid_cf(m.as_str()))
}

pub fn extract_cf(s: &str) -> Option<String> {
    CF_RE.find_iter(s).find(|m| valid_cf(m.as_str())).map(|m| m.as_str().to_string())
}

pub fn mask_cf_counted(s: &str) -> (String, u32) {
    crate::patterns::replace_counted(&CF_RE, s, |caps: &regex::Captures| {
        let cf = &caps[0];
        if !valid_cf(cf) {
            return None;
        }
        Some("*".repeat(16))
    })
}

pub fn mask_cf(s: &str) -> String {
    mask_cf_counted(s).0
}
