

use once_cell::sync::Lazy;
use regex::Regex;

pub static IBAN_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b([A-Z]{2}\d{2}[A-Z0-9]{4}[0-9]{7}([A-Z0-9]?){0,16})\b").unwrap()
});

pub fn mask_iban(value: &str) -> String {
    IBAN_RE
        .replace_all(value, |caps: &regex::Captures| {
            let iban = caps.get(0).unwrap().as_str();
            
            let (visible, secret) = iban.split_at(4.min(iban.len()));
            format!("{}{}", visible, "*".repeat(secret.len()))
        })
        .into_owned()
}

pub fn contains_iban(value: &str) -> bool {
    IBAN_RE.is_match(value)
}

/// Returns the first IBAN found, or None.
pub fn extract_iban(value: &str) -> Option<String> {
    IBAN_RE.find(value).map(|m| m.as_str().to_string())
}
