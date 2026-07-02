use once_cell::sync::Lazy;
use regex::Regex;

const ALPHA: &str = "ACDEFGHJKMNPQRTUVWXY";

static MBI_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(&format!(
        r"\b([1-9][{a}][0-9{a}][0-9][{a}][0-9{a}][0-9][{a}]{{2}}[0-9]{{2}})\b",
        a = ALPHA
    ))
    .unwrap()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_in_position_6_matches() {
        assert!(contains_mbi("1EG4T25MK73"));
    }

    #[test]
    fn test_alpha_in_position_6_matches() {
        assert!(contains_mbi("1EG4TE5MK72"));
    }

    #[test]
    fn test_digits_in_positions_8_9_rejected() {
        assert!(!contains_mbi("1EG4TE51273"));
    }

    #[test]
    fn test_excluded_letters_rejected() {
        assert!(!contains_mbi("1SG4TE5MK72"));
    }

    #[test]
    fn test_mask_replaces_with_asterisks() {
        assert_eq!(mask_mbi("MBI: 1EG4T25MK73"), "MBI: ***********");
    }
}
