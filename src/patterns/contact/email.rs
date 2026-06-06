

use once_cell::sync::Lazy;
use regex::Regex;

pub static EMAIL_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b([a-zA-Z0-9._%+\-]+)@([a-zA-Z0-9.\-]+\.[a-zA-Z]{2,})\b").unwrap()
});

pub fn mask_email(value: &str) -> String {
    EMAIL_RE
        .replace_all(value, |caps: &regex::Captures| {
            let local = caps.get(1).unwrap().as_str();
            let domain = caps.get(2).unwrap().as_str();
            format!("{}@{}", "*".repeat(local.len()), domain)
        })
        .into_owned()
}

pub fn contains_email(value: &str) -> bool {
    EMAIL_RE.is_match(value)
}