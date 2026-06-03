//! European national ID detection and masking.
//!
//! Covers:
//!   - Spanish DNI (Documento Nacional de Identidad): 8 digits + letter
//!   - Spanish NIE (Número de Identidad de Extranjero): X/Y/Z + 7 digits + letter
//!   - UK NIN (National Insurance Number): AB 12 34 56 C format
//!   - German Personalausweis: 10 alphanumeric chars + check digit
//!
//! DNI and NIE include modulo 23 check letter validation.
//! NIN enforces prefix letter-pair rules + HMRC-excluded prefix pairs.
//! Personalausweis uses weighted-sum check digit (weights 7, 3, 1 cyclical).

use once_cell::sync::Lazy;
use regex::Regex;

// ── Regexes ───────────────────────────────────────────────────────────────────

/// Spanish DNI: 8 digits followed by a single uppercase letter.
static DNI_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{8}[A-HJ-NP-TV-Z])\b").unwrap()
});

/// Spanish NIE: X, Y, or Z + 7 digits + check letter.
static NIE_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b([XYZ]\d{7}[A-HJ-NP-TV-Z])\b").unwrap()
});

/// UK National Insurance Number.
/// Prefix rules: first letter not D/F/I/Q/U/V, second letter not D/F/I/O/Q/U/V.
/// Suffix: A, B, C, or D only.
/// Optional spaces between groups.
static NIN_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"\b([A-CEGHJ-PR-TW-Z][A-CEGHJ-NPR-TW-Z]\s?\d{2}\s?\d{2}\s?\d{2}\s?[ABCD])\b"
    ).unwrap()
});

/// German Personalausweis: letter + 8 alphanumeric + 1 check digit.
static PA_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b([A-Z][A-Z0-9]{8}[0-9])\b").unwrap()
});

// ── Validation ────────────────────────────────────────────────────────────────

/// DNI/NIE check letter lookup table (modulo 23).
const DNI_LETTERS: &[u8] = b"TRWAGMYFPDXBNJZSQVHLCKE";

/// Validates the check letter of a Spanish DNI using modulo 23.
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

/// Validates the check letter of a Spanish NIE using modulo 23.
/// X → 0, Y → 1, Z → 2, then same modulo 23 as DNI.
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

/// Prefix pairs excluded by HMRC regardless of individual letter rules.
const NIN_INVALID_PREFIXES: &[&str] = &["BG", "GB", "KN", "NK", "NT", "TN", "ZZ"];

/// Returns true if the NIN prefix pair is HMRC-valid.
fn valid_nin_prefix(nin: &str) -> bool {
    let prefix: String = nin.chars().filter(|c| c.is_ascii_alphabetic()).take(2).collect();
    !NIN_INVALID_PREFIXES.contains(&prefix.as_str())
}

/// Converts an alphanumeric Personalausweis character to its numeric value.
/// 0-9 → 0-9, A-Z → 10-35.
fn pa_char_value(c: char) -> u32 {
    if c.is_ascii_digit() {
        c as u32 - b'0' as u32
    } else {
        c as u32 - b'A' as u32 + 10
    }
}

/// Validates a German Personalausweis number using the weighted-sum check digit.
/// Weights [7, 3, 1] applied cyclically to positions 1-9; result mod 10 = position 10.
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

// ── Public API ────────────────────────────────────────────────────────────────

/// Returns true if the input contains a valid Spanish DNI.
pub fn contains_dni(s: &str) -> bool {
    DNI_RE.find_iter(s).any(|m| valid_dni(m.as_str()))
}

/// Masks the body of any valid DNI found, preserving the check letter.
///
/// Example: `12345678Z` → `********Z`
pub fn mask_dni(s: &str) -> String {
    DNI_RE
        .replace_all(s, |caps: &regex::Captures| {
            let dni = &caps[0];
            if !valid_dni(dni) {
                return dni.to_string();
            }
            let letter = &dni[dni.len() - 1..];
            format!("{}{}", "*".repeat(8), letter)
        })
        .into_owned()
}

/// Returns true if the input contains a valid Spanish NIE.
pub fn contains_nie(s: &str) -> bool {
    NIE_RE.find_iter(s).any(|m| valid_nie(m.as_str()))
}

/// Masks the body of any valid NIE found, preserving the check letter.
///
/// Example: `X1234567L` → `*********L` (full redaction — prefix reveals nationality)
pub fn mask_nie(s: &str) -> String {
    NIE_RE
        .replace_all(s, |caps: &regex::Captures| {
            let nie = &caps[0];
            if !valid_nie(nie) {
                return nie.to_string();
            }
            let letter = &nie[nie.len() - 1..];
            format!("{}{}", "*".repeat(nie.len() - 1), letter)
        })
        .into_owned()
}

/// Returns true if the input contains a valid UK NIN.
pub fn contains_nin(s: &str) -> bool {
    NIN_RE.find_iter(s).any(|m| valid_nin_prefix(m.as_str()))
}

/// Masks any UK NIN found, preserving only the suffix letter.
///
/// Example: `AB 12 34 56 C` → `*********** C`
pub fn mask_nin(s: &str) -> String {
    NIN_RE
        .replace_all(s, |caps: &regex::Captures| {
            let nin = &caps[0];
            if !valid_nin_prefix(nin) {
                return nin.to_string();
            }
            let suffix = &nin[nin.len() - 1..];
            format!("{}{}", "*".repeat(nin.len() - 1), suffix)
        })
        .into_owned()
}

/// Returns true if the input contains a valid German Personalausweis number.
pub fn contains_personalausweis(s: &str) -> bool {
    PA_RE.find_iter(s).any(|m| valid_personalausweis(m.as_str()))
}

/// Masks any German Personalausweis number found (full redaction).
///
/// Example: `T220001293` → `**********`
pub fn mask_personalausweis(s: &str) -> String {
    PA_RE
        .replace_all(s, |caps: &regex::Captures| {
            let id = &caps[0];
            if !valid_personalausweis(id) {
                return id.to_string();
            }
            "*".repeat(id.len())
        })
        .into_owned()
}
