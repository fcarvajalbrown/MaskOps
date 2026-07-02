use once_cell::sync::Lazy;
use regex::Regex;

pub static IBAN_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b[A-Z]{2}\d{2}(?: ?[A-Z0-9]{4}){2,7}(?: ?[A-Z0-9]{1,3})?\b").unwrap()
});

const MIN_IBAN_LEN: usize = 15;
const MAX_IBAN_LEN: usize = 34;

fn mod97_valid(compact: &str) -> bool {
    if compact.len() < MIN_IBAN_LEN || compact.len() > MAX_IBAN_LEN {
        return false;
    }
    let rearranged = compact.as_bytes()[4..]
        .iter()
        .chain(compact.as_bytes()[..4].iter());
    let mut rem: u32 = 0;
    for b in rearranged {
        rem = match b {
            b'0'..=b'9' => (rem * 10 + (b - b'0') as u32) % 97,
            b'A'..=b'Z' => (rem * 100 + (b - b'A') as u32 + 10) % 97,
            _ => return false,
        };
    }
    rem == 1
}

fn valid_iban_span(m: &str) -> Option<usize> {
    let mut end = m.len();
    loop {
        let cand = m[..end].trim_end();
        let compact: String = cand.chars().filter(|c| *c != ' ').collect();
        if mod97_valid(&compact) {
            return Some(cand.len());
        }
        match cand.rfind(' ') {
            Some(pos) => end = pos,
            None => return None,
        }
    }
}

pub fn mask_iban_counted(value: &str) -> (String, u32) {
    crate::patterns::replace_counted(&IBAN_RE, value, |caps: &regex::Captures| {
        let m = caps.get(0).unwrap().as_str();
        let end = valid_iban_span(m)?;
        let (iban, tail) = m.split_at(end);
        let (visible, secret) = iban.split_at(4);
        let masked: String = secret.chars().map(|c| if c == ' ' { ' ' } else { '*' }).collect();
        Some(format!("{}{}{}", visible, masked, tail))
    })
}

pub fn mask_iban(value: &str) -> String {
    mask_iban_counted(value).0
}

pub fn contains_iban(value: &str) -> bool {
    IBAN_RE
        .find_iter(value)
        .any(|m| valid_iban_span(m.as_str()).is_some())
}

pub fn extract_iban(value: &str) -> Option<String> {
    IBAN_RE.find_iter(value).find_map(|m| {
        valid_iban_span(m.as_str()).map(|end| m.as_str()[..end].to_string())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compact_iban_masked() {
        let out = mask_iban("DE89370400440532013000");
        assert_eq!(out, "DE89******************");
    }

    #[test]
    fn test_print_format_iban_masked() {
        let out = mask_iban("pay DE89 3704 0044 0532 0130 00 now");
        assert_eq!(out, "pay DE89 **** **** **** **** ** now");
    }

    #[test]
    fn test_invalid_checksum_untouched() {
        let input = "DE89370400440532013001";
        assert_eq!(mask_iban(input), input);
        assert!(!contains_iban(input));
    }

    #[test]
    fn test_iban_shaped_reference_code_untouched() {
        let input = "ref DE12ABCD12345678901";
        assert_eq!(mask_iban(input), input);
    }

    #[test]
    fn test_32_char_iban_masked() {
        let lc = "LC55HEMM000100010012001200023015";
        assert_eq!(lc.len(), 32);
        assert!(contains_iban(lc));
        assert!(mask_iban(lc).starts_with("LC55"));
        assert!(!mask_iban(lc).contains("HEMM"));
    }

    #[test]
    fn test_spaced_iban_followed_by_numbers() {
        let out = mask_iban("DE89 3704 0044 0532 0130 00 amount 1234");
        assert!(out.starts_with("DE89 ****"));
        assert!(out.contains("amount 1234"));
    }

    #[test]
    fn test_extract_print_format() {
        assert_eq!(
            extract_iban("acct DE89 3704 0044 0532 0130 00 ok"),
            Some("DE89 3704 0044 0532 0130 00".to_string())
        );
    }
}
