//! European national ID detection and masking.
//!
//! Covers:
//!   - Spanish DNI (Documento Nacional de Identidad): 8 digits + letter
//!   - Spanish NIE (Número de Identidad de Extranjero): X/Y/Z + 7 digits + letter
//!   - UK NIN (National Insurance Number): AB 12 34 56 C format
//!   - German Personalausweis: 10 alphanumeric chars + check digit (format-only)
//!
//! DNI and NIE include modulo 23 check letter validation.
//! NIN includes prefix letter-pair rules per HMRC specification.
//! Personalausweis uses format-only matching — check digit validation is pending (v0.2.0+).

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

/// German Personalausweis: letter + 9 alphanumeric chars (format-only).
/// Real format: LXXXXXXXXX where L is a letter and X is alphanumeric.
/// Check digit validation deferred to v0.2.0.
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

/// Returns true if the input contains a UK NIN.
pub fn contains_nin(s: &str) -> bool {
    NIN_RE.is_match(s)
}

/// Masks any UK NIN found, preserving only the suffix letter.
///
/// Example: `AB 12 34 56 C` → `*********** C`
pub fn mask_nin(s: &str) -> String {
    NIN_RE
        .replace_all(s, |caps: &regex::Captures| {
            let nin = &caps[0];
            let suffix = &nin[nin.len() - 1..];
            format!("{}{}", "*".repeat(nin.len() - 1), suffix)
        })
        .into_owned()
}

/// Returns true if the input contains a German Personalausweis number.
pub fn contains_personalausweis(s: &str) -> bool {
    PA_RE.is_match(s)
}

/// Masks any German Personalausweis number found (full redaction).
///
/// Example: `T220001293` → `**********`
pub fn mask_personalausweis(s: &str) -> String {
    PA_RE
        .replace_all(s, |caps: &regex::Captures| {
            "*".repeat(caps[0].len())
        })
        .into_owned()
}