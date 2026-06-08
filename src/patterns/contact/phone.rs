

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

pub fn mask_phone_consistent(value: &str, hasher: &crate::patterns::consistent::ConsistentHasher) -> String {
    PHONE_RE
        .replace_all(value, |caps: &regex::Captures| {
            let full = caps.get(0).unwrap().as_str();
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

            match hasher.encrypt(&subscriber_digits) {
                Ok(hashed) => {
                    let mut hash_iter = hashed.chars();
                    let reassembled: String = rest
                        .chars()
                        .map(|c| {
                            if c.is_ascii_digit() {
                                hash_iter.next().unwrap_or(c)
                            } else {
                                c
                            }
                        })
                        .collect();
                    format!("{}{}", keep, reassembled)
                }
                Err(_) => full.to_string(),
            }
        })
        .into_owned()
}

pub fn mask_phone_fpe(value: &str, cipher: &crate::patterns::fpe::Ff3Cipher) -> String {
    PHONE_RE
        .replace_all(value, |caps: &regex::Captures| {
            let full = caps.get(0).unwrap().as_str();
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

            match cipher.encrypt(&subscriber_digits) {
                Ok(encrypted) => {
                    
                    let mut enc_iter = encrypted.chars();
                    let reassembled: String = rest
                        .chars()
                        .map(|c| {
                            if c.is_ascii_digit() {
                                enc_iter.next().unwrap_or(c)
                            } else {
                                c
                            }
                        })
                        .collect();
                    format!("{}{}", keep, reassembled)
                }
                Err(_) => full.to_string(),
            }
        })
        .into_owned()
}