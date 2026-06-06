

use once_cell::sync::Lazy;
use regex::Regex;

static NIR_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"\b([12]\d{2}(?:0[1-9]|1[0-2]|20)(?:\d{2}|2[AB])\d{6}\d{2})\b"
    ).unwrap()
});

fn valid_nir(nir: &str) -> bool {
    if nir.len() != 15 {
        return false;
    }
    let body = &nir[..13];
    let key_str = &nir[13..];
    let numeric = body.replace("2A", "19").replace("2B", "18");
    let n: u64 = match numeric.parse() {
        Ok(v) => v,
        Err(_) => return false,
    };
    let expected_key = 97 - (n % 97);
    let parsed_key: u64 = match key_str.parse() {
        Ok(v) => v,
        Err(_) => return false,
    };
    parsed_key == expected_key && parsed_key >= 1 && parsed_key <= 97
}

pub fn contains_nir(s: &str) -> bool {
    NIR_RE.find_iter(s).any(|m| valid_nir(m.as_str()))
}

pub fn mask_nir(s: &str) -> String {
    NIR_RE
        .replace_all(s, |caps: &regex::Captures| {
            let nir = &caps[0];
            if !valid_nir(nir) {
                return nir.to_string();
            }
            "*".repeat(nir.len())
        })
        .into_owned()
}
