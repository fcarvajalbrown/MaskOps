use aes::Aes256;
use aes::cipher::{BlockEncrypt, KeyInit, generic_array::GenericArray};

pub(crate) const RADIX: u64 = 10;

pub const TWEAK_LEN: usize = 7;

pub const KEY_LEN: usize = 32;

pub(crate) const MIN_LEN: usize = 6;

pub(crate) const MAX_LEN: usize = 30;

pub struct Ff3Cipher {
    aes: Aes256,
    tl: [u8; 4],
    tr: [u8; 4],
}

impl Ff3Cipher {
    pub fn new(key: &[u8; KEY_LEN], tweak: &[u8; TWEAK_LEN]) -> Self {
        let mut rev_key = *key;
        rev_key.reverse();
        let aes = Aes256::new(GenericArray::from_slice(&rev_key));
        let tl = [tweak[0], tweak[1], tweak[2], tweak[3] & 0xF0];
        let tr = [tweak[4], tweak[5], tweak[6], (tweak[3] & 0x0F) << 4];
        Self { aes, tl, tr }
    }

    pub fn encrypt(&self, plaintext: &str) -> Result<String, FpeError> {
        let nums = parse_digits(plaintext)?;
        let result = self.ff3_feistel(&nums, true)?;
        Ok(digits_to_string(&result))
    }

    pub fn decrypt(&self, ciphertext: &str) -> Result<String, FpeError> {
        let nums = parse_digits(ciphertext)?;
        let result = self.ff3_feistel(&nums, false)?;
        Ok(digits_to_string(&result))
    }

    fn ff3_feistel(&self, x: &[u8], forward: bool) -> Result<Vec<u8>, FpeError> {
        let n = x.len();
        validate_len(n)?;

        let u = (n + 1) / 2;
        let v = n - u;

        let mut a = x[..u].to_vec();
        let mut b = x[u..].to_vec();

        if forward {
            for i in 0..8u32 {
                let (m, w) = if i % 2 == 0 { (u, &self.tr) } else { (v, &self.tl) };
                let y = self.round_y(w, i, &b);
                let modulus = radix_pow(m);
                let c = (num_rev(&a) + y % modulus) % modulus;
                a = b;
                b = rev_str_radix(c, m);
            }
        } else {
            for i in (0..8u32).rev() {
                let (m, w) = if i % 2 == 0 { (u, &self.tr) } else { (v, &self.tl) };
                let y = self.round_y(w, i, &a);
                let modulus = radix_pow(m);
                let c = (num_rev(&b) + modulus - (y % modulus)) % modulus;
                b = a;
                a = rev_str_radix(c, m);
            }
        }

        let mut result = a;
        result.extend_from_slice(&b);
        Ok(result)
    }

    fn round_y(&self, w: &[u8; 4], round: u32, half: &[u8]) -> u128 {
        let mut p = [0u8; 16];
        let rb = round.to_be_bytes();
        for k in 0..4 {
            p[k] = w[k] ^ rb[k];
        }
        let num_half = num_rev(half).to_be_bytes();
        p[4..16].copy_from_slice(&num_half[4..]);

        p.reverse();
        let mut block = GenericArray::clone_from_slice(&p);
        self.aes.encrypt_block(&mut block);
        let mut s: [u8; 16] = block.into();
        s.reverse();
        u128::from_be_bytes(s)
    }
}

#[derive(Debug, PartialEq)]
pub enum FpeError {
    InvalidCharacter(char),
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

pub(crate) fn parse_digits(s: &str) -> Result<Vec<u8>, FpeError> {
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

pub(crate) fn validate_len(n: usize) -> Result<(), FpeError> {
    if n < MIN_LEN || n > MAX_LEN {
        return Err(FpeError::InvalidLength(n));
    }
    Ok(())
}

pub(crate) fn digits_to_string(digits: &[u8]) -> String {
    digits.iter().map(|d| char::from_digit(*d as u32, 10).unwrap()).collect()
}

fn radix_pow(m: usize) -> u128 {
    (RADIX as u128).pow(m as u32)
}

fn num_rev(digits: &[u8]) -> u128 {
    digits.iter().rev().fold(0u128, |acc, &d| acc * RADIX as u128 + d as u128)
}

fn rev_str_radix(mut n: u128, len: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(len);
    for _ in 0..len {
        out.push((n % RADIX as u128) as u8);
        n /= RADIX as u128;
    }
    out
}

use crate::patterns::fpe_ff1::Ff1Cipher;

pub enum FpeCipher {
    Ff3(Ff3Cipher),
    Ff1(Ff1Cipher),
    Rekey(Box<FpeCipher>, Box<FpeCipher>),
}

impl FpeCipher {
    pub fn encrypt(&self, plaintext: &str) -> Result<String, FpeError> {
        match self {
            FpeCipher::Ff3(c) => c.encrypt(plaintext),
            FpeCipher::Ff1(c) => c.encrypt(plaintext),
            FpeCipher::Rekey(old, new) => new.encrypt(&old.decrypt(plaintext)?),
        }
    }

    pub fn decrypt(&self, ciphertext: &str) -> Result<String, FpeError> {
        match self {
            FpeCipher::Ff3(c) => c.decrypt(ciphertext),
            FpeCipher::Ff1(c) => c.decrypt(ciphertext),
            FpeCipher::Rekey(old, new) => old.encrypt(&new.decrypt(ciphertext)?),
        }
    }
}

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
    fn test_roundtrip_all_supported_lengths() {
        let cipher = test_cipher();
        for len in MIN_LEN..=MAX_LEN {
            let input: String = "1234567890".chars().cycle().take(len).collect();
            let ct = cipher.encrypt(&input).unwrap();
            assert_eq!(ct.len(), len);
            assert_eq!(cipher.decrypt(&ct).unwrap(), input);
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
    fn test_below_ff31_domain_minimum_rejected() {
        assert_eq!(
            test_cipher().encrypt("12345"),
            Err(FpeError::InvalidLength(5))
        );
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

    #[test]
    fn test_ff31_matches_independent_implementation() {
        let mut key = [0u8; KEY_LEN];
        for (i, b) in key.iter_mut().enumerate() {
            *b = i as u8;
        }
        let tweak = [0u8, 1, 2, 3, 4, 5, 6];
        let cipher = Ff3Cipher::new(&key, &tweak);
        let vectors = [
            ("890121234567890000", "828619517847699900"),
            ("4111111111111111", "1064960767628711"),
            ("123456789", "607331705"),
            ("14155552671", "34780551212"),
            ("12345678901234567890123456", "80998387094033638262634220"),
            ("000000", "706154"),
        ];
        for (pt, expected_ct) in vectors {
            assert_eq!(cipher.encrypt(pt).unwrap(), expected_ct);
            assert_eq!(cipher.decrypt(expected_ct).unwrap(), pt);
        }
    }

    #[test]
    fn test_ff31_matches_independent_implementation_second_key() {
        let key = [0xEFu8; KEY_LEN];
        let tweak = [0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54, 0x32];
        let cipher = Ff3Cipher::new(&key, &tweak);
        assert_eq!(cipher.encrypt("890121234567890000").unwrap(), "321377551284055879");
        assert_eq!(cipher.encrypt("0123456789").unwrap(), "8177437935");
    }
}
