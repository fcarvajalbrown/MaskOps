pub mod latam_id;
pub mod ecuador;
pub mod peru;
pub mod uruguay;
pub mod brazil_cnpj;

pub use ecuador::{contains_ec_cedula, mask_ec_cedula, extract_ec_cedula, mask_ec_cedula_fpe, mask_ec_cedula_consistent};
pub use peru::{
    contains_pe_dni, mask_pe_dni, extract_pe_dni, mask_pe_dni_fpe, mask_pe_dni_consistent,
    contains_pe_dni_bare, mask_pe_dni_bare, extract_pe_dni_bare,
    mask_pe_dni_bare_fpe, mask_pe_dni_bare_consistent,
};
pub use uruguay::{contains_uy_ci, mask_uy_ci, extract_uy_ci, mask_uy_ci_fpe, mask_uy_ci_consistent};
pub use brazil_cnpj::{contains_cnpj, mask_cnpj, extract_cnpj, mask_cnpj_fpe, mask_cnpj_consistent};
