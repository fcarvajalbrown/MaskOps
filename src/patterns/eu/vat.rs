

use once_cell::sync::Lazy;
use regex::Regex;

pub static VAT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"\b(ATU[0-9]{8}|BE[01][0-9]{9}|BG[0-9]{9,10}|CY[0-9]{8}[A-Z]|CZ[0-9]{8,10}|DE[0-9]{9}|DK[0-9]{8}|EE[0-9]{9}|EL[0-9]{9}|ES[A-Z0-9][0-9]{7}[A-Z0-9]|FI[0-9]{8}|FR[A-Z0-9]{2}[0-9]{9}|HR[0-9]{11}|HU[0-9]{8}|IE([0-9]{7}[A-Z]{1,2}|[0-9][A-Z][0-9]{5}[A-Z])|IT[0-9]{11}|LT([0-9]{9}|[0-9]{12})|LU[0-9]{8}|LV[0-9]{11}|MT[0-9]{8}|NL[0-9]{9}B[0-9]{2}|PL[0-9]{10}|PT[0-9]{9}|RO[0-9]{2,10}|SE[0-9]{12}|SI[0-9]{8}|SK[0-9]{10})\b"
    ).unwrap()
});

pub fn mask_vat_counted(value: &str) -> (String, u32) {
    crate::patterns::replace_counted(&VAT_RE, value, |caps: &regex::Captures| {
        let vat = caps.get(0).unwrap().as_str();
        let (prefix, rest) = vat.split_at(2.min(vat.len()));
        Some(format!("{}{}", prefix, "*".repeat(rest.len())))
    })
}

pub fn mask_vat(value: &str) -> String {
    mask_vat_counted(value).0
}

pub fn contains_vat(value: &str) -> bool {
    VAT_RE.is_match(value)
}

/// Returns the first EU VAT number found, or None.
pub fn extract_vat(value: &str) -> Option<String> {
    VAT_RE.find(value).map(|m| m.as_str().to_string())
}
