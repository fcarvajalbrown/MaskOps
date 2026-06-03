//! French NIR (Numéro d'Inscription au Répertoire) — INSEE social security number.
//!
//! Format: 15 chars — sex(1) + year(2) + month(2) + dept(2) + commune(3) + order(3) + key(2).
//! Dept may be 2A or 2B (Corsica); all other positions are digits.
//! Check key: 97 − (first-13-char numeric value mod 97), zero-padded to 2 digits.
//!
//! Compliance: French Loi informatique et libertés (transposition of GDPR).
//! Non-digit PII (alphanumeric Corsica variant) — asterisk only, no FPE/consistent.
//! GDPR Art. 9: health-linked identifier (INSEE is used in healthcare records).

use once_cell::sync::Lazy;
use regex::Regex;

/// Compact NIR (no spaces): sex(1) + YY(2) + MM(2) + dept(2|2A|2B) + commune(3) + order(3) + key(2).
static NIR_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"\b([12]\d{2}(?:0[1-9]|1[0-2]|20)(?:\d{2}|2[AB])\d{6}\d{2})\b"
    ).unwrap()
});

fn valid_nir(nir: &str) -> bool {
    if nir.len() != 15 {
        return false;
    }
    let body = &nir[..13];
    let key_str = &nir[13..];
    let numeric = body.replace("2A", "19").replace("2B", "18");
    let n: u64 = match numeric.parse() {
        Ok(v) => v,
        Err(_) => return false,
    };
    let expected_key = 97 - (n % 97);
    let parsed_key: u64 = match key_str.parse() {
        Ok(v) => v,
        Err(_) => return false,
    };
    parsed_key == expected_key && parsed_key >= 1 && parsed_key <= 97
}

/// Returns true if the input contains a valid French NIR.
pub fn contains_nir(s: &str) -> bool {
    NIR_RE.find_iter(s).any(|m| valid_nir(m.as_str()))
}

/// Masks any valid French NIR found (full redaction with asterisks).
///
/// Example: `185037505600181` → `***************`
pub fn mask_nir(s: &str) -> String {
    NIR_RE
        .replace_all(s, |caps: &regex::Captures| {
            let nir = &caps[0];
            if !valid_nir(nir) {
                return nir.to_string();
            }
            "*".repeat(nir.len())
        })
        .into_owned()
}
