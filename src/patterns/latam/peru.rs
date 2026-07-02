use once_cell::sync::Lazy;
use regex::Regex;

static PE_DNI_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b([0-9]{8})\b").unwrap()
});

static PE_DNI_CONTEXT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\b(dni|d\.n\.i\.?|documento nacional de identidad)\b").unwrap()
});

const CONTEXT_WINDOW: usize = 32;

fn floor_char_boundary(s: &str, mut i: usize) -> usize {
    while !s.is_char_boundary(i) {
        i -= 1;
    }
    i
}

fn ceil_char_boundary(s: &str, mut i: usize) -> usize {
    while i < s.len() && !s.is_char_boundary(i) {
        i += 1;
    }
    i
}

fn has_dni_context(s: &str, start: usize, end: usize) -> bool {
    let w_start = floor_char_boundary(s, start.saturating_sub(CONTEXT_WINDOW));
    let w_end = ceil_char_boundary(s, (end + CONTEXT_WINDOW).min(s.len()));
    PE_DNI_CONTEXT_RE.is_match(&s[w_start..w_end])
}

pub fn extract_pe_dni(s: &str) -> Option<String> {
    PE_DNI_RE
        .find_iter(s)
        .find(|m| has_dni_context(s, m.start(), m.end()))
        .map(|m| m.as_str().to_string())
}

pub fn extract_pe_dni_bare(s: &str) -> Option<String> {
    PE_DNI_RE.find(s).map(|m| m.as_str().to_string())
}

pub fn contains_pe_dni(s: &str) -> bool {
    PE_DNI_RE
        .find_iter(s)
        .any(|m| has_dni_context(s, m.start(), m.end()))
}

pub fn contains_pe_dni_bare(s: &str) -> bool {
    PE_DNI_RE.is_match(s)
}

pub fn mask_pe_dni_counted(s: &str) -> (String, u32) {
    crate::patterns::replace_counted(&PE_DNI_RE, s, |caps: &regex::Captures| {
        let m = caps.get(0).unwrap();
        if !has_dni_context(s, m.start(), m.end()) {
            return None;
        }
        Some("********".to_string())
    })
}

pub fn mask_pe_dni_bare_counted(s: &str) -> (String, u32) {
    crate::patterns::replace_counted(&PE_DNI_RE, s, |_caps: &regex::Captures| {
        Some("********".to_string())
    })
}

pub fn mask_pe_dni(s: &str) -> String {
    mask_pe_dni_counted(s).0
}

pub fn mask_pe_dni_bare(s: &str) -> String {
    mask_pe_dni_bare_counted(s).0
}

fn encrypt_matches(
    s: &str,
    require_context: bool,
    encrypt: &dyn Fn(&str) -> Option<String>,
) -> String {
    PE_DNI_RE
        .replace_all(s, |caps: &regex::Captures| {
            let m = caps.get(0).unwrap();
            if require_context && !has_dni_context(s, m.start(), m.end()) {
                return m.as_str().to_string();
            }
            encrypt(m.as_str()).unwrap_or_else(|| m.as_str().to_string())
        })
        .into_owned()
}

pub fn mask_pe_dni_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher) -> String {
    encrypt_matches(s, true, &|t| hasher.encrypt(t).ok())
}

pub fn mask_pe_dni_bare_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher) -> String {
    encrypt_matches(s, false, &|t| hasher.encrypt(t).ok())
}

pub fn mask_pe_dni_fpe(s: &str, cipher: &crate::patterns::fpe::FpeCipher) -> String {
    encrypt_matches(s, true, &|t| cipher.encrypt(t).ok())
}

pub fn mask_pe_dni_bare_fpe(s: &str, cipher: &crate::patterns::fpe::FpeCipher) -> String {
    encrypt_matches(s, false, &|t| cipher.encrypt(t).ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dni_with_context_masked() {
        assert_eq!(mask_pe_dni("DNI 12345678"), "DNI ********");
    }

    #[test]
    fn test_dni_with_colon_context_masked() {
        assert_eq!(mask_pe_dni("D.N.I.: 12345678"), "D.N.I.: ********");
    }

    #[test]
    fn test_date_without_context_untouched() {
        assert_eq!(mask_pe_dni("fecha 20250630"), "fecha 20250630");
    }

    #[test]
    fn test_order_number_without_context_untouched() {
        assert_eq!(mask_pe_dni("order 45821903 shipped"), "order 45821903 shipped");
    }

    #[test]
    fn test_contains_requires_context() {
        assert!(contains_pe_dni("cliente con DNI 12345678"));
        assert!(!contains_pe_dni("invoice 12345678"));
    }

    #[test]
    fn test_bare_variant_masks_without_context() {
        assert_eq!(mask_pe_dni_bare("invoice 12345678"), "invoice ********");
        assert!(contains_pe_dni_bare("invoice 12345678"));
    }

    #[test]
    fn test_context_after_number_masked() {
        assert_eq!(mask_pe_dni("12345678 (DNI)"), "******** (DNI)");
    }
}
