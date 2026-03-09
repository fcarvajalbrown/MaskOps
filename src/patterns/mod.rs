//! PII pattern modules.
//!
//! Each submodule exposes:
//! - A `mask_*` function: replaces matched PII with `*` chars
//! - A `contains_*` function: boolean detection only

pub mod country_codes;
pub mod email;
pub mod iban;
pub mod phone;
pub mod vat;

use crate::patterns::iban::{mask_iban, contains_iban};
use crate::patterns::vat::{mask_vat, contains_vat};
use crate::patterns::email::{mask_email, contains_email};
use crate::patterns::phone::{mask_phone, contains_phone};

/// Applies all available masking patterns sequentially to the input string.
/// Order: IBAN → VAT → Email → Phone
pub fn mask_all(value: &str) -> String {
    let s = mask_iban(value);
    let s = mask_vat(&s);
    let s = mask_email(&s);
    let s = mask_phone(&s);
    s
}

/// Returns true if any known PII pattern is found in the string.
pub fn contains_any_pii(value: &str) -> bool {
    contains_iban(value) || contains_vat(value) || contains_email(value) || contains_phone(value)
}