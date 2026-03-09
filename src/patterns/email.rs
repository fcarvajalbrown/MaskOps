//! Email address detection and masking patterns.
//!
//! Matches standard email addresses per RFC 5322 (simplified for performance).
//! Preserves the domain suffix for audit purposes.
//!
//! Example: `john.doe@example.com` → `****@example.com`

use once_cell::sync::Lazy;
use regex::Regex;

/// Matches standard email addresses.
pub static EMAIL_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b([a-zA-Z0-9._%+\-]+)@([a-zA-Z0-9.\-]+\.[a-zA-Z]{2,})\b").unwrap()
});

/// Masks an email address, preserving the domain suffix.
///
/// Example: `john.doe@example.com` → `****@example.com`
pub fn mask_email(value: &str) -> String {
    EMAIL_RE
        .replace_all(value, |caps: &regex::Captures| {
            let local = caps.get(1).unwrap().as_str();
            let domain = caps.get(2).unwrap().as_str();
            format!("{}@{}", "*".repeat(local.len()), domain)
        })
        .into_owned()
}

/// Returns true if the string contains at least one email address.
pub fn contains_email(value: &str) -> bool {
    EMAIL_RE.is_match(value)
}