//! Credit card number detection and masking.
//!
//! Covers: Visa, Mastercard, American Express, Discover, Maestro.
//! Masking style: BIN (first 6) + last 4 preserved, middle replaced with `*`.
//! This follows PCI-DSS maximum display rules and is GDPR-compliant for pseudonymisation.
//!
//! Luhn validation is applied to eliminate false positives.
//!
//! Examples:
//!   4111111111111111   →  411111******1111   (Visa, 16 digits)
//!   371449635398431    →  371449*****8431    (Amex, 15 digits)
//!   6304000000000000   →  630400******0000   (Maestro, 16 digits)

use once_cell::sync::Lazy;
use regex::Regex;

/// Matches Visa, Mastercard, Amex, Discover, and Maestro card numbers.
/// Accepts optional spaces or hyphens as group separators.
/// Uses verbose mode ((?x)) for readability.
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

/// Validates a card number string using the Luhn algorithm.
///
/// Strips spaces and hyphens before processing.
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

/// Returns true if the input contains a valid credit card number.
pub fn contains_card(s: &str) -> bool {
    CARD_RE.find_iter(s).any(|m| luhn_valid(m.as_str()))
}

/// Masks any valid credit card number found, preserving BIN (first 6) and last 4 digits.
///
/// Separators (spaces, hyphens) are stripped — output is always compact digits.
///
/// Examples:
///   `4111111111111111`   → `411111******1111`
///   `3714 496353 98431`  → `371449*****8431`
pub fn mask_card(s: &str) -> String {
    CARD_RE
        .replace_all(s, |caps: &regex::Captures| {
            let raw = caps.get(0).unwrap().as_str();
            if !luhn_valid(raw) {
                return raw.to_string();
            }
            let digits: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
            let len = digits.len();
            let bin = &digits[..6];
            let last4 = &digits[len - 4..];
            let middle = len - 10; // total digits - 6 (BIN) - 4 (last4)
            format!("{}{}{}", bin, "*".repeat(middle), last4)
        })
        .into_owned()
}