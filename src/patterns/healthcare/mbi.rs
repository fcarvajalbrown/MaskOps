

use once_cell::sync::Lazy;
use regex::Regex;

const ALPHA: &str = "ACDEFGHJKMNPQRTUVWXY";

static MBI_RE: Lazy<Regex> = Lazy::new(|| {
    
    
    
    
    
    
    
    
    
    Regex::new(
        r"\b([1-9][ACDEFGHJKMNPQRTUVWXY][0-9ACDEFGHJKMNPQRTUVWXY][0-9][ACDEFGHJKMNPQRTUVWXY]{2}[0-9][0-9ACDEFGHJKMNPQRTUVWXY]{2}[0-9]{2})\b"
    ).unwrap()
});

pub fn extract_mbi(s: &str) -> Option<String> {
    MBI_RE.find(s).map(|m| m.as_str().to_string())
}

pub fn contains_mbi(s: &str) -> bool {
    MBI_RE.is_match(s)
}

pub fn mask_mbi_counted(s: &str) -> (String, u32) {
    crate::patterns::replace_counted(&MBI_RE, s, |_caps: &regex::Captures| {
        Some("*".repeat(11))
    })
}

pub fn mask_mbi(s: &str) -> String {
    mask_mbi_counted(s).0
}

#[allow(dead_code)]
const _ALPHA_CHECK: &str = ALPHA;
