//! Consistent hash-based pseudonymization (one-way, deterministic).
//!
//! GDPR: pseudonymization (Art. 4(5)), not anonymization. Salt is a secret.
//! Compliance: all digit PII families; HMAC-SHA256; not reversible without salt.

use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub struct ConsistentHasher {
    salt: Vec<u8>,
}

impl ConsistentHasher {
    pub fn new(salt: &str) -> Self {
        Self { salt: salt.as_bytes().to_vec() }
    }

    /// Returns a same-length deterministic digit string for the given digit input.
    ///
    /// HMAC-SHA256(salt, digits) → first n digits from hash bytes (each byte mod 10).
    /// Slight statistical bias (~2.4%) is acceptable for pseudonymization.
    /// HMAC-SHA256 yields 32 bytes; all PII digit sequences are ≤ 19 chars.
    pub fn encrypt(&self, digits: &str) -> Result<String, ()> {
        let n = digits.len();
        if n == 0 {
            return Ok(String::new());
        }
        let mut mac = HmacSha256::new_from_slice(&self.salt).map_err(|_| ())?;
        mac.update(digits.as_bytes());
        let hash = mac.finalize().into_bytes();
        let out: String = hash
            .iter()
            .map(|b| char::from_digit((b % 10) as u32, 10).unwrap())
            .take(n)
            .collect();
        if out.len() < n {
            return Err(());
        }
        Ok(out)
    }
}
