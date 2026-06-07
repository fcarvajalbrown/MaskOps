

use once_cell::sync::Lazy;
use regex::Regex;

pub static EMAIL_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b([a-zA-Z0-9._%+\-]+)@([a-zA-Z0-9.\-]+\.[a-zA-Z]{2,})\b").unwrap()
});

pub fn mask_email_counted(value: &str) -> (String, u32) {
    crate::patterns::replace_counted(&EMAIL_RE, value, |caps: &regex::Captures| {
        let local = caps.get(1).unwrap().as_str();
        let domain = caps.get(2).unwrap().as_str();
        Some(format!("{}@{}", "*".repeat(local.len()), domain))
    })
}

pub fn mask_email(value: &str) -> String {
    mask_email_counted(value).0
}

pub fn contains_email(value: &str) -> bool {
    EMAIL_RE.is_match(value)
}

/// Returns the first email address found, or None.
pub fn extract_email(value: &str) -> Option<String> {
    EMAIL_RE.find(value).map(|m| m.as_str().to_string())
}