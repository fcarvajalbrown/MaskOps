

use once_cell::sync::Lazy;
use regex::Regex;

static PE_DNI_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b([0-9]{8})\b").unwrap()
});

pub fn extract_pe_dni(s: &str) -> Option<String> {
    PE_DNI_RE.find(s).map(|m| m.as_str().to_string())
}

pub fn contains_pe_dni(s: &str) -> bool {
    PE_DNI_RE.is_match(s)
}

pub fn mask_pe_dni(s: &str) -> String {
    PE_DNI_RE
        .replace_all(s, "********")
        .into_owned()
}

pub fn mask_pe_dni_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher) -> String {
    PE_DNI_RE
        .replace_all(s, |caps: &regex::Captures| {
            match hasher.encrypt(&caps[0]) {
                Ok(hashed) => hashed,
                Err(_)     => caps[0].to_string(),
            }
        })
        .into_owned()
}

pub fn mask_pe_dni_fpe(s: &str, cipher: &crate::patterns::fpe::Ff3Cipher) -> String {
    PE_DNI_RE
        .replace_all(s, |caps: &regex::Captures| {
            match cipher.encrypt(&caps[0]) {
                Ok(enc) => enc,
                Err(_)  => caps[0].to_string(),
            }
        })
        .into_owned()
}
