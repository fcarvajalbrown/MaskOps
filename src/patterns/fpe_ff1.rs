use aes::Aes256;
use aes::cipher::{BlockEncrypt, KeyInit, generic_array::GenericArray};

use crate::patterns::fpe::{parse_digits, validate_len, digits_to_string, FpeError, KEY_LEN};

const RADIX: u128 = 10;

pub struct Ff1Cipher {
    aes: Aes256,
    tweak: Vec<u8>,
}

impl Ff1Cipher {
    pub fn new(key: &[u8; KEY_LEN], tweak: &[u8]) -> Self {
        let aes = Aes256::new(GenericArray::from_slice(key));
        Self { aes, tweak: tweak.to_vec() }
    }

    pub fn encrypt(&self, plaintext: &str) -> Result<String, FpeError> {
        let nums = parse_digits(plaintext)?;
        let result = self.feistel(&nums, true)?;
        Ok(digits_to_string(&result))
    }

    pub fn decrypt(&self, ciphertext: &str) -> Result<String, FpeError> {
        let nums = parse_digits(ciphertext)?;
        let result = self.feistel(&nums, false)?;
        Ok(digits_to_string(&result))
    }

    fn aes_block(&self, block: &[u8; 16]) -> [u8; 16] {
        let mut b = GenericArray::clone_from_slice(block);
        self.aes.encrypt_block(&mut b);
        b.into()
    }

    fn prf(&self, data: &[u8]) -> [u8; 16] {
        let mut y = [0u8; 16];
        for chunk in data.chunks(16) {
            let mut blk = [0u8; 16];
            for i in 0..16 {
                blk[i] = y[i] ^ chunk[i];
            }
            y = self.aes_block(&blk);
        }
        y
    }

    fn expand_s(&self, r: &[u8; 16], d: usize) -> Vec<u8> {
        let mut s = r.to_vec();
        let mut j: u128 = 1;
        while s.len() < d {
            let jb = j.to_be_bytes();
            let mut block = *r;
            for k in 0..16 {
                block[k] ^= jb[k];
            }
            s.extend_from_slice(&self.aes_block(&block));
            j += 1;
        }
        s.truncate(d);
        s
    }

    fn feistel(&self, x: &[u8], forward: bool) -> Result<Vec<u8>, FpeError> {
        let n = x.len();
        validate_len(n)?;

        let u = n / 2;
        let v = n - u;

        let bits = pow10(v).ilog2() as usize + 1;
        let b_bytes = (bits + 7) / 8;
        let d = 4 * ((b_bytes + 3) / 4) + 4;

        let t = self.tweak.len();
        let p = build_p(u, n, t);

        let mut a = x[..u].to_vec();
        let mut b = x[u..].to_vec();

        let rounds: Vec<u8> = if forward {
            (0u8..10).collect()
        } else {
            (0u8..10).rev().collect()
        };

        for round in rounds {
            let m = if round % 2 == 0 { u } else { v };
            let source = if forward { &b } else { &a };

            let q = self.build_q(round, source, b_bytes);
            let mut pq = Vec::with_capacity(p.len() + q.len());
            pq.extend_from_slice(&p);
            pq.extend_from_slice(&q);

            let r = self.prf(&pq);
            let s = self.expand_s(&r, d);
            let y = num_from_bytes(&s);

            let modulus = pow10(m);
            let target = if forward { &a } else { &b };
            let target_num = num_radix(target) % modulus;

            let c = if forward {
                (target_num + y % modulus) % modulus
            } else {
                (target_num + modulus - y % modulus) % modulus
            };
            let c_digits = str_radix(c, m);

            if forward {
                a = b;
                b = c_digits;
            } else {
                b = a;
                a = c_digits;
            }
        }

        let mut result = a;
        result.extend_from_slice(&b);
        Ok(result)
    }

    fn build_q(&self, round: u8, half: &[u8], b_bytes: usize) -> Vec<u8> {
        let t = self.tweak.len();
        let pad = (-(t as i64) - (b_bytes as i64) - 1).rem_euclid(16) as usize;
        let mut q = Vec::with_capacity(t + pad + 1 + b_bytes);
        q.extend_from_slice(&self.tweak);
        q.extend(std::iter::repeat(0u8).take(pad));
        q.push(round);
        let num = num_radix(half);
        let num_bytes = num.to_be_bytes();
        q.extend_from_slice(&num_bytes[16 - b_bytes..]);
        q
    }
}

fn build_p(u: usize, n: usize, t: usize) -> [u8; 16] {
    let mut p = [0u8; 16];
    p[0] = 1;
    p[1] = 2;
    p[2] = 1;
    p[3] = 0;
    p[4] = 0;
    p[5] = RADIX as u8;
    p[6] = 10;
    p[7] = (u % 256) as u8;
    p[8..12].copy_from_slice(&(n as u32).to_be_bytes());
    p[12..16].copy_from_slice(&(t as u32).to_be_bytes());
    p
}

fn pow10(m: usize) -> u128 {
    RADIX.pow(m as u32)
}

fn num_radix(digits: &[u8]) -> u128 {
    digits.iter().fold(0u128, |acc, &d| acc * RADIX + d as u128)
}

fn str_radix(mut x: u128, m: usize) -> Vec<u8> {
    let mut out = vec![0u8; m];
    for i in (0..m).rev() {
        out[i] = (x % RADIX) as u8;
        x /= RADIX;
    }
    out
}

fn num_from_bytes(b: &[u8]) -> u128 {
    b.iter().fold(0u128, |acc, &x| (acc << 8) | x as u128)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_cipher() -> Ff1Cipher {
        Ff1Cipher::new(&[0u8; KEY_LEN], &[0u8; 7])
    }

    #[test]
    fn test_roundtrip_card() {
        let cipher = test_cipher();
        let pt = "4111111111111111";
        let ct = cipher.encrypt(pt).unwrap();
        assert_eq!(ct.len(), pt.len());
        assert_ne!(ct, pt);
        assert_eq!(cipher.decrypt(&ct).unwrap(), pt);
    }

    #[test]
    fn test_roundtrip_all_lengths() {
        let cipher = test_cipher();
        for len in [6, 9, 11, 13, 16, 20, 30] {
            let input: String = "1234567890".chars().cycle().take(len).collect();
            let ct = cipher.encrypt(&input).unwrap();
            assert_eq!(ct.len(), len);
            assert_eq!(cipher.decrypt(&ct).unwrap(), input);
        }
    }

    #[test]
    fn test_output_is_all_digits() {
        let ct = test_cipher().encrypt("9876543210123456").unwrap();
        assert!(ct.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_different_tweaks_differ() {
        let key = [0u8; KEY_LEN];
        let c1 = Ff1Cipher::new(&key, &[1u8; 7]);
        let c2 = Ff1Cipher::new(&key, &[2u8; 7]);
        let pt = "1234567890123456";
        assert_ne!(c1.encrypt(pt).unwrap(), c2.encrypt(pt).unwrap());
    }

    #[test]
    fn test_ff1_differs_from_ff3() {
        use crate::patterns::fpe::Ff3Cipher;
        let key = [7u8; KEY_LEN];
        let tweak = [3u8; 7];
        let ff1 = Ff1Cipher::new(&key, &tweak);
        let ff3 = Ff3Cipher::new(&key, &tweak);
        let pt = "4111111111111111";
        assert_ne!(ff1.encrypt(pt).unwrap(), ff3.encrypt(pt).unwrap());
    }

    #[test]
    fn test_invalid_character_rejected() {
        assert!(test_cipher().encrypt("12X4567").is_err());
    }

    #[test]
    fn test_ff1_matches_nist_sp80038g_samples() {
        let key: [u8; KEY_LEN] = [
            0x2B, 0x7E, 0x15, 0x16, 0x28, 0xAE, 0xD2, 0xA6,
            0xAB, 0xF7, 0x15, 0x88, 0x09, 0xCF, 0x4F, 0x3C,
            0xEF, 0x43, 0x59, 0xD8, 0xD5, 0x80, 0xAA, 0x4F,
            0x7F, 0x03, 0x6D, 0x6F, 0x04, 0xFC, 0x6A, 0x94,
        ];
        let sample7 = Ff1Cipher::new(&key, &[]);
        assert_eq!(sample7.encrypt("0123456789").unwrap(), "6657667009");
        assert_eq!(sample7.decrypt("6657667009").unwrap(), "0123456789");

        let tweak8: [u8; 10] = [0x39, 0x38, 0x37, 0x36, 0x35, 0x34, 0x33, 0x32, 0x31, 0x30];
        let sample8 = Ff1Cipher::new(&key, &tweak8);
        assert_eq!(sample8.encrypt("0123456789").unwrap(), "1001623463");
        assert_eq!(sample8.decrypt("1001623463").unwrap(), "0123456789");
    }
}
