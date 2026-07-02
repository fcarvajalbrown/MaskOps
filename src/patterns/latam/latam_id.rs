

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

static CNPJ_TAIL_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^/\d{4}-?\d{2}\b").unwrap()
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
        .find(|m| !part_of_larger_id(s, m.end()))
        .map(|m| m.as_str().to_string())
}

pub fn extract_co_cc(s: &str) -> Option<String> {
    CO_CC_RE.find_iter(s)
        .find(|m| !part_of_larger_id(s, m.end()))
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

pub fn mask_rut_counted(s: &str) -> (String, u32) {
    crate::patterns::replace_counted(&RUT_RE, s, |caps: &regex::Captures| {
        let rut = &caps[0];
        if !valid_rut(rut) {
            return None;
        }
        let dv = &rut[rut.len() - 1..];
        let body_len = rut.len() - 2;
        Some(format!("{}-{}", "*".repeat(body_len), dv))
    })
}

pub fn mask_rut(s: &str) -> String {
    mask_rut_counted(s).0
}

pub fn contains_cpf(s: &str) -> bool {
    CPF_RE.find_iter(s).any(|m| valid_cpf(m.as_str()))
}

pub fn mask_cpf_counted(s: &str) -> (String, u32) {
    crate::patterns::replace_counted(&CPF_RE, s, |caps: &regex::Captures| {
        let cpf = &caps[0];
        if !valid_cpf(cpf) {
            return None;
        }
        let digits: String = cpf.chars().filter(|c| c.is_ascii_digit()).collect();
        let check = &digits[9..];
        Some(format!("{}-{}", "*".repeat(9), check))
    })
}

pub fn mask_cpf(s: &str) -> String {
    mask_cpf_counted(s).0
}

pub fn contains_curp(s: &str) -> bool {
    CURP_RE.is_match(s)
}

pub fn mask_curp_counted(s: &str) -> (String, u32) {
    crate::patterns::replace_counted(&CURP_RE, s, |caps: &regex::Captures| {
        Some("*".repeat(caps[0].len()))
    })
}

pub fn mask_curp(s: &str) -> String {
    mask_curp_counted(s).0
}

fn mask_rut_with(
    s: &str,
    claims: &crate::patterns::TokenClaims,
    encrypt: &dyn Fn(&str) -> Option<String>,
) -> String {
    RUT_RE
        .replace_all(s, |caps: &regex::Captures| {
            let m = caps.get(0).unwrap();
            let rut = m.as_str();
            if !valid_rut(rut) || !claims.is_free(m.start(), m.end()) {
                return rut.to_string();
            }
            let clean: String = rut.chars().filter(|c| c.is_alphanumeric()).collect();
            let body = &clean[..clean.len() - 1];
            let dv   = &clean[clean.len() - 1..];
            let body_template = &rut[..rut.len() - 2];

            match encrypt(body) {
                Some(encrypted) => {
                    claims.claim(m.start(), m.end());
                    format!("{}-{}", crate::patterns::reinsert_digits(body_template, &encrypted), dv)
                }
                None => rut.to_string(),
            }
        })
        .into_owned()
}

pub fn mask_rut_fpe(s: &str, cipher: &crate::patterns::fpe::FpeCipher, claims: &crate::patterns::TokenClaims) -> String {
    mask_rut_with(s, claims, &|d| cipher.encrypt(d).ok())
}

fn mask_cpf_with(
    s: &str,
    claims: &crate::patterns::TokenClaims,
    encrypt: &dyn Fn(&str) -> Option<String>,
) -> String {
    CPF_RE
        .replace_all(s, |caps: &regex::Captures| {
            let m = caps.get(0).unwrap();
            let cpf = m.as_str();
            if !valid_cpf(cpf) || !claims.is_free(m.start(), m.end()) {
                return cpf.to_string();
            }
            let digits: String = cpf.chars().filter(|c| c.is_ascii_digit()).collect();
            match encrypt(&digits) {
                Some(encrypted) => {
                    claims.claim(m.start(), m.end());
                    crate::patterns::reinsert_digits(cpf, &encrypted)
                }
                None => cpf.to_string(),
            }
        })
        .into_owned()
}

pub fn mask_cpf_fpe(s: &str, cipher: &crate::patterns::fpe::FpeCipher, claims: &crate::patterns::TokenClaims) -> String {
    mask_cpf_with(s, claims, &|d| cipher.encrypt(d).ok())
}

fn followed_by_id_suffix(s: &str, end: usize) -> bool {
    let rest = &s[end..];
    let mut chars = rest.chars();
    match chars.next() {
        Some('-') => matches!(chars.next(), Some(c) if c.is_ascii_digit() || c == 'K' || c == 'k'),
        _ => false,
    }
}

fn followed_by_cnpj_tail(s: &str, end: usize) -> bool {
    CNPJ_TAIL_RE.is_match(&s[end..])
}

fn part_of_larger_id(s: &str, end: usize) -> bool {
    followed_by_id_suffix(s, end) || followed_by_cnpj_tail(s, end)
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
    ARG_DNI_RE.find_iter(s).any(|m| !part_of_larger_id(s, m.end()))
}

pub fn mask_arg_dni_counted(s: &str) -> (String, u32) {
    crate::patterns::replace_counted(&ARG_DNI_RE, s, |caps: &regex::Captures| {
        let m = caps.get(0).unwrap();
        if part_of_larger_id(s, m.end()) {
            return None;
        }
        Some(m.as_str().chars().map(|c| if c.is_ascii_digit() { '*' } else { c }).collect())
    })
}

pub fn mask_arg_dni(s: &str) -> String {
    mask_arg_dni_counted(s).0
}

pub fn mask_arg_dni_fpe(s: &str, cipher: &crate::patterns::fpe::FpeCipher, claims: &crate::patterns::TokenClaims) -> String {
    crate::patterns::mask_family(&ARG_DNI_RE, s, claims,
        &|_, _, end| !part_of_larger_id(s, end), &|d| cipher.encrypt(d).ok())
}

pub fn contains_co_cc(s: &str) -> bool {
    CO_CC_RE.find_iter(s).any(|m| !part_of_larger_id(s, m.end()))
}

pub fn mask_co_cc_counted(s: &str) -> (String, u32) {
    crate::patterns::replace_counted(&CO_CC_RE, s, |caps: &regex::Captures| {
        let m = caps.get(0).unwrap();
        if part_of_larger_id(s, m.end()) {
            return None;
        }
        Some(m.as_str().chars().map(|c| if c.is_ascii_digit() { '*' } else { c }).collect())
    })
}

pub fn mask_co_cc(s: &str) -> String {
    mask_co_cc_counted(s).0
}

pub fn mask_co_cc_fpe(s: &str, cipher: &crate::patterns::fpe::FpeCipher, claims: &crate::patterns::TokenClaims) -> String {
    crate::patterns::mask_family(&CO_CC_RE, s, claims,
        &|_, _, end| !part_of_larger_id(s, end), &|d| cipher.encrypt(d).ok())
}

pub fn contains_co_nit(s: &str) -> bool {
    CO_NIT_RE.captures_iter(s).any(|caps| valid_nit(&caps[1], &caps[2]))
}

pub fn mask_co_nit_counted(s: &str) -> (String, u32) {
    crate::patterns::replace_counted(&CO_NIT_RE, s, |caps: &regex::Captures| {
        if !valid_nit(&caps[1], &caps[2]) {
            return None;
        }
        Some(format!("*********-{}", &caps[2]))
    })
}

pub fn mask_co_nit(s: &str) -> String {
    mask_co_nit_counted(s).0
}

fn mask_co_nit_with(
    s: &str,
    claims: &crate::patterns::TokenClaims,
    encrypt: &dyn Fn(&str) -> Option<String>,
) -> String {
    CO_NIT_RE
        .replace_all(s, |caps: &regex::Captures| {
            let m = caps.get(0).unwrap();
            if !valid_nit(&caps[1], &caps[2]) || !claims.is_free(m.start(), m.end()) {
                return caps[0].to_string();
            }
            match encrypt(&caps[1]) {
                Some(enc) => {
                    claims.claim(m.start(), m.end());
                    format!("{}-{}", enc, &caps[2])
                }
                None => caps[0].to_string(),
            }
        })
        .into_owned()
}

pub fn mask_co_nit_fpe(s: &str, cipher: &crate::patterns::fpe::FpeCipher, claims: &crate::patterns::TokenClaims) -> String {
    mask_co_nit_with(s, claims, &|d| cipher.encrypt(d).ok())
}

pub fn mask_rut_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher, claims: &crate::patterns::TokenClaims) -> String {
    mask_rut_with(s, claims, &|d| hasher.encrypt(d).ok())
}

pub fn mask_cpf_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher, claims: &crate::patterns::TokenClaims) -> String {
    mask_cpf_with(s, claims, &|d| hasher.encrypt(d).ok())
}

pub fn mask_arg_dni_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher, claims: &crate::patterns::TokenClaims) -> String {
    crate::patterns::mask_family(&ARG_DNI_RE, s, claims,
        &|_, _, end| !part_of_larger_id(s, end), &|d| hasher.encrypt(d).ok())
}

pub fn mask_co_cc_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher, claims: &crate::patterns::TokenClaims) -> String {
    crate::patterns::mask_family(&CO_CC_RE, s, claims,
        &|_, _, end| !part_of_larger_id(s, end), &|d| hasher.encrypt(d).ok())
}

pub fn mask_co_nit_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher, claims: &crate::patterns::TokenClaims) -> String {
    mask_co_nit_with(s, claims, &|d| hasher.encrypt(d).ok())
}