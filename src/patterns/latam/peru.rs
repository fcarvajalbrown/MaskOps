//! Peruvian DNI (Documento Nacional de Identidad) detection and masking.
//!
//! Format: 8 digits. No check digit — format-only validation.
//! Issued by RENIEC. Valid range: 00000001–99999999.
//!
//! Compliance: Peru Ley 29733 (Protección de Datos Personales).
//! Digit-based PII — supports FPE.
//! GDPR Art. 4(5): FPE output is pseudonymization, not anonymization.
//!
//! Note: the bare 8-digit pattern is also matched by ARG DNI (dotted format).
//! Peruvian DNI appears in compact form (no dots) in documents.

use once_cell::sync::Lazy;
use regex::Regex;

/// Matches exactly 8 consecutive digits not preceded/followed by more digits.
static PE_DNI_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b([0-9]{8})\b").unwrap()
});

/// Returns true if the input contains a Peruvian DNI (8 compact digits).
pub fn contains_pe_dni(s: &str) -> bool {
    PE_DNI_RE.is_match(s)
}

/// Masks any Peruvian DNI found (full redaction).
///
/// Example: `12345678` → `********`
pub fn mask_pe_dni(s: &str) -> String {
    PE_DNI_RE
        .replace_all(s, "********")
        .into_owned()
}

/// Masks a Peruvian DNI using HMAC-SHA256 consistent pseudonymization on all 8 digits.
///
/// Not reversible without salt.
pub fn mask_pe_dni_consistent(s: &str, hasher: &crate::patterns::consistent::ConsistentHasher) -> String {
    PE_DNI_RE
        .replace_all(s, |caps: &regex::Captures| {
            match hasher.encrypt(&caps[0]) {
                Ok(hashed) => hashed,
                Err(_)     => caps[0].to_string(),
            }
        })
        .into_owned()
}

/// Masks a Peruvian DNI using FF3-1 FPE on all 8 digits.
///
/// Reversible with the same key and tweak.
pub fn mask_pe_dni_fpe(s: &str, cipher: &crate::patterns::fpe::Ff3Cipher) -> String {
    PE_DNI_RE
        .replace_all(s, |caps: &regex::Captures| {
            match cipher.encrypt(&caps[0]) {
                Ok(enc) => enc,
                Err(_)  => caps[0].to_string(),
            }
        })
        .into_owned()
}
