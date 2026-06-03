//! Italian Codice Fiscale (fiscal code) detection and masking.
//!
//! Format: 16 alphanumeric chars.
//! Structure: surname(3L) + name(3L) + year(2D) + month(1L) + day-sex(2D) + municipality(1L+3D) + check(1L).
//! Check character: position-dependent lookup tables, sum mod 26 → letter.
//!
//! Compliance: Italian GDPR implementation (D.Lgs. 101/2018), Codice Privacy.
//! Non-digit PII (alphanumeric) — asterisk only, no FPE/consistent.
//! GDPR Art. 4(1): direct personal identifier.

use once_cell::sync::Lazy;
use regex::Regex;

static CF_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b([A-Z]{6}\d{2}[A-Z]\d{2}[A-Z]\d{3}[A-Z])\b").unwrap()
});

// Values for characters at odd positions (1-indexed), i.e. 0-indexed even positions.
// Index 0-9: digits 0-9. Index 10-35: letters A-Z.
const ODD_VALUES: [u32; 36] = [
    // digits 0-9
    1, 0, 5, 7, 9, 13, 15, 17, 19, 21,
    // letters A-Z
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
    // 0-indexed even positions (0,2,4,...,14) = odd 1-indexed → ODD_VALUES.
    // 0-indexed odd positions (1,3,5,...,13) = even 1-indexed → direct value.
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

/// Returns true if the input contains a valid Italian Codice Fiscale.
pub fn contains_cf(s: &str) -> bool {
    CF_RE.find_iter(s).any(|m| valid_cf(m.as_str()))
}

/// Masks any valid Codice Fiscale found (full redaction with asterisks).
///
/// Example: `RSSMRA80A01H501U` → `****************`
pub fn mask_cf(s: &str) -> String {
    CF_RE
        .replace_all(s, |caps: &regex::Captures| {
            let cf = &caps[0];
            if !valid_cf(cf) {
                return cf.to_string();
            }
            "*".repeat(16)
        })
        .into_owned()
}
