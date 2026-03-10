//! Format-Preserving Encryption (FPE) using FF3-1 (NIST SP 800-38G Rev. 1).
//!
//! Implements radix-10 FF3-1 for pseudonymising digit-based PII:
//!   - Credit cards (15–16 digits)
//!   - Phone numbers (E.164 digit sequence, prefix stripped)
//!   - RUT (Chile) digit body
//!   - CPF (Brazil) digit body
//!
//! FF3-1 is GDPR-compliant pseudonymisation when the key is kept separate
//! from the data. The tweak (7 bytes) should encode a tenant or dataset
//! identifier to prevent cross-context decryption.
//!
//! # Security properties
//!   - AES-256 (32-byte key)
//!   - 7-byte caller-supplied tweak
//!   - Format preserved: n digits in → n digits out
//!   - Reversible: encrypt then decrypt returns original
//!   - Minimum input length: 2 digits (FF3-1 spec minimum for radix-10)
//!   - Maximum input length: 30 digits (radix-10 FF3-1 practical limit)
//!
//! # References
//!   - NIST SP 800-38G Rev. 1: https://doi.org/10.6028/NIST.SP.800-38Gr1-draft
//!   - FF3-1 attack surface: https://eprint.iacr.org/2021/1065

use aes::Aes256;
use aes::cipher::{BlockEncrypt, KeyInit, generic_array::GenericArray};

// ── Constants ─────────────────────────────────────────────────────────────────

/// Radix for digit-domain FPE (decimal).
const RADIX: u64 = 10;

/// FF3-1 tweak length in bytes (fixed by spec).
pub const TWEAK_LEN: usize = 7;

/// FF3-1 key length in bytes (AES-256).
pub const KEY_LEN: usize = 32;

/// Minimum plaintext length for radix-10 FF3-1.
/// Derived from spec constraint: radix^minlen >= 100.
const MIN_LEN: usize = 2;

/// Maximum practical plaintext length for radix-10 FF3-1.
const MAX_LEN: usize = 30;

// ── Ff3Cipher ─────────────────────────────────────────────────────────────────

/// FF3-1 cipher context for radix-10 format-preserving encryption.
///
/// Create once and reuse across rows — AES key schedule is computed at
/// construction time and cached inside the `Aes256` instance.
pub struct Ff3Cipher {
    /// AES-256 cipher keyed with K (used on even rounds).
    aes_enc: Aes256,
    /// AES-256 cipher keyed with REV(K) (used on odd rounds).
    aes_dec: Aes256,
    /// 7-byte tweak supplied by caller.
    tweak: [u8; TWEAK_LEN],
}

impl Ff3Cipher {
    /// Construct a new FF3-1 cipher context.
    ///
    /// # Arguments
    /// * `key`   — 32-byte AES-256 key (kept separate from data for GDPR compliance)
    /// * `tweak` — 7-byte context identifier (e.g. tenant ID, dataset name truncated/hashed)
    pub fn new(key: &[u8; KEY_LEN], tweak: &[u8; TWEAK_LEN]) -> Self {
        // FF3-1 uses the byte-reversed key for odd rounds.
        let mut rev_key = *key;
        rev_key.reverse();

        let aes_enc = Aes256::new(GenericArray::from_slice(key));
        let aes_dec = Aes256::new(GenericArray::from_slice(&rev_key));

        Self { aes_enc, aes_dec, tweak: *tweak }
    }

    /// Encrypt a string of decimal digits using FF3-1.
    ///
    /// Returns the ciphertext as a decimal digit string of the same length.
    pub fn encrypt(&self, plaintext: &str) -> Result<String, FpeError> {
        let nums = parse_digits(plaintext)?;
        let result = self.ff3_feistel(&nums, true)?;
        Ok(digits_to_string(&result))
    }

    /// Decrypt a string of decimal digits using FF3-1.
    ///
    /// Returns the plaintext as a decimal digit string of the same length.
    ///
    /// # GDPR note
    /// Decryption capability should be restricted to access-controlled services
    /// separate from the masking pipeline. Do not expose this in ETL or ingestion
    /// contexts where `mask_pii_fpe` is used.
    pub fn decrypt(&self, ciphertext: &str) -> Result<String, FpeError> {
        let nums = parse_digits(ciphertext)?;
        let result = self.ff3_feistel(&nums, false)?;
        Ok(digits_to_string(&result))
    }

    // ── FF3-1 core ────────────────────────────────────────────────────────────

    /// Core FF3-1 Feistel network (NIST SP 800-38G Rev.1, Section 4).
    ///
    /// Encrypt (forward=true,  rounds 0→7):
    ///   m  = u if i even, v if i odd
    ///   P  = tweak_half(i) ^ [i]^1 || REV(B) as 12-byte int
    ///   S  = AES_K(P) for even, AES_REVK(P) for odd
    ///   c  = (NUMradix(REV(A)) + NUMradix(S)) mod radix^m
    ///   C  = REV(STRm(c));  A = B; B = C
    ///
    /// Decrypt (forward=false, rounds 7→0):
    ///   Same structure but subtract instead of add,
    ///   and A/B are initialised swapped.
    fn ff3_feistel(&self, x: &[u8], forward: bool) -> Result<Vec<u8>, FpeError> {
        let n = x.len();
        validate_len(n)?;

        let u = (n + 1) / 2; // left half length (ceiling)
        let v = n - u;        // right half length (floor)

        // For decrypt the halves are swapped at entry and swapped back at exit.
        let (mut a, mut b) = if forward {
            (x[..u].to_vec(), x[u..].to_vec())
        } else {
            (x[u..].to_vec(), x[..u].to_vec())
        };

        let rounds: Vec<u8> = if forward {
            (0u8..8).collect()
        } else {
            (0u8..8).rev().collect()
        };

        for i in rounds {
            let m   = if i % 2 == 0 { u } else { v };
            let aes = if i % 2 == 0 { &self.aes_enc } else { &self.aes_dec };

            // Build 16-byte P block.
            let p = self.build_p(i, &b);

            // S = AES(P), interpreted as big-endian u128.
            let s = self.prf(aes, &p);
            let y = bytes_to_u128_be(&s);

            // NUMradix(REV(A))
            let mut a_rev = a.clone();
            a_rev.reverse();
            let num_a   = num_from_digits(&a_rev);
            let modulus = radix_pow(m);

            let c = if forward {
                (num_a + y % modulus) % modulus
            } else {
                (num_a + modulus - (y % modulus)) % modulus
            };

            // C = REV(STRm(c));  swap A = B, B = C
            let mut c_digits = digits_of(c, m);
            c_digits.reverse();
            a = b;
            b = c_digits;
        }

        // Re-assemble: for decrypt swap back.
        let result = if forward {
            let mut r = a; r.extend_from_slice(&b); r
        } else {
            let mut r = b; r.extend_from_slice(&a); r
        };

        Ok(result)
    }

    /// Build the 16-byte P block for round i.
    ///
    /// P[0..4]  = tweak half XOR i in last byte
    /// P[4..8]  = other tweak half (zeroed last byte)
    /// P[8..16] = NUMradix(REV(B)) as 8-byte big-endian integer
    fn build_p(&self, round: u8, b: &[u8]) -> [u8; 16] {
        let mut p = [0u8; 16];
        let t = &self.tweak;

        // Even rounds use Wr = T[4..7]; odd rounds use Wl = T[0..3].
        // The round byte is XOR'd into the last byte of the selected half.
        if round % 2 == 0 {
            p[0] = t[4];
            p[1] = t[5];
            p[2] = t[6];
            p[3] = round;
            p[4] = t[0];
            p[5] = t[1];
            p[6] = t[2];
            p[7] = t[3];
        } else {
            p[0] = t[0];
            p[1] = t[1];
            p[2] = t[2];
            p[3] = t[3] ^ round;
            p[4] = t[4];
            p[5] = t[5];
            p[6] = t[6];
            p[7] = 0;
        }

        // REV(B) as integer into p[8..16].
        let mut b_rev = b.to_vec();
        b_rev.reverse();
        let num_b = num_from_digits(&b_rev);
        let num_b_bytes = num_b.to_be_bytes();
        p[8..16].copy_from_slice(&num_b_bytes[8..]);

        p
    }

    /// PRF: one AES block encryption.
    fn prf(&self, aes: &Aes256, block: &[u8; 16]) -> [u8; 16] {
        let mut out = GenericArray::clone_from_slice(block);
        aes.encrypt_block(&mut out);
        out.into()
    }
}

// ── FpeError ──────────────────────────────────────────────────────────────────

/// Errors returned by FPE operations.
#[derive(Debug, PartialEq)]
pub enum FpeError {
    /// Input contains characters outside the radix alphabet.
    InvalidCharacter(char),
    /// Input length is outside [MIN_LEN, MAX_LEN].
    InvalidLength(usize),
}

impl std::fmt::Display for FpeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FpeError::InvalidCharacter(c) => write!(f, "FPE: invalid character '{}'", c),
            FpeError::InvalidLength(n) =>
                write!(f, "FPE: length {} out of range [{}, {}]", n, MIN_LEN, MAX_LEN),
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Parse a decimal digit string into a Vec<u8> of digit values (0–9).
fn parse_digits(s: &str) -> Result<Vec<u8>, FpeError> {
    let mut out = Vec::with_capacity(s.len());
    for c in s.chars() {
        match c.to_digit(10) {
            Some(d) => out.push(d as u8),
            None    => return Err(FpeError::InvalidCharacter(c)),
        }
    }
    validate_len(out.len())?;
    Ok(out)
}

/// Validate digit sequence length against FF3-1 constraints.
fn validate_len(n: usize) -> Result<(), FpeError> {
    if n < MIN_LEN || n > MAX_LEN {
        return Err(FpeError::InvalidLength(n));
    }
    Ok(())
}

/// Convert a slice of digit values to a decimal string.
fn digits_to_string(digits: &[u8]) -> String {
    digits.iter().map(|d| char::from_digit(*d as u32, 10).unwrap()).collect()
}

/// Compute radix^m as u128.
fn radix_pow(m: usize) -> u128 {
    (RADIX as u128).pow(m as u32)
}

/// Interpret a digit slice as a base-10 integer.
fn num_from_digits(digits: &[u8]) -> u128 {
    digits.iter().fold(0u128, |acc, &d| acc * RADIX as u128 + d as u128)
}

/// Decompose a u128 into exactly `len` base-10 digits (big-endian, zero-padded).
fn digits_of(mut n: u128, len: usize) -> Vec<u8> {
    let mut out = vec![0u8; len];
    for i in (0..len).rev() {
        out[i] = (n % RADIX as u128) as u8;
        n /= RADIX as u128;
    }
    out
}

/// Interpret 16 bytes as a big-endian u128.
fn bytes_to_u128_be(b: &[u8; 16]) -> u128 {
    u128::from_be_bytes(*b)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn test_cipher() -> Ff3Cipher {
        Ff3Cipher::new(&[0u8; KEY_LEN], &[0u8; TWEAK_LEN])
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip_card() {
        let cipher = test_cipher();
        let pt = "4111111111111111";
        let ct = cipher.encrypt(pt).unwrap();
        assert_eq!(ct.len(), pt.len());
        assert_ne!(ct, pt);
        assert_eq!(cipher.decrypt(&ct).unwrap(), pt);
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip_phone() {
        let cipher = test_cipher();
        let pt = "14155552671";
        let ct = cipher.encrypt(pt).unwrap();
        assert_eq!(ct.len(), pt.len());
        assert_eq!(cipher.decrypt(&ct).unwrap(), pt);
    }

    #[test]
    fn test_encrypt_preserves_length() {
        let cipher = test_cipher();
        for len in [2, 6, 10, 15, 16, 20, 30] {
            let input: String = "1234567890".chars().cycle().take(len).collect();
            let ct = cipher.encrypt(&input).unwrap();
            assert_eq!(ct.len(), len);
        }
    }

    #[test]
    fn test_different_tweaks_produce_different_output() {
        let key = [0u8; KEY_LEN];
        let c1 = Ff3Cipher::new(&key, &[1u8; TWEAK_LEN]);
        let c2 = Ff3Cipher::new(&key, &[2u8; TWEAK_LEN]);
        let pt = "1234567890123456";
        assert_ne!(c1.encrypt(pt).unwrap(), c2.encrypt(pt).unwrap());
    }

    #[test]
    fn test_invalid_character_rejected() {
        assert!(test_cipher().encrypt("1234X678").is_err());
    }

    #[test]
    fn test_too_short_rejected() {
        assert!(test_cipher().encrypt("1").is_err());
    }

    #[test]
    fn test_too_long_rejected() {
        assert!(test_cipher().encrypt(&"1".repeat(31)).is_err());
    }

    #[test]
    fn test_output_is_all_digits() {
        let ct = test_cipher().encrypt("9876543210123456").unwrap();
        assert!(ct.chars().all(|c| c.is_ascii_digit()));
    }
}