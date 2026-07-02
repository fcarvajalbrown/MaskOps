

use once_cell::sync::Lazy;
use regex::Regex;

pub static CARD_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?x)
        \b(?:
            # Amex: 34xx or 37xx prefix, 15 digits total
            3[47][0-9]{2}[\s\-]?[0-9]{6}[\s\-]?[0-9]{5}
            |
            # Visa: 4xxx prefix, 16 digits total
            4[0-9]{3}[\s\-]?[0-9]{4}[\s\-]?[0-9]{4}[\s\-]?[0-9]{4}
            |
            # Mastercard: 51-55 or 2221-2720 prefix, 16 digits total
            (?:5[1-5][0-9]{2}|2(?:2[2-9][1-9]|[3-6][0-9]{2}|7[01][0-9]|720))
            [\s\-]?[0-9]{4}[\s\-]?[0-9]{4}[\s\-]?[0-9]{4}
            |
            # Discover: 6011, 65xx, 644-649 prefix, 16 digits total
            (?:6011|65[0-9]{2}|64[4-9][0-9])[\s\-]?[0-9]{4}[\s\-]?[0-9]{4}[\s\-]?[0-9]{4}
            |
            # Maestro: 6304, 6759, 6761-6763 prefix, 16 digits total
            (?:6304|6759|676[1-3])[0-9]{12}
        )\b
    ").unwrap()
});

fn luhn_valid(card: &str) -> bool {
    let digits: Vec<u32> = card
        .chars()
        .filter(|c| c.is_ascii_digit())
        .filter_map(|c| c.to_digit(10))
        .collect();

    if digits.len() < 12 {
        return false;
    }

    let sum: u32 = digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &d)| {
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

pub fn extract_card(s: &str) -> Option<String> {
    CARD_RE.find_iter(s).find(|m| luhn_valid(m.as_str())).map(|m| m.as_str().to_string())
}

pub fn contains_card(s: &str) -> bool {
    CARD_RE.find_iter(s).any(|m| luhn_valid(m.as_str()))
}

pub fn mask_card_counted(s: &str) -> (String, u32) {
    crate::patterns::replace_counted(&CARD_RE, s, |caps: &regex::Captures| {
        let raw = caps.get(0).unwrap().as_str();
        if !luhn_valid(raw) {
            return None;
        }
        let digits: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
        let len = digits.len();
        let bin = &digits[..6];
        let last4 = &digits[len - 4..];
        let middle = len - 10;
        Some(format!("{}{}{}", bin, "*".repeat(middle), last4))
    })
}

pub fn mask_card(s: &str) -> String {
    mask_card_counted(s).0
}

fn mask_card_with(
    s: &str,
    claims: &crate::patterns::TokenClaims,
    encrypt: &dyn Fn(&str) -> Option<String>,
) -> String {
    CARD_RE
        .replace_all(s, |caps: &regex::Captures| {
            let m = caps.get(0).unwrap();
            let raw = m.as_str();
            if !luhn_valid(raw) || !claims.is_free(m.start(), m.end()) {
                return raw.to_string();
            }
            let digits: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
            match encrypt(&digits) {
                Some(encrypted) => {
                    claims.claim(m.start(), m.end());
                    crate::patterns::reinsert_digits(raw, &encrypted)
                }
                None => raw.to_string(),
            }
        })
        .into_owned()
}

pub fn mask_card_consistent(
    s: &str,
    hasher: &crate::patterns::consistent::ConsistentHasher,
    claims: &crate::patterns::TokenClaims,
) -> String {
    mask_card_with(s, claims, &|d| hasher.encrypt(d).ok())
}

pub fn mask_card_fpe(
    s: &str,
    cipher: &crate::patterns::fpe::FpeCipher,
    claims: &crate::patterns::TokenClaims,
) -> String {
    mask_card_with(s, claims, &|d| cipher.encrypt(d).ok())
}