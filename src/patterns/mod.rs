pub mod country_codes;
pub mod email;
pub mod iban;
pub mod ip;
pub mod phone;
pub mod rut;
pub mod vat;

use crate::patterns::iban::{mask_iban, contains_iban};
use crate::patterns::vat::{mask_vat, contains_vat};
use crate::patterns::email::{mask_email, contains_email};
use crate::patterns::phone::{mask_phone, contains_phone};
use crate::patterns::ip::{mask_ip, contains_ip};
use crate::patterns::rut::{mask_rut, contains_rut};

/// Applies all available masking patterns sequentially to the input string.
/// Order: IBAN → VAT → Email → Phone → IP → RUT
pub fn mask_all(value: &str) -> String {
    let s = mask_iban(value);
    let s = mask_vat(&s);
    let s = mask_email(&s);
    let s = mask_phone(&s);
    let s = mask_ip(&s);
    let s = mask_rut(&s);
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
}