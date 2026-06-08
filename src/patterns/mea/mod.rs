pub mod south_africa;
pub mod israel;

pub(crate) fn boundary_ok(s: &str, start: usize, end: usize) -> bool {
    let bytes = s.as_bytes();
    let before_ok = start == 0 || bytes[start - 1] != b'-';
    let after_ok = bytes.get(end) != Some(&b'-');
    before_ok && after_ok
}

pub use south_africa::{contains_za_id, mask_za_id, extract_za_id, mask_za_id_fpe, mask_za_id_consistent};
pub use israel::{contains_il_id, mask_il_id, extract_il_id, mask_il_id_fpe, mask_il_id_consistent};
