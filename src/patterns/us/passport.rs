

use once_cell::sync::Lazy;
use regex::Regex;

static PASSPORT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b([A-Z][0-9]{8})\b").unwrap()
});

pub fn contains_us_passport(s: &str) -> bool {
    PASSPORT_RE.is_match(s)
}

pub fn mask_us_passport(s: &str) -> String {
    PASSPORT_RE
        .replace_all(s, "*********")
        .into_owned()
}
