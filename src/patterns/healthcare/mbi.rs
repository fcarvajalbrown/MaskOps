

use once_cell::sync::Lazy;
use regex::Regex;

const ALPHA: &str = "ACDEFGHJKMNPQRTUVWXY";

static MBI_RE: Lazy<Regex> = Lazy::new(|| {
    
    
    
    
    
    
    
    
    
    Regex::new(
        r"\b([1-9][ACDEFGHJKMNPQRTUVWXY][0-9ACDEFGHJKMNPQRTUVWXY][0-9][ACDEFGHJKMNPQRTUVWXY]{2}[0-9][0-9ACDEFGHJKMNPQRTUVWXY]{2}[0-9]{2})\b"
    ).unwrap()
});

pub fn contains_mbi(s: &str) -> bool {
    MBI_RE.is_match(s)
}

pub fn mask_mbi(s: &str) -> String {
    MBI_RE
        .replace_all(s, "*".repeat(11).as_str())
        .into_owned()
}

#[allow(dead_code)]
const _ALPHA_CHECK: &str = ALPHA;
