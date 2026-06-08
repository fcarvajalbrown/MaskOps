

use once_cell::sync::Lazy;
use regex::Regex;

static SSN_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{3})-(\d{2})-(\d{4})\b").unwrap()
});

const KNOWN_INVALID: &[&str] = &["078051120", "219099999"];

fn valid_ssn(area: &str, group: &str, serial: &str) -> bool {
    let a: u32 = match area.parse() { Ok(v) => v, Err(_) => return false };
    let g: u32 = match group.parse() { Ok(v) => v, Err(_) => return false };
    let s: u32 = match serial.parse() { Ok(v) => v, Err(_) => return false };
    if a == 0 || a == 666 || a >= 900 { return false; }
    if g == 0 { return false; }
    if s == 0 { return false; }
    let compact = format!("{:03}{:02}{:04}", a, g, s);
    !KNOWN_INVALID.contains(&compact.as_str())
}

pub fn extract_ssn(s: &str) -> Option<String> {
    SSN_RE.captures_iter(s)
        .find(|c| valid_ssn(&c[1], &c[2], &c[3]))
        .map(|c| c[0].to_string())
}

pub fn contains_ssn(s: &str) -> bool {
    SSN_RE.captures_iter(s).any(|caps| valid_ssn(&caps[1], &caps[2], &caps[3]))
}

pub fn mask_ssn_counted(s: &str) -> (String, u32) {
    crate::patterns::replace_counted(&SSN_RE, s, |caps: &regex::Captures| {
        if !valid_ssn(&caps[1], &caps[2], &caps[3]) {
            return None;
        }
        Some("***-**-****".to_string())
    })
}

pub fn mask_ssn(s: &str) -> String {
    mask_ssn_counted(s).0
}

pub fn mask_ssn_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher) -> String {
    SSN_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_ssn(&caps[1], &caps[2], &caps[3]) {
                return caps[0].to_string();
            }
            let digits = format!("{}{}{}", &caps[1], &caps[2], &caps[3]);
            match hasher.encrypt(&digits) {
                Ok(hashed) => format!("{}-{}-{}", &hashed[..3], &hashed[3..5], &hashed[5..]),
                Err(_)     => caps[0].to_string(),
            }
        })
        .into_owned()
}

pub fn mask_ssn_fpe(s: &str, cipher: &crate::patterns::fpe::FpeCipher) -> String {
    SSN_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_ssn(&caps[1], &caps[2], &caps[3]) {
                return caps[0].to_string();
            }
            let digits = format!("{}{}{}", &caps[1], &caps[2], &caps[3]);
            match cipher.encrypt(&digits) {
                Ok(enc) => format!("{}-{}-{}", &enc[..3], &enc[3..5], &enc[5..]),
                Err(_) => caps[0].to_string(),
            }
        })
        .into_owned()
}
