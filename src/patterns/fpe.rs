

use aes::Aes256;
use aes::cipher::{BlockEncrypt, KeyInit, generic_array::GenericArray};

pub(crate) const RADIX: u64 = 10;

pub const TWEAK_LEN: usize = 7;

pub const KEY_LEN: usize = 32;

pub(crate) const MIN_LEN: usize = 2;

pub(crate) const MAX_LEN: usize = 30;

pub struct Ff3Cipher {
    
    aes_enc: Aes256,
    
    aes_dec: Aes256,
    
    tweak: [u8; TWEAK_LEN],
}

impl Ff3Cipher {
    
    
    
    
    
    pub fn new(key: &[u8; KEY_LEN], tweak: &[u8; TWEAK_LEN]) -> Self {
        
        let mut rev_key = *key;
        rev_key.reverse();

        let aes_enc = Aes256::new(GenericArray::from_slice(key));
        let aes_dec = Aes256::new(GenericArray::from_slice(&rev_key));

        Self { aes_enc, aes_dec, tweak: *tweak }
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

            
            let p = self.build_p(i, &b);

            
            let s = self.prf(aes, &p);
            let y = bytes_to_u128_be(&s);

            
            let mut a_rev = a.clone();
            a_rev.reverse();
            let num_a   = num_from_digits(&a_rev);
            let modulus = radix_pow(m);

            let c = if forward {
                (num_a + y % modulus) % modulus
            } else {
                (num_a + modulus - (y % modulus)) % modulus
            };

            
            let mut c_digits = digits_of(c, m);
            c_digits.reverse();
            a = b;
            b = c_digits;
        }

        
        let result = if forward {
            let mut r = a; r.extend_from_slice(&b); r
        } else {
            let mut r = b; r.extend_from_slice(&a); r
        };

        Ok(result)
    }

    
    
    
    
    
    fn build_p(&self, round: u8, b: &[u8]) -> [u8; 16] {
        let mut p = [0u8; 16];
        let t = &self.tweak;

        
        
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

        
        let mut b_rev = b.to_vec();
        b_rev.reverse();
        let num_b = num_from_digits(&b_rev);
        let num_b_bytes = num_b.to_be_bytes();
        p[8..16].copy_from_slice(&num_b_bytes[8..]);

        p
    }

    
    fn prf(&self, aes: &Aes256, block: &[u8; 16]) -> [u8; 16] {
        let mut out = GenericArray::clone_from_slice(block);
        aes.encrypt_block(&mut out);
        out.into()
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

fn num_from_digits(digits: &[u8]) -> u128 {
    digits.iter().fold(0u128, |acc, &d| acc * RADIX as u128 + d as u128)
}

fn digits_of(mut n: u128, len: usize) -> Vec<u8> {
    let mut out = vec![0u8; len];
    for i in (0..len).rev() {
        out[i] = (n % RADIX as u128) as u8;
        n /= RADIX as u128;
    }
    out
}

fn bytes_to_u128_be(b: &[u8; 16]) -> u128 {
    u128::from_be_bytes(*b)
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