use crate::patterns::country_codes::identify_country;
use crate::patterns::fpe::{FpeCipher, FpeError};
use crate::patterns::reinsert_digits;

pub const REKEY_PATTERNS: &[&str] = &[
    "phone", "rut", "cpf", "cnpj", "credit_card", "ssn", "arg_dni", "co_cc", "co_nit",
    "ec_cedula", "pe_dni", "npi", "nhs", "uy_ci", "sin", "tfn", "pesel", "bsn",
    "personnummer", "my_number", "rrn", "za_id", "il_id",
];

pub fn validate_rekey_pattern(pattern: &str) -> Result<(), String> {
    if REKEY_PATTERNS.contains(&pattern) {
        Ok(())
    } else {
        Err(format!(
            "unknown rekey pattern '{}', valid digit families: {}",
            pattern,
            REKEY_PATTERNS.join(", ")
        ))
    }
}

pub fn rekey_cell(cell: &str, pattern: Option<&str>, cipher: &FpeCipher) -> Result<String, FpeError> {
    if !cell.bytes().any(|b| b.is_ascii_digit()) {
        return Ok(cell.to_string());
    }
    match pattern {
        Some("phone") => rekey_phone(cell, cipher),
        Some("rut") | Some("co_nit") => rekey_keep_suffix(cell, cipher),
        _ => rekey_all_digits(cell, cipher),
    }
}

fn rekey_all_digits(cell: &str, cipher: &FpeCipher) -> Result<String, FpeError> {
    let digits: String = cell.chars().filter(|c| c.is_ascii_digit()).collect();
    let rotated = cipher.encrypt(&digits)?;
    Ok(reinsert_digits(cell, &rotated))
}

fn rekey_keep_suffix(cell: &str, cipher: &FpeCipher) -> Result<String, FpeError> {
    match cell.rfind('-') {
        Some(idx) => {
            let (body, suffix) = cell.split_at(idx);
            let digits: String = body.chars().filter(|c| c.is_ascii_digit()).collect();
            let rotated = cipher.encrypt(&digits)?;
            Ok(format!("{}{}", reinsert_digits(body, &rotated), suffix))
        }
        None => rekey_all_digits(cell, cipher),
    }
}

fn rekey_phone(cell: &str, cipher: &FpeCipher) -> Result<String, FpeError> {
    let normalized: String = cell.chars().filter(|c| c.is_ascii_digit() || *c == '+').collect();
    let prefix_len = identify_country(&normalized).map(|cc| cc.prefix.len()).unwrap_or(2);
    let split = prefix_len.min(cell.len());
    let (keep, rest) = cell.split_at(split);
    let subscriber: String = rest.chars().filter(|c| c.is_ascii_digit()).collect();
    let rotated = cipher.encrypt(&subscriber)?;
    Ok(format!("{}{}", keep, reinsert_digits(rest, &rotated)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::patterns::fpe::Ff3Cipher;

    fn old() -> FpeCipher {
        FpeCipher::Ff3(Ff3Cipher::new(&[1u8; 32], &[1u8; 7]))
    }
    fn new() -> FpeCipher {
        FpeCipher::Ff3(Ff3Cipher::new(&[2u8; 32], &[2u8; 7]))
    }
    fn rekey() -> FpeCipher {
        FpeCipher::Rekey(Box::new(old()), Box::new(new()))
    }

    #[test]
    fn test_rekey_ssn_shaped_token() {
        let payload = "123456789";
        let ct_old = old().encrypt(payload).unwrap();
        let token = format!("{}-{}-{}", &ct_old[..3], &ct_old[3..5], &ct_old[5..]);

        let rotated = rekey_cell(&token, Some("ssn"), &rekey()).unwrap();

        let ct_new = new().encrypt(payload).unwrap();
        let expected = format!("{}-{}-{}", &ct_new[..3], &ct_new[3..5], &ct_new[5..]);
        assert_eq!(rotated, expected);
    }

    #[test]
    fn test_rekey_default_handles_separators() {
        let payload = "123456789";
        let ct_old = old().encrypt(payload).unwrap();
        let token = format!("{}-{}-{}", &ct_old[..3], &ct_old[3..5], &ct_old[5..]);

        let rotated = rekey_cell(&token, None, &rekey()).unwrap();

        let ct_new = new().encrypt(payload).unwrap();
        let expected = format!("{}-{}-{}", &ct_new[..3], &ct_new[3..5], &ct_new[5..]);
        assert_eq!(rotated, expected);
    }

    #[test]
    fn test_rekey_rut_keeps_check_digit() {
        let body = "12345678";
        let ct_old = old().encrypt(body).unwrap();
        let token = format!("{}-9", reinsert_digits("12.345.678", &ct_old));

        let rotated = rekey_cell(&token, Some("rut"), &rekey()).unwrap();

        let ct_new = new().encrypt(body).unwrap();
        let expected = format!("{}-9", reinsert_digits("12.345.678", &ct_new));
        assert_eq!(rotated, expected);
        assert!(rotated.ends_with("-9"));
    }

    #[test]
    fn test_rekey_bare_card() {
        let payload = "4111111111111111";
        let ct_old = old().encrypt(payload).unwrap();

        let rotated = rekey_cell(&ct_old, None, &rekey()).unwrap();

        assert_eq!(rotated, new().encrypt(payload).unwrap());
    }

    #[test]
    fn test_rekey_short_token_errors() {
        assert!(rekey_cell("12-3", None, &rekey()).is_err());
    }

    #[test]
    fn test_validate_rejects_non_digit_family() {
        assert!(validate_rekey_pattern("email").is_err());
        assert!(validate_rekey_pattern("ssn").is_ok());
    }
}
