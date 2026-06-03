pub mod eu;
pub mod latam;
pub mod contact;
pub mod financial;
pub mod us;
pub mod healthcare;
pub mod apac;
pub mod country_codes;
pub mod fpe;
pub mod consistent;

use crate::patterns::eu::iban::{mask_iban, contains_iban};
use crate::patterns::eu::vat::{mask_vat, contains_vat};
use crate::patterns::eu::european_id::{
    mask_dni, contains_dni, mask_nie, contains_nie, mask_nin, contains_nin,
    mask_personalausweis, contains_personalausweis,
};
use crate::patterns::eu::nir::{mask_nir, contains_nir};
use crate::patterns::eu::codice_fiscale::{mask_cf, contains_cf};
use crate::patterns::latam::{contains_uy_ci, mask_uy_ci, mask_uy_ci_fpe, mask_uy_ci_consistent};
use crate::patterns::apac::{
    contains_sin, mask_sin, mask_sin_fpe, mask_sin_consistent,
    contains_tfn, mask_tfn, mask_tfn_fpe, mask_tfn_consistent,
};
use crate::patterns::contact::email::{mask_email, contains_email};
use crate::patterns::contact::phone::{mask_phone, contains_phone, mask_phone_fpe};
use crate::patterns::contact::ip::{mask_ip, contains_ip};
use crate::patterns::latam::{contains_ec_cedula, mask_ec_cedula, mask_ec_cedula_fpe,
                             contains_pe_dni, mask_pe_dni, mask_pe_dni_fpe};
use crate::patterns::healthcare::{
    contains_npi, mask_npi, mask_npi_fpe,
    contains_mbi, mask_mbi,
    contains_nhs, mask_nhs, mask_nhs_fpe,
};
use crate::patterns::latam::latam_id::{
    mask_rut, contains_rut, mask_cpf, contains_cpf, mask_curp, contains_curp,
    mask_rut_fpe, mask_cpf_fpe,
    mask_arg_dni, contains_arg_dni, mask_arg_dni_fpe,
    mask_co_cc, contains_co_cc, mask_co_cc_fpe,
    mask_co_nit, contains_co_nit, mask_co_nit_fpe,
};
use crate::patterns::financial::credit_card::{mask_card, contains_card, mask_card_fpe};
use crate::patterns::us::{mask_ssn, contains_ssn, mask_ssn_fpe, mask_us_passport, contains_us_passport};
pub use crate::patterns::fpe::{Ff3Cipher, KEY_LEN, TWEAK_LEN};
pub use crate::patterns::consistent::ConsistentHasher;
use crate::patterns::contact::phone::mask_phone_consistent;
use crate::patterns::financial::credit_card::mask_card_consistent;
use crate::patterns::latam::latam_id::{
    mask_rut_consistent, mask_cpf_consistent,
    mask_arg_dni_consistent, mask_co_cc_consistent, mask_co_nit_consistent,
};
use crate::patterns::latam::{mask_ec_cedula_consistent, mask_pe_dni_consistent};
use crate::patterns::us::mask_ssn_consistent;
use crate::patterns::healthcare::{mask_npi_consistent, mask_nhs_consistent};

// ---------------------------------------------------------------------------
// Non-digit PII: IBAN, VAT, Email, IP, EU IDs
// Masked with asterisks in both asterisk and FPE modes.
// ---------------------------------------------------------------------------

/// Masks non-digit PII (IBAN, VAT, email, IP, EU IDs) with asterisks.
///
/// Does not touch digit-based PII (cards, phones, RUT, CPF, CURP).
/// Called by both `mask_all` and `mask_all_fpe`.
pub fn mask_non_digit(value: &str) -> String {
    let s = mask_iban(value);
    let s = mask_vat(&s);
    let s = mask_email(&s);
    let s = mask_ip(&s);
    let s = mask_curp(&s);
    let s = mask_dni(&s);
    let s = mask_nie(&s);
    let s = mask_nin(&s);
    let s = mask_personalausweis(&s);
    let s = mask_us_passport(&s);
    let s = mask_mbi(&s);
    let s = mask_nir(&s);
    let s = mask_cf(&s);
    s
}

// ---------------------------------------------------------------------------
// Digit PII: cards, phones, RUT, CPF
// Two masking modes: asterisk and FPE.
// ---------------------------------------------------------------------------

/// Masks digit-based PII (cards, phones, RUT, CPF) with asterisks.
pub fn mask_digit(value: &str) -> String {
    let s = mask_phone(value);
    let s = mask_rut(&s);
    let s = mask_cpf(&s);
    let s = mask_card(&s);
    let s = mask_ssn(&s);
    let s = mask_arg_dni(&s);
    let s = mask_co_cc(&s);
    let s = mask_co_nit(&s);
    let s = mask_ec_cedula(&s);
    let s = mask_pe_dni(&s);
    let s = mask_npi(&s);
    let s = mask_nhs(&s);
    let s = mask_uy_ci(&s);
    let s = mask_sin(&s);
    let s = mask_tfn(&s);
    s
}

/// Masks digit-based PII (cards, phones, RUT, CPF) with FF3-1 FPE.
///
/// Format is preserved — output has the same length and digit structure
/// as the input. Reversible with the same key and tweak.
pub fn mask_digit_fpe(value: &str, cipher: &Ff3Cipher) -> String {
    let s = mask_phone_fpe(value, cipher);
    let s = mask_rut_fpe(&s, cipher);
    let s = mask_cpf_fpe(&s, cipher);
    let s = mask_card_fpe(&s, cipher);
    let s = mask_ssn_fpe(&s, cipher);
    let s = mask_arg_dni_fpe(&s, cipher);
    let s = mask_co_cc_fpe(&s, cipher);
    let s = mask_co_nit_fpe(&s, cipher);
    let s = mask_ec_cedula_fpe(&s, cipher);
    let s = mask_pe_dni_fpe(&s, cipher);
    let s = mask_npi_fpe(&s, cipher);
    let s = mask_nhs_fpe(&s, cipher);
    let s = mask_uy_ci_fpe(&s, cipher);
    let s = mask_sin_fpe(&s, cipher);
    let s = mask_tfn_fpe(&s, cipher);
    s
}

// ---------------------------------------------------------------------------
// Aggregators
// ---------------------------------------------------------------------------

/// Masks all PII with asterisks.
///
/// Order: non-digit PII first, then digit PII.
/// Use `mask_all_fpe` for reversible pseudonymisation of digit PII.
pub fn mask_all(value: &str) -> String {
    let s = mask_non_digit(value);
    mask_digit(&s)
}

/// Masks all PII — non-digit with asterisks, digit with FF3-1 FPE.
///
/// Non-digit PII (IBAN, VAT, email, IP, EU IDs) is still asterisked.
/// Digit PII (cards, phones, RUT, CPF) is pseudonymised with FF3-1.
pub fn mask_all_fpe(value: &str, cipher: &Ff3Cipher) -> String {
    let s = mask_non_digit(value);
    mask_digit_fpe(&s, cipher)
}

/// Masks digit-based PII with HMAC-SHA256 consistent pseudonymization.
///
/// Same input → same output given the same salt. Not reversible without the salt.
pub fn mask_digit_consistent(value: &str, hasher: &ConsistentHasher) -> String {
    let s = mask_phone_consistent(value, hasher);
    let s = mask_rut_consistent(&s, hasher);
    let s = mask_cpf_consistent(&s, hasher);
    let s = mask_card_consistent(&s, hasher);
    let s = mask_ssn_consistent(&s, hasher);
    let s = mask_arg_dni_consistent(&s, hasher);
    let s = mask_co_cc_consistent(&s, hasher);
    let s = mask_co_nit_consistent(&s, hasher);
    let s = mask_ec_cedula_consistent(&s, hasher);
    let s = mask_pe_dni_consistent(&s, hasher);
    let s = mask_npi_consistent(&s, hasher);
    let s = mask_nhs_consistent(&s, hasher);
    let s = mask_uy_ci_consistent(&s, hasher);
    let s = mask_sin_consistent(&s, hasher);
    let s = mask_tfn_consistent(&s, hasher);
    s
}

/// Masks all PII — non-digit with asterisks, digit with HMAC-SHA256 consistent pseudonymization.
///
/// Same input → same output given the same salt. Not reversible without the salt.
pub fn mask_all_consistent(value: &str, hasher: &ConsistentHasher) -> String {
    let s = mask_non_digit(value);
    mask_digit_consistent(&s, hasher)
}

/// Masks only the selected PII patterns with asterisks.
///
/// Pattern names: email, phone, ip, iban, vat, dni, nie, nin, personalausweis,
/// us_passport, curp, rut, cpf, ssn, arg_dni, co_cc, co_nit, ec_cedula, credit_card.
pub fn mask_all_selected(value: &str, patterns: &[&str]) -> String {
    let mut s = value.to_string();
    for pat in patterns {
        s = match *pat {
            "email"           => mask_email(&s),
            "phone"           => mask_phone(&s),
            "ip"              => mask_ip(&s),
            "iban"            => mask_iban(&s),
            "vat"             => mask_vat(&s),
            "dni"             => mask_dni(&s),
            "nie"             => mask_nie(&s),
            "nin"             => mask_nin(&s),
            "personalausweis" => mask_personalausweis(&s),
            "us_passport"     => mask_us_passport(&s),
            "curp"            => mask_curp(&s),
            "rut"             => mask_rut(&s),
            "cpf"             => mask_cpf(&s),
            "ssn"             => mask_ssn(&s),
            "arg_dni"         => mask_arg_dni(&s),
            "co_cc"           => mask_co_cc(&s),
            "co_nit"          => mask_co_nit(&s),
            "ec_cedula"       => mask_ec_cedula(&s),
            "credit_card"     => mask_card(&s),
            "npi"             => mask_npi(&s),
            "mbi"             => mask_mbi(&s),
            "nhs"             => mask_nhs(&s),
            "pe_dni"          => mask_pe_dni(&s),
            "nir"             => mask_nir(&s),
            "codice_fiscale"  => mask_cf(&s),
            "uy_ci"           => mask_uy_ci(&s),
            "sin"             => mask_sin(&s),
            "tfn"             => mask_tfn(&s),
            _                 => s,
        };
    }
    s
}

/// Masks only the selected PII patterns — non-digit asterisked, digit FPE.
pub fn mask_all_selected_fpe(value: &str, patterns: &[&str], cipher: &Ff3Cipher) -> String {
    let mut s = value.to_string();
    for pat in patterns {
        s = match *pat {
            "email"           => mask_email(&s),
            "phone"           => mask_phone_fpe(&s, cipher),
            "ip"              => mask_ip(&s),
            "iban"            => mask_iban(&s),
            "vat"             => mask_vat(&s),
            "dni"             => mask_dni(&s),
            "nie"             => mask_nie(&s),
            "nin"             => mask_nin(&s),
            "personalausweis" => mask_personalausweis(&s),
            "us_passport"     => mask_us_passport(&s),
            "curp"            => mask_curp(&s),
            "rut"             => mask_rut_fpe(&s, cipher),
            "cpf"             => mask_cpf_fpe(&s, cipher),
            "ssn"             => mask_ssn_fpe(&s, cipher),
            "arg_dni"         => mask_arg_dni_fpe(&s, cipher),
            "co_cc"           => mask_co_cc_fpe(&s, cipher),
            "co_nit"          => mask_co_nit_fpe(&s, cipher),
            "ec_cedula"       => mask_ec_cedula_fpe(&s, cipher),
            "credit_card"     => mask_card_fpe(&s, cipher),
            "npi"             => mask_npi_fpe(&s, cipher),
            "mbi"             => mask_mbi(&s),  // alphanumeric — asterisk only
            "nhs"             => mask_nhs_fpe(&s, cipher),
            "pe_dni"          => mask_pe_dni_fpe(&s, cipher),
            "nir"             => mask_nir(&s),       // non-digit — asterisk only
            "codice_fiscale"  => mask_cf(&s),        // non-digit — asterisk only
            "uy_ci"           => mask_uy_ci_fpe(&s, cipher),
            "sin"             => mask_sin_fpe(&s, cipher),
            "tfn"             => mask_tfn_fpe(&s, cipher),
            _                 => s,
        };
    }
    s
}

/// Masks only the selected PII patterns — non-digit asterisked, digit consistently hashed.
pub fn mask_all_selected_consistent(value: &str, patterns: &[&str], hasher: &ConsistentHasher) -> String {
    let mut s = value.to_string();
    for pat in patterns {
        s = match *pat {
            "email"           => mask_email(&s),
            "phone"           => mask_phone_consistent(&s, hasher),
            "ip"              => mask_ip(&s),
            "iban"            => mask_iban(&s),
            "vat"             => mask_vat(&s),
            "dni"             => mask_dni(&s),
            "nie"             => mask_nie(&s),
            "nin"             => mask_nin(&s),
            "personalausweis" => mask_personalausweis(&s),
            "us_passport"     => mask_us_passport(&s),
            "curp"            => mask_curp(&s),
            "rut"             => mask_rut_consistent(&s, hasher),
            "cpf"             => mask_cpf_consistent(&s, hasher),
            "ssn"             => mask_ssn_consistent(&s, hasher),
            "arg_dni"         => mask_arg_dni_consistent(&s, hasher),
            "co_cc"           => mask_co_cc_consistent(&s, hasher),
            "co_nit"          => mask_co_nit_consistent(&s, hasher),
            "ec_cedula"       => mask_ec_cedula_consistent(&s, hasher),
            "credit_card"     => mask_card_consistent(&s, hasher),
            "npi"             => mask_npi_consistent(&s, hasher),
            "mbi"             => mask_mbi(&s),
            "nhs"             => mask_nhs_consistent(&s, hasher),
            "pe_dni"          => mask_pe_dni_consistent(&s, hasher),
            "nir"             => mask_nir(&s),       // non-digit — asterisk only
            "codice_fiscale"  => mask_cf(&s),        // non-digit — asterisk only
            "uy_ci"           => mask_uy_ci_consistent(&s, hasher),
            "sin"             => mask_sin_consistent(&s, hasher),
            "tfn"             => mask_tfn_consistent(&s, hasher),
            _                 => s,
        };
    }
    s
}

/// Returns true if any of the selected PII patterns is found in the string.
pub fn contains_any_selected(value: &str, patterns: &[&str]) -> bool {
    patterns.iter().any(|pat| match *pat {
        "email"           => contains_email(value),
        "phone"           => contains_phone(value),
        "ip"              => contains_ip(value),
        "iban"            => contains_iban(value),
        "vat"             => contains_vat(value),
        "dni"             => contains_dni(value),
        "nie"             => contains_nie(value),
        "nin"             => contains_nin(value),
        "personalausweis" => contains_personalausweis(value),
        "us_passport"     => contains_us_passport(value),
        "curp"            => contains_curp(value),
        "rut"             => contains_rut(value),
        "cpf"             => contains_cpf(value),
        "ssn"             => contains_ssn(value),
        "arg_dni"         => contains_arg_dni(value),
        "co_cc"           => contains_co_cc(value),
        "co_nit"          => contains_co_nit(value),
        "ec_cedula"       => contains_ec_cedula(value),
        "credit_card"     => contains_card(value),
        "npi"             => contains_npi(value),
        "mbi"             => contains_mbi(value),
        "nhs"             => contains_nhs(value),
        "pe_dni"          => contains_pe_dni(value),
        "nir"             => contains_nir(value),
        "codice_fiscale"  => contains_cf(value),
        "uy_ci"           => contains_uy_ci(value),
        "sin"             => contains_sin(value),
        "tfn"             => contains_tfn(value),
        _                 => false,
    })
}

/// Returns true if any known PII pattern is found in the string.
pub fn contains_any_pii(value: &str) -> bool {
    contains_iban(value)
        || contains_vat(value)
        || contains_email(value)
        || contains_phone(value)
        || contains_ip(value)
        || contains_rut(value)
        || contains_cpf(value)
        || contains_curp(value)
        || contains_card(value)
        || contains_dni(value)
        || contains_nie(value)
        || contains_nin(value)
        || contains_personalausweis(value)
        || contains_ssn(value)
        || contains_us_passport(value)
        || contains_arg_dni(value)
        || contains_co_cc(value)
        || contains_co_nit(value)
        || contains_ec_cedula(value)
        || contains_pe_dni(value)
        || contains_npi(value)
        || contains_mbi(value)
        || contains_nhs(value)
        || contains_nir(value)
        || contains_cf(value)
        || contains_uy_ci(value)
        || contains_sin(value)
        || contains_tfn(value)
}