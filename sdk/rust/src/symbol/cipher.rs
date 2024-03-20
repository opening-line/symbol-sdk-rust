use crate::symbol::key::SharedKey;
use crate::symbol::models::*;

use aes_gcm::{aead::generic_array::GenericArray, aead::Aead, Aes256Gcm};
use cipher::KeyInit;
use hex::decode;

pub struct Cipher(aes_gcm::Aes256Gcm);

impl Cipher {
    pub fn new(shared_key: SharedKey) -> Self {
        let key = GenericArray::from_slice(shared_key.as_bytes());
        Self(Aes256Gcm::new(key))
    }
    pub fn encrypt(&self, clear_text: &[u8], nonce: &[u8]) -> Result<Vec<u8>, SymbolError> {
        let nonce = GenericArray::from_slice(nonce);
        Ok(self.0.encrypt(nonce, clear_text)?)
    }
    pub fn decrypt(&self, cipher_text: &[u8], nonce: &[u8]) -> Result<Vec<u8>, SymbolError> {
        let nonce = GenericArray::from_slice(nonce);
        Ok(self.0.decrypt(nonce, cipher_text)?)
    }
}

impl MosaicId {
    pub fn from_str(str: &str) -> Result<Self, SymbolError> {
        let bytes = decode(str)?;
        if bytes.len() != MosaicId::SIZE {
            return Err(SymbolError::SizeError {
                expect: 8,
                real: bytes.len(),
            });
        }
        let mosaic_id = Self(u64::from_be_bytes(bytes.try_into().unwrap()));
        Ok(mosaic_id)
    }
}
