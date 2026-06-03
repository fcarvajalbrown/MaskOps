pub mod latam_id;
pub mod ecuador;
pub mod peru;

pub use ecuador::{contains_ec_cedula, mask_ec_cedula, mask_ec_cedula_fpe, mask_ec_cedula_consistent};
pub use peru::{contains_pe_dni, mask_pe_dni, mask_pe_dni_fpe, mask_pe_dni_consistent};
