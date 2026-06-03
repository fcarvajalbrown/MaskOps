//! Phone number detection and masking patterns.
//!
//! Detects E.164 international format (+CC followed by digits).
//! Uses country_codes.rs to identify the country automatically.
//! Preserves the country code prefix, masks the subscriber number.
//!
//! Example: `+56912345678` → `+56*********`
//! Example: `+49 170 1234567` → `+49**********`

use once_cell::sync::Lazy;
use regex::Regex;
use crate::patterns::country_codes::identify_country;

/// Matches E.164 international phone numbers with optional separators.
/// Requires a leading + and country code, followed by 6-14 digits.
pub static PHONE_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\+([0-9]{1,3})[\s\-\.]?([0-9][\s\-\.]?){6,14}[0-9]").unwrap()
});

/// Masks a phone number, preserving the country code prefix.
///
/// Example: `+56912345678` → `+56*********`
pub fn mask_phone(value: &str) -> String {
    PHONE_RE
        .replace_all(value, |caps: &regex::Captures| {
            let full = caps.get(0).unwrap().as_str();
            // Normalize: remove separators to find prefix boundary
            let normalized: String = full.chars().filter(|c| c.is_ascii_digit() || *c == '+').collect();
            let prefix_len = identify_country(&normalized)
                .map(|cc| cc.prefix.len())
                .unwrap_or(2); // fallback: preserve +X
            let (keep, rest) = full.split_at(prefix_len.min(full.len()));
            // Mask only digit/separator chars in the subscriber part
            let masked: String = rest.chars().map(|c| if c.is_ascii_digit() { '*' } else { c }).collect();
            format!("{}{}", keep, masked)
        })
        .into_owned()
}

/// Returns true if the string contains at least one phone number.
pub fn contains_phone(value: &str) -> bool {
    PHONE_RE.is_match(value)
}

/// Masks a phone number using FF3-1 format-preserving encryption on the subscriber digits.
///
/// The country code prefix is preserved. The subscriber digit sequence is
/// extracted, encrypted as a unit, and reassembled with the original separators.
/// Reversible with the same key and tweak.
///
/// Example: `+56912345678` → `+56361984203`  (same length, reversible)
pub fn mask_phone_fpe(value: &str, cipher: &crate::patterns::fpe::Ff3Cipher) -> String {
    PHONE_RE
        .replace_all(value, |caps: &regex::Captures| {
            let full = caps.get(0).unwrap().as_str();
            let normalized: String = full
                .chars()
                .filter(|c| c.is_ascii_digit() || *c == '+')
                .collect();
            let prefix_len = identify_country(&normalized)
                .map(|cc| cc.prefix.len())
                .unwrap_or(2);

            let keep = &full[..prefix_len.min(full.len())];
            let rest = &full[prefix_len.min(full.len())..];

            // Extract only digits from subscriber part for FPE.
            let subscriber_digits: String = rest
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect();

            if subscriber_digits.len() < 2 {
                return full.to_string();
            }

            match cipher.encrypt(&subscriber_digits) {
                Ok(encrypted) => {
                    // Reassemble: reinsert non-digit chars at their original positions.
                    let mut enc_iter = encrypted.chars();
                    let reassembled: String = rest
                        .chars()
                        .map(|c| {
                            if c.is_ascii_digit() {
                                enc_iter.next().unwrap_or(c)
                            } else {
                                c
                            }
                        })
                        .collect();
                    format!("{}{}", keep, reassembled)
                }
                Err(_) => full.to_string(),
            }
        })
        .into_owned()
}