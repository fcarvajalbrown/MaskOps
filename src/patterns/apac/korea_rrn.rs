
use once_cell::sync::Lazy;
use regex::Regex;

static RRN_FMT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{6}-\d{7})\b").unwrap()
});

static RRN_COMPACT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{13})\b").unwrap()
});

const RRN_WEIGHTS: [u32; 12] = [2, 3, 4, 5, 6, 7, 8, 9, 2, 3, 4, 5];

fn valid_rrn(digits: &str) -> bool {
    if digits.len() != 13 {
        return false;
    }
    let ds: Vec<u32> = digits.chars().map(|c| c as u32 - b'0' as u32).collect();
    let mm = ds[2] * 10 + ds[3];
    let dd = ds[4] * 10 + ds[5];
    if mm < 1 || mm > 12 || dd < 1 || dd > 31 {
        return false;
    }
    let sum: u32 = ds[..12].iter().zip(RRN_WEIGHTS.iter()).map(|(d, &w)| d * w).sum();
    let expected = (11 - (sum % 11)) % 10;
    ds[12] == expected
}

pub fn extract_rrn(s: &str) -> Option<String> {
    RRN_FMT_RE
        .find_iter(s)
        .find(|m| {
            let d: String = m.as_str().chars().filter(|c| c.is_ascii_digit()).collect();
            valid_rrn(&d)
        })
        .map(|m| m.as_str().to_string())
        .or_else(|| {
            RRN_COMPACT_RE
                .find_iter(s)
                .find(|m| valid_rrn(m.as_str()))
                .map(|m| m.as_str().to_string())
        })
}

pub fn contains_rrn(s: &str) -> bool {
    RRN_FMT_RE.find_iter(s).any(|m| {
        let d: String = m.as_str().chars().filter(|c| c.is_ascii_digit()).collect();
        valid_rrn(&d)
    }) || RRN_COMPACT_RE.find_iter(s).any(|m| valid_rrn(m.as_str()))
}

pub fn mask_rrn_counted(s: &str) -> (String, u32) {
    let (s, n_fmt) = crate::patterns::replace_counted(&RRN_FMT_RE, s, |caps: &regex::Captures| {
        let raw = &caps[0];
        let d: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
        if !valid_rrn(&d) {
            return None;
        }
        Some("*".repeat(raw.len()))
    });
    let (s, n_compact) = crate::patterns::replace_counted(&RRN_COMPACT_RE, &s, |caps: &regex::Captures| {
        if !valid_rrn(&caps[0]) {
            return None;
        }
        Some("*".repeat(13))
    });
    (s, n_fmt + n_compact)
}

pub fn mask_rrn(s: &str) -> String {
    mask_rrn_counted(s).0
}

fn rrn_valid(t: &str) -> bool {
    let d: String = t.chars().filter(|c| c.is_ascii_digit()).collect();
    valid_rrn(&d)
}

pub fn mask_rrn_fpe(s: &str, cipher: &crate::patterns::fpe::FpeCipher, claims: &crate::patterns::TokenClaims) -> String {
    let enc = |d: &str| cipher.encrypt(d).ok();
    let s = crate::patterns::mask_family(&RRN_FMT_RE, s, claims, &|t, _, _| rrn_valid(t), &enc);
    crate::patterns::mask_family(&RRN_COMPACT_RE, &s, claims, &|t, _, _| rrn_valid(t), &enc)
}

pub fn mask_rrn_consistent(
    s: &str,
    hasher: &crate::patterns::consistent::ConsistentHasher,
    claims: &crate::patterns::TokenClaims,
) -> String {
    let enc = |d: &str| hasher.encrypt(d).ok();
    let s = crate::patterns::mask_family(&RRN_FMT_RE, s, claims, &|t, _, _| rrn_valid(t), &enc);
    crate::patterns::mask_family(&RRN_COMPACT_RE, &s, claims, &|t, _, _| rrn_valid(t), &enc)
}
