

use once_cell::sync::Lazy;
use regex::Regex;

static SIN_FMT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b([1-9]\d{2}-\d{3}-\d{3})\b").unwrap()
});

static SIN_COMPACT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b([1-9]\d{8})\b").unwrap()
});

fn luhn_valid(digits: &str) -> bool {
    if digits.len() != 9 {
        return false;
    }
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

fn not_followed_by_dash(s: &str, end: usize) -> bool {
    s.as_bytes().get(end) != Some(&b'-')
}

pub fn extract_sin(s: &str) -> Option<String> {
    SIN_FMT_RE.find_iter(s)
        .find(|m| {
            let d: String = m.as_str().chars().filter(|c| c.is_ascii_digit()).collect();
            luhn_valid(&d)
        })
        .map(|m| m.as_str().to_string())
        .or_else(|| {
            SIN_COMPACT_RE.find_iter(s)
                .find(|m| luhn_valid(m.as_str()) && not_followed_by_dash(s, m.end()))
                .map(|m| m.as_str().to_string())
        })
}

pub fn contains_sin(s: &str) -> bool {
    SIN_FMT_RE.find_iter(s).any(|m| {
        let d: String = m.as_str().chars().filter(|c| c.is_ascii_digit()).collect();
        luhn_valid(&d)
    }) || SIN_COMPACT_RE.find_iter(s).any(|m| {
        luhn_valid(m.as_str()) && not_followed_by_dash(s, m.end())
    })
}

pub fn mask_sin_counted(s: &str) -> (String, u32) {
    let (s, n_fmt) = crate::patterns::replace_counted(&SIN_FMT_RE, s, |caps: &regex::Captures| {
        let raw = &caps[0];
        let d: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
        if !luhn_valid(&d) { return None; }
        Some("*".repeat(raw.len()))
    });
    let (s, n_compact) = mask_sin_compact_asterisk_counted(&s);
    (s, n_fmt + n_compact)
}

pub fn mask_sin(s: &str) -> String {
    mask_sin_counted(s).0
}

fn mask_sin_compact_asterisk_counted(s: &str) -> (String, u32) {
    let mut result = String::with_capacity(s.len());
    let mut last = 0;
    let mut count = 0u32;
    for m in SIN_COMPACT_RE.find_iter(s) {
        if !luhn_valid(m.as_str()) || !not_followed_by_dash(s, m.end()) {
            result.push_str(&s[last..m.end()]);
            last = m.end();
            continue;
        }
        result.push_str(&s[last..m.start()]);
        result.push_str(&"*".repeat(9));
        count += 1;
        last = m.end();
    }
    result.push_str(&s[last..]);
    (result, count)
}

pub fn mask_sin_fpe(s: &str, cipher: &crate::patterns::fpe::FpeCipher) -> String {
    let s = SIN_FMT_RE
        .replace_all(s, |caps: &regex::Captures| {
            let raw = &caps[0];
            let d: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
            if !luhn_valid(&d) { return raw.to_string(); }
            match cipher.encrypt(&d) {
                Ok(enc) => format!("{}-{}-{}", &enc[..3], &enc[3..6], &enc[6..]),
                Err(_) => raw.to_string(),
            }
        })
        .into_owned();
    mask_sin_compact_fpe(&s, cipher)
}

fn mask_sin_compact_fpe(s: &str, cipher: &crate::patterns::fpe::FpeCipher) -> String {
    let mut result = String::with_capacity(s.len());
    let mut last = 0;
    for m in SIN_COMPACT_RE.find_iter(s) {
        if !luhn_valid(m.as_str()) || !not_followed_by_dash(s, m.end()) {
            result.push_str(&s[last..m.end()]);
            last = m.end();
            continue;
        }
        result.push_str(&s[last..m.start()]);
        match cipher.encrypt(m.as_str()) {
            Ok(enc) => result.push_str(&enc),
            Err(_)  => result.push_str(m.as_str()),
        }
        last = m.end();
    }
    result.push_str(&s[last..]);
    result
}

pub fn mask_sin_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher) -> String {
    let s = SIN_FMT_RE
        .replace_all(s, |caps: &regex::Captures| {
            let raw = &caps[0];
            let d: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
            if !luhn_valid(&d) { return raw.to_string(); }
            match hasher.encrypt(&d) {
                Ok(hashed) => format!("{}-{}-{}", &hashed[..3], &hashed[3..6], &hashed[6..]),
                Err(_) => raw.to_string(),
            }
        })
        .into_owned();
    mask_sin_compact_consistent(&s, hasher)
}

fn mask_sin_compact_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher) -> String {
    let mut result = String::with_capacity(s.len());
    let mut last = 0;
    for m in SIN_COMPACT_RE.find_iter(s) {
        if !luhn_valid(m.as_str()) || !not_followed_by_dash(s, m.end()) {
            result.push_str(&s[last..m.end()]);
            last = m.end();
            continue;
        }
        result.push_str(&s[last..m.start()]);
        match hasher.encrypt(m.as_str()) {
            Ok(hashed) => result.push_str(&hashed),
            Err(_)     => result.push_str(m.as_str()),
        }
        last = m.end();
    }
    result.push_str(&s[last..]);
    result
}
