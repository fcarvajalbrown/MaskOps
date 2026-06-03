pub mod latam_id;
pub mod ecuador;
pub mod peru;
pub mod uruguay;

pub use ecuador::{contains_ec_cedula, mask_ec_cedula, mask_ec_cedula_fpe, mask_ec_cedula_consistent};
pub use peru::{contains_pe_dni, mask_pe_dni, mask_pe_dni_fpe, mask_pe_dni_consistent};
pub use uruguay::{contains_uy_ci, mask_uy_ci, mask_uy_ci_fpe, mask_uy_ci_consistent};
