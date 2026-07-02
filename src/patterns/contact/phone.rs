

use once_cell::sync::Lazy;
use regex::Regex;
use crate::patterns::country_codes::identify_country;

pub static PHONE_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\+([0-9]{1,3})[\s\-\.]?([0-9][\s\-\.]?){6,14}[0-9]").unwrap()
});

pub fn mask_phone_counted(value: &str) -> (String, u32) {
    crate::patterns::replace_counted(&PHONE_RE, value, |caps: &regex::Captures| {
        let full = caps.get(0).unwrap().as_str();
        let normalized: String = full.chars().filter(|c| c.is_ascii_digit() || *c == '+').collect();
        let prefix_len = identify_country(&normalized)
            .map(|cc| cc.prefix.len())
            .unwrap_or(2);
        let (keep, rest) = full.split_at(prefix_len.min(full.len()));
        let masked: String = rest.chars().map(|c| if c.is_ascii_digit() { '*' } else { c }).collect();
        Some(format!("{}{}", keep, masked))
    })
}

pub fn mask_phone(value: &str) -> String {
    mask_phone_counted(value).0
}

pub fn contains_phone(value: &str) -> bool {
    PHONE_RE.is_match(value)
}

pub fn extract_phone(value: &str) -> Option<String> {
    PHONE_RE.find(value).map(|m| m.as_str().to_string())
}

fn mask_phone_with(
    value: &str,
    claims: &crate::patterns::TokenClaims,
    encrypt: &dyn Fn(&str) -> Option<String>,
) -> String {
    PHONE_RE
        .replace_all(value, |caps: &regex::Captures| {
            let m = caps.get(0).unwrap();
            let full = m.as_str();
            if !claims.is_free(m.start(), m.end()) {
                return full.to_string();
            }
            let normalized: String = full
                .chars()
                .filter(|c| c.is_ascii_digit() || *c == '+')
                .collect();
            let prefix_len = identify_country(&normalized)
                .map(|cc| cc.prefix.len())
                .unwrap_or(2);

            let keep = &full[..prefix_len.min(full.len())];
            let rest = &full[prefix_len.min(full.len())..];

            let subscriber_digits: String = rest
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect();

            if subscriber_digits.len() < 2 {
                return full.to_string();
            }

            match encrypt(&subscriber_digits) {
                Some(encrypted) => {
                    claims.claim(m.start(), m.end());
                    format!("{}{}", keep, crate::patterns::reinsert_digits(rest, &encrypted))
                }
                None => full.to_string(),
            }
        })
        .into_owned()
}

pub fn mask_phone_consistent(
    value: &str,
    hasher: &crate::patterns::consistent::ConsistentHasher,
    claims: &crate::patterns::TokenClaims,
) -> String {
    mask_phone_with(value, claims, &|d| hasher.encrypt(d).ok())
}

pub fn mask_phone_fpe(
    value: &str,
    cipher: &crate::patterns::fpe::FpeCipher,
    claims: &crate::patterns::TokenClaims,
) -> String {
    mask_phone_with(value, claims, &|d| cipher.encrypt(d).ok())
}