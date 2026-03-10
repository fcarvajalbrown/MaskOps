pub mod country_codes;
pub mod email;
pub mod iban;
pub mod ip;
pub mod phone;
pub mod vat;
pub mod latam_id;
pub mod credit_card;
pub mod european_id;
pub mod fpe;

use crate::patterns::iban::{mask_iban, contains_iban};
use crate::patterns::vat::{mask_vat, contains_vat};
use crate::patterns::email::{mask_email, contains_email};
use crate::patterns::phone::{mask_phone, contains_phone, mask_phone_fpe};
use crate::patterns::ip::{mask_ip, contains_ip};
use crate::patterns::latam_id::{
    mask_rut, contains_rut, mask_cpf, contains_cpf, mask_curp, contains_curp,
    mask_rut_fpe, mask_cpf_fpe,
};
use crate::patterns::credit_card::{mask_card, contains_card, mask_card_fpe};
use crate::patterns::european_id::{
    mask_dni, contains_dni, mask_nie, contains_nie, mask_nin, contains_nin,
    mask_personalausweis, contains_personalausweis,
};
pub use crate::patterns::fpe::{Ff3Cipher, KEY_LEN, TWEAK_LEN};

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
}