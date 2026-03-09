pub mod country_codes;
pub mod email;
pub mod iban;
pub mod ip;
pub mod phone;
pub mod vat;
pub mod latam_id;
pub mod credit_card;
pub mod european_id;

use crate::patterns::iban::{mask_iban, contains_iban};
use crate::patterns::vat::{mask_vat, contains_vat};
use crate::patterns::email::{mask_email, contains_email};
use crate::patterns::phone::{mask_phone, contains_phone};
use crate::patterns::ip::{mask_ip, contains_ip};
use crate::patterns::latam_id::{
    mask_rut, contains_rut, mask_cpf, contains_cpf, mask_curp, contains_curp,
};
use crate::patterns::credit_card::{mask_card, contains_card};
use crate::patterns::european_id::{
    mask_dni, contains_dni, mask_nie, contains_nie, mask_nin, contains_nin,
}; 

/// Applies all available masking patterns sequentially to the input string.
/// Order: IBAN → VAT → Email → Phone → IP → RUT → CPF → CURP → Card
pub fn mask_all(value: &str) -> String {
    let s = mask_iban(value);
    let s = mask_vat(&s);
    let s = mask_email(&s);
    let s = mask_phone(&s);
    let s = mask_ip(&s);
    let s = mask_rut(&s);
    let s = mask_cpf(&s);
    let s = mask_curp(&s);
    let s = mask_card(&s);
    let s = mask_dni(&s);
    let s = mask_nie(&s);
    let s = mask_nin(&s);
    s
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