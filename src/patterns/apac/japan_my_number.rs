
use once_cell::sync::Lazy;
use regex::Regex;

static MN_SPACED_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{4}[ \-]\d{4}[ \-]\d{4})\b").unwrap()
});

static MN_COMPACT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b([1-9]\d{11})\b").unwrap()
});

const MN_WEIGHTS: [u32; 11] = [6, 5, 4, 3, 2, 7, 6, 5, 4, 3, 2];

fn valid_my_number(digits: &str) -> bool {
    if digits.len() != 12 {
        return false;
    }
    let ds: Vec<u32> = digits.chars().map(|c| c as u32 - b'0' as u32).collect();
    let sum: u32 = ds[..11].iter().zip(MN_WEIGHTS.iter()).map(|(d, &w)| d * w).sum();
    let rem = sum % 11;
    let expected = if rem <= 1 { 0 } else { 11 - rem };
    ds[11] == expected
}

pub fn extract_my_number(s: &str) -> Option<String> {
    MN_SPACED_RE
        .find_iter(s)
        .find(|m| {
            let d: String = m.as_str().chars().filter(|c| c.is_ascii_digit()).collect();
            valid_my_number(&d)
        })
        .map(|m| m.as_str().to_string())
        .or_else(|| {
            MN_COMPACT_RE
                .find_iter(s)
                .find(|m| valid_my_number(m.as_str()))
                .map(|m| m.as_str().to_string())
        })
}

pub fn contains_my_number(s: &str) -> bool {
    MN_SPACED_RE.find_iter(s).any(|m| {
        let d: String = m.as_str().chars().filter(|c| c.is_ascii_digit()).collect();
        valid_my_number(&d)
    }) || MN_COMPACT_RE.find_iter(s).any(|m| valid_my_number(m.as_str()))
}

pub fn mask_my_number_counted(s: &str) -> (String, u32) {
    let (s, n_spaced) = crate::patterns::replace_counted(&MN_SPACED_RE, s, |caps: &regex::Captures| {
        let raw = &caps[0];
        let d: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
        if !valid_my_number(&d) {
            return None;
        }
        let sep: char = raw.chars().nth(4).unwrap_or(' ');
        Some(format!("****{}****{}****", sep, sep))
    });
    let (s, n_compact) = crate::patterns::replace_counted(&MN_COMPACT_RE, &s, |caps: &regex::Captures| {
        if !valid_my_number(&caps[0]) {
            return None;
        }
        Some("*".repeat(12))
    });
    (s, n_spaced + n_compact)
}

pub fn mask_my_number(s: &str) -> String {
    mask_my_number_counted(s).0
}

pub fn mask_my_number_fpe(s: &str, cipher: &crate::patterns::fpe::Ff3Cipher) -> String {
    let s = MN_SPACED_RE
        .replace_all(s, |caps: &regex::Captures| {
            let raw = &caps[0];
            let d: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
            if !valid_my_number(&d) {
                return raw.to_string();
            }
            let sep: char = raw.chars().nth(4).unwrap_or(' ');
            match cipher.encrypt(&d) {
                Ok(enc) => format!("{}{}{}{}{}", &enc[..4], sep, &enc[4..8], sep, &enc[8..]),
                Err(_) => raw.to_string(),
            }
        })
        .into_owned();
    MN_COMPACT_RE
        .replace_all(&s, |caps: &regex::Captures| {
            if !valid_my_number(&caps[0]) {
                return caps[0].to_string();
            }
            match cipher.encrypt(&caps[0]) {
                Ok(enc) => enc,
                Err(_) => caps[0].to_string(),
            }
        })
        .into_owned()
}

pub fn mask_my_number_consistent(
    s: &str,
    hasher: &crate::patterns::consistent::ConsistentHasher,
) -> String {
    let s = MN_SPACED_RE
        .replace_all(s, |caps: &regex::Captures| {
            let raw = &caps[0];
            let d: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
            if !valid_my_number(&d) {
                return raw.to_string();
            }
            let sep: char = raw.chars().nth(4).unwrap_or(' ');
            match hasher.encrypt(&d) {
                Ok(hashed) => format!("{}{}{}{}{}", &hashed[..4], sep, &hashed[4..8], sep, &hashed[8..]),
                Err(_) => raw.to_string(),
            }
        })
        .into_owned();
    MN_COMPACT_RE
        .replace_all(&s, |caps: &regex::Captures| {
            if !valid_my_number(&caps[0]) {
                return caps[0].to_string();
            }
            match hasher.encrypt(&caps[0]) {
                Ok(hashed) => hashed,
                Err(_) => caps[0].to_string(),
            }
        })
        .into_owned()
}
