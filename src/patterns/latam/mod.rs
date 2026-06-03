pub mod latam_id;
pub mod ecuador;
pub mod peru;

pub use ecuador::{contains_ec_cedula, mask_ec_cedula, mask_ec_cedula_fpe};
pub use peru::{contains_pe_dni, mask_pe_dni, mask_pe_dni_fpe};
