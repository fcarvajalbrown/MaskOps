

use once_cell::sync::Lazy;
use regex::Regex;

static PASSPORT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b([A-Z][0-9]{8})\b").unwrap()
});

pub fn extract_us_passport(s: &str) -> Option<String> {
    PASSPORT_RE.find(s).map(|m| m.as_str().to_string())
}

pub fn contains_us_passport(s: &str) -> bool {
    PASSPORT_RE.is_match(s)
}

pub fn mask_us_passport_counted(s: &str) -> (String, u32) {
    crate::patterns::replace_counted(&PASSPORT_RE, s, |_caps: &regex::Captures| {
        Some("*********".to_string())
    })
}

pub fn mask_us_passport(s: &str) -> String {
    mask_us_passport_counted(s).0
}
