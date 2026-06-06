pub mod canada_sin;
pub mod australia_tfn;

pub use canada_sin::{contains_sin, mask_sin, extract_sin, mask_sin_fpe, mask_sin_consistent};
pub use australia_tfn::{contains_tfn, mask_tfn, extract_tfn, mask_tfn_fpe, mask_tfn_consistent};
