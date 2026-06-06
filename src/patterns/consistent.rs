

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
