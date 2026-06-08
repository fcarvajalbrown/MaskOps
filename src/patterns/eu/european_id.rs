

use once_cell::sync::Lazy;
use regex::Regex;

static DNI_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{8}[A-HJ-NP-TV-Z])\b").unwrap()
});

static NIE_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b([XYZ]\d{7}[A-HJ-NP-TV-Z])\b").unwrap()
});

static NIN_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"\b([A-CEGHJ-PR-TW-Z][A-CEGHJ-NPR-TW-Z]\s?\d{2}\s?\d{2}\s?\d{2}\s?[ABCD])\b"
    ).unwrap()
});

static PA_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b([A-Z][A-Z0-9]{8}[0-9])\b").unwrap()
});

const DNI_LETTERS: &[u8] = b"TRWAGMYFPDXBNJZSQVHLCKE";

fn valid_dni(dni: &str) -> bool {
    let digits: String = dni.chars().take(8).collect();
    let letter = match dni.chars().last() {
        Some(c) => c,
        None => return false,
    };
    let n: u32 = match digits.parse() {
        Ok(v) => v,
        Err(_) => return false,
    };
    let expected = DNI_LETTERS[(n % 23) as usize] as char;
    letter == expected
}

fn valid_nie(nie: &str) -> bool {
    let first = match nie.chars().next() {
        Some(c) => c,
        None => return false,
    };
    let prefix = match first {
        'X' => '0',
        'Y' => '1',
        'Z' => '2',
        _ => return false,
    };
    let normalized = format!("{}{}", prefix, &nie[1..]);
    valid_dni(&normalized)
}

const NIN_INVALID_PREFIXES: &[&str] = &["BG", "GB", "KN", "NK", "NT", "TN", "ZZ"];

fn valid_nin_prefix(nin: &str) -> bool {
    let prefix: String = nin.chars().filter(|c| c.is_ascii_alphabetic()).take(2).collect();
    !NIN_INVALID_PREFIXES.contains(&prefix.as_str())
}

fn pa_char_value(c: char) -> u32 {
    if c.is_ascii_digit() {
        c as u32 - b'0' as u32
    } else {
        c as u32 - b'A' as u32 + 10
    }
}

fn valid_personalausweis(id: &str) -> bool {
    if id.len() != 10 {
        return false;
    }
    let chars: Vec<char> = id.chars().collect();
    let weights = [7u32, 3, 1, 7, 3, 1, 7, 3, 1];
    let sum: u32 = chars[..9]
        .iter()
        .zip(weights.iter())
        .map(|(c, w)| pa_char_value(*c) * w)
        .sum();
    let check_digit = sum % 10;
    let last = chars[9] as u32 - b'0' as u32;
    check_digit == last
}

pub fn contains_dni(s: &str) -> bool {
    DNI_RE.find_iter(s).any(|m| valid_dni(m.as_str()))
}

pub fn mask_dni_counted(s: &str) -> (String, u32) {
    crate::patterns::replace_counted(&DNI_RE, s, |caps: &regex::Captures| {
        let dni = &caps[0];
        if !valid_dni(dni) {
            return None;
        }
        let letter = &dni[dni.len() - 1..];
        Some(format!("{}{}", "*".repeat(8), letter))
    })
}

pub fn mask_dni(s: &str) -> String {
    mask_dni_counted(s).0
}

pub fn contains_nie(s: &str) -> bool {
    NIE_RE.find_iter(s).any(|m| valid_nie(m.as_str()))
}

pub fn mask_nie_counted(s: &str) -> (String, u32) {
    crate::patterns::replace_counted(&NIE_RE, s, |caps: &regex::Captures| {
        let nie = &caps[0];
        if !valid_nie(nie) {
            return None;
        }
        let letter = &nie[nie.len() - 1..];
        Some(format!("{}{}", "*".repeat(nie.len() - 1), letter))
    })
}

pub fn mask_nie(s: &str) -> String {
    mask_nie_counted(s).0
}

pub fn contains_nin(s: &str) -> bool {
    NIN_RE.find_iter(s).any(|m| valid_nin_prefix(m.as_str()))
}

pub fn mask_nin_counted(s: &str) -> (String, u32) {
    crate::patterns::replace_counted(&NIN_RE, s, |caps: &regex::Captures| {
        let nin = &caps[0];
        if !valid_nin_prefix(nin) {
            return None;
        }
        let suffix = &nin[nin.len() - 1..];
        Some(format!("{}{}", "*".repeat(nin.len() - 1), suffix))
    })
}

pub fn mask_nin(s: &str) -> String {
    mask_nin_counted(s).0
}

pub fn contains_personalausweis(s: &str) -> bool {
    PA_RE.find_iter(s).any(|m| valid_personalausweis(m.as_str()))
}

pub fn mask_personalausweis_counted(s: &str) -> (String, u32) {
    crate::patterns::replace_counted(&PA_RE, s, |caps: &regex::Captures| {
        let id = &caps[0];
        if !valid_personalausweis(id) {
            return None;
        }
        Some("*".repeat(id.len()))
    })
}

pub fn mask_personalausweis(s: &str) -> String {
    mask_personalausweis_counted(s).0
}

/// Returns the first valid Spanish DNI found, or None.
pub fn extract_dni(s: &str) -> Option<String> {
    DNI_RE.find_iter(s).find(|m| valid_dni(m.as_str())).map(|m| m.as_str().to_string())
}

/// Returns the first valid Spanish NIE found, or None.
pub fn extract_nie(s: &str) -> Option<String> {
    NIE_RE.find_iter(s).find(|m| valid_nie(m.as_str())).map(|m| m.as_str().to_string())
}

/// Returns the first valid UK NIN found, or None.
pub fn extract_nin(s: &str) -> Option<String> {
    NIN_RE.find_iter(s).find(|m| valid_nin_prefix(m.as_str())).map(|m| m.as_str().to_string())
}

/// Returns the first valid German Personalausweis number found, or None.
pub fn extract_personalausweis(s: &str) -> Option<String> {
    PA_RE.find_iter(s).find(|m| valid_personalausweis(m.as_str())).map(|m| m.as_str().to_string())
}
