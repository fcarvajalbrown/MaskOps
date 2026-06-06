

use once_cell::sync::Lazy;
use regex::Regex;

static RUT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{1,2}\.?\d{3}\.?\d{3}-[\dKk])\b").unwrap()
});

static CPF_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{3}\.?\d{3}\.?\d{3}-?\d{2})\b").unwrap()
});

static CURP_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b([A-Z][AEIOU][A-Z]{2}\d{6}[HM][A-Z]{2}[B-DF-HJ-NP-TV-Z]{3}[A-Z0-9]\d)\b")
        .unwrap()
});

static ARG_DNI_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{1,2}\.\d{3}\.\d{3})\b").unwrap()
});

static CO_CC_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{1,3}\.\d{3}\.\d{3}(?:\.\d{3})?)\b").unwrap()
});

static CO_NIT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{9})-(\d)\b").unwrap()
});

fn valid_rut(rut: &str) -> bool {
    let clean: String = rut.chars().filter(|c| c.is_alphanumeric()).collect();
    if clean.len() < 2 {
        return false;
    }
    let (body, dv) = clean.split_at(clean.len() - 1);
    let dv = dv.to_uppercase();

    let digits: Vec<u32> = body.chars().rev().filter_map(|c| c.to_digit(10)).collect();
    let factors = [2, 3, 4, 5, 6, 7];
    let sum: u32 = digits.iter().enumerate().map(|(i, &d)| d * factors[i % 6]).sum();

    let expected = match 11 - (sum % 11) {
        11 => "0".to_string(),
        10 => "K".to_string(),
        n => n.to_string(),
    };
    dv == expected
}

fn valid_cpf(cpf: &str) -> bool {
    let digits: Vec<u32> = cpf
        .chars()
        .filter(|c| c.is_ascii_digit())
        .filter_map(|c| c.to_digit(10))
        .collect();

    if digits.len() != 11 {
        return false;
    }
    
    if digits.windows(2).all(|w| w[0] == w[1]) {
        return false;
    }

    let sum1: u32 = digits[..9].iter().enumerate().map(|(i, &d)| d * (10 - i as u32)).sum();
    let r1 = (sum1 * 10) % 11;
    let d1 = if r1 == 10 { 0 } else { r1 };

    let sum2: u32 = digits[..10].iter().enumerate().map(|(i, &d)| d * (11 - i as u32)).sum();
    let r2 = (sum2 * 10) % 11;
    let d2 = if r2 == 10 { 0 } else { r2 };

    digits[9] == d1 && digits[10] == d2
}

pub fn extract_rut(s: &str) -> Option<String> {
    RUT_RE.find_iter(s).find(|m| valid_rut(m.as_str())).map(|m| m.as_str().to_string())
}

pub fn extract_cpf(s: &str) -> Option<String> {
    CPF_RE.find_iter(s).find(|m| valid_cpf(m.as_str())).map(|m| m.as_str().to_string())
}

pub fn extract_curp(s: &str) -> Option<String> {
    CURP_RE.find(s).map(|m| m.as_str().to_string())
}

pub fn extract_arg_dni(s: &str) -> Option<String> {
    ARG_DNI_RE.find_iter(s)
        .find(|m| !followed_by_id_suffix(s, m.end()))
        .map(|m| m.as_str().to_string())
}

pub fn extract_co_cc(s: &str) -> Option<String> {
    CO_CC_RE.find_iter(s)
        .find(|m| !followed_by_id_suffix(s, m.end()))
        .map(|m| m.as_str().to_string())
}

pub fn extract_co_nit(s: &str) -> Option<String> {
    CO_NIT_RE.captures_iter(s)
        .find(|c| valid_nit(&c[1], &c[2]))
        .map(|c| c[0].to_string())
}

pub fn contains_rut(s: &str) -> bool {
    RUT_RE.find_iter(s).any(|m| valid_rut(m.as_str()))
}

pub fn mask_rut(s: &str) -> String {
    RUT_RE
        .replace_all(s, |caps: &regex::Captures| {
            let rut = &caps[0];
            if !valid_rut(rut) {
                return rut.to_string();
            }
            let dv = &rut[rut.len() - 1..];
            let body_len = rut.len() - 2; 
            format!("{}-{}", "*".repeat(body_len), dv)
        })
        .into_owned()
}

pub fn contains_cpf(s: &str) -> bool {
    CPF_RE.find_iter(s).any(|m| valid_cpf(m.as_str()))
}

pub fn mask_cpf(s: &str) -> String {
    CPF_RE
        .replace_all(s, |caps: &regex::Captures| {
            let cpf = &caps[0];
            if !valid_cpf(cpf) {
                return cpf.to_string();
            }
            let digits: String = cpf.chars().filter(|c| c.is_ascii_digit()).collect();
            let check = &digits[9..];
            format!("{}-{}", "*".repeat(9), check)
        })
        .into_owned()
}

pub fn contains_curp(s: &str) -> bool {
    CURP_RE.is_match(s)
}

pub fn mask_curp(s: &str) -> String {
    CURP_RE
        .replace_all(s, |caps: &regex::Captures| "*".repeat(caps[0].len()))
        .into_owned()
}

pub fn mask_rut_fpe(s: &str, cipher: &crate::patterns::fpe::Ff3Cipher) -> String {
    RUT_RE
        .replace_all(s, |caps: &regex::Captures| {
            let rut = &caps[0];
            if !valid_rut(rut) {
                return rut.to_string();
            }
            let clean: String = rut.chars().filter(|c| c.is_alphanumeric()).collect();
            let body = &clean[..clean.len() - 1];
            let dv   = &clean[clean.len() - 1..];

            match cipher.encrypt(body) {
                Ok(encrypted) => format!("{}-{}", encrypted, dv),
                Err(_)        => rut.to_string(),
            }
        })
        .into_owned()
}

pub fn mask_cpf_fpe(s: &str, cipher: &crate::patterns::fpe::Ff3Cipher) -> String {
    CPF_RE
        .replace_all(s, |caps: &regex::Captures| {
            let cpf = &caps[0];
            if !valid_cpf(cpf) {
                return cpf.to_string();
            }
            let digits: String = cpf.chars().filter(|c| c.is_ascii_digit()).collect();

            match cipher.encrypt(&digits) {
                Ok(encrypted) => encrypted,
                Err(_)        => cpf.to_string(),
            }
        })
        .into_owned()
}

fn followed_by_id_suffix(s: &str, end: usize) -> bool {
    let rest = &s[end..];
    let mut chars = rest.chars();
    match chars.next() {
        Some('-') => matches!(chars.next(), Some(c) if c.is_ascii_digit() || c == 'K' || c == 'k'),
        _ => false,
    }
}

const NIT_WEIGHTS: &[u32] = &[3, 7, 13, 17, 19, 23, 29, 37, 41];

fn valid_nit(body: &str, check: &str) -> bool {
    let digits: Vec<u32> = body.chars().filter_map(|c| c.to_digit(10)).collect();
    if digits.len() != 9 { return false; }
    let check_digit = match check.chars().next().and_then(|c| c.to_digit(10)) {
        Some(d) => d,
        None => return false,
    };
    let sum: u32 = digits.iter().rev().zip(NIT_WEIGHTS.iter()).map(|(d, w)| d * w).sum();
    let remainder = sum % 11;
    let expected = match remainder {
        0 | 1 => remainder,
        _ => 11 - remainder,
    };
    check_digit == expected
}

pub fn contains_arg_dni(s: &str) -> bool {
    ARG_DNI_RE.find_iter(s).any(|m| !followed_by_id_suffix(s, m.end()))
}

pub fn mask_arg_dni(s: &str) -> String {
    ARG_DNI_RE
        .replace_all(s, |caps: &regex::Captures| {
            let m = caps.get(0).unwrap();
            if followed_by_id_suffix(s, m.end()) {
                return m.as_str().to_string();
            }
            m.as_str().chars().map(|c| if c.is_ascii_digit() { '*' } else { c }).collect()
        })
        .into_owned()
}

pub fn mask_arg_dni_fpe(s: &str, cipher: &crate::patterns::fpe::Ff3Cipher) -> String {
    ARG_DNI_RE
        .replace_all(s, |caps: &regex::Captures| {
            let m = caps.get(0).unwrap();
            if followed_by_id_suffix(s, m.end()) {
                return m.as_str().to_string();
            }
            let digits: String = m.as_str().chars().filter(|c| c.is_ascii_digit()).collect();
            match cipher.encrypt(&digits) {
                Ok(enc) => enc,
                Err(_)  => m.as_str().to_string(),
            }
        })
        .into_owned()
}

pub fn contains_co_cc(s: &str) -> bool {
    CO_CC_RE.find_iter(s).any(|m| !followed_by_id_suffix(s, m.end()))
}

pub fn mask_co_cc(s: &str) -> String {
    CO_CC_RE
        .replace_all(s, |caps: &regex::Captures| {
            let m = caps.get(0).unwrap();
            if followed_by_id_suffix(s, m.end()) {
                return m.as_str().to_string();
            }
            m.as_str().chars().map(|c| if c.is_ascii_digit() { '*' } else { c }).collect()
        })
        .into_owned()
}

pub fn mask_co_cc_fpe(s: &str, cipher: &crate::patterns::fpe::Ff3Cipher) -> String {
    CO_CC_RE
        .replace_all(s, |caps: &regex::Captures| {
            let m = caps.get(0).unwrap();
            if followed_by_id_suffix(s, m.end()) {
                return m.as_str().to_string();
            }
            let digits: String = m.as_str().chars().filter(|c| c.is_ascii_digit()).collect();
            match cipher.encrypt(&digits) {
                Ok(enc) => enc,
                Err(_)  => m.as_str().to_string(),
            }
        })
        .into_owned()
}

pub fn contains_co_nit(s: &str) -> bool {
    CO_NIT_RE.captures_iter(s).any(|caps| valid_nit(&caps[1], &caps[2]))
}

pub fn mask_co_nit(s: &str) -> String {
    CO_NIT_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_nit(&caps[1], &caps[2]) {
                return caps[0].to_string();
            }
            format!("*********-{}", &caps[2])
        })
        .into_owned()
}

pub fn mask_co_nit_fpe(s: &str, cipher: &crate::patterns::fpe::Ff3Cipher) -> String {
    CO_NIT_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_nit(&caps[1], &caps[2]) {
                return caps[0].to_string();
            }
            match cipher.encrypt(&caps[1]) {
                Ok(enc) => format!("{}-{}", enc, &caps[2]),
                Err(_)  => caps[0].to_string(),
            }
        })
        .into_owned()
}

pub fn mask_rut_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher) -> String {
    RUT_RE
        .replace_all(s, |caps: &regex::Captures| {
            let rut = &caps[0];
            if !valid_rut(rut) {
                return rut.to_string();
            }
            let clean: String = rut.chars().filter(|c| c.is_alphanumeric()).collect();
            let body = &clean[..clean.len() - 1];
            let dv   = &clean[clean.len() - 1..];
            match hasher.encrypt(body) {
                Ok(hashed) => format!("{}-{}", hashed, dv),
                Err(_)     => rut.to_string(),
            }
        })
        .into_owned()
}

pub fn mask_cpf_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher) -> String {
    CPF_RE
        .replace_all(s, |caps: &regex::Captures| {
            let cpf = &caps[0];
            if !valid_cpf(cpf) {
                return cpf.to_string();
            }
            let digits: String = cpf.chars().filter(|c| c.is_ascii_digit()).collect();
            match hasher.encrypt(&digits) {
                Ok(hashed) => hashed,
                Err(_)     => cpf.to_string(),
            }
        })
        .into_owned()
}

pub fn mask_arg_dni_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher) -> String {
    ARG_DNI_RE
        .replace_all(s, |caps: &regex::Captures| {
            let m = caps.get(0).unwrap();
            if followed_by_id_suffix(s, m.end()) {
                return m.as_str().to_string();
            }
            let digits: String = m.as_str().chars().filter(|c| c.is_ascii_digit()).collect();
            match hasher.encrypt(&digits) {
                Ok(hashed) => hashed,
                Err(_)     => m.as_str().to_string(),
            }
        })
        .into_owned()
}

pub fn mask_co_cc_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher) -> String {
    CO_CC_RE
        .replace_all(s, |caps: &regex::Captures| {
            let m = caps.get(0).unwrap();
            if followed_by_id_suffix(s, m.end()) {
                return m.as_str().to_string();
            }
            let digits: String = m.as_str().chars().filter(|c| c.is_ascii_digit()).collect();
            match hasher.encrypt(&digits) {
                Ok(hashed) => hashed,
                Err(_)     => m.as_str().to_string(),
            }
        })
        .into_owned()
}

pub fn mask_co_nit_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher) -> String {
    CO_NIT_RE
        .replace_all(s, |caps: &regex::Captures| {
            if !valid_nit(&caps[1], &caps[2]) {
                return caps[0].to_string();
            }
            match hasher.encrypt(&caps[1]) {
                Ok(hashed) => format!("{}-{}", hashed, &caps[2]),
                Err(_)     => caps[0].to_string(),
            }
        })
        .into_owned()
}