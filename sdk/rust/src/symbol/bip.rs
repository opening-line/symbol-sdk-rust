use crate::symbol::key::*;
use crate::symbol::models::*;

pub use bip39::Mnemonic;
use hmac::Mac;
type HmacSha512 = hmac::Hmac<sha2::Sha512>;

#[derive(Clone)]
pub struct Bip32Node {
    pub private_key: PrivateKey,
    chain_code: [u8; PrivateKey::SIZE],
}

impl Bip32Node {
    pub const ROOT_HMAC_KEY: &'static [u8] = "ed25519 seed".as_bytes();
    pub fn new(hmac_key: &[u8], data: &[u8]) -> Result<Self, SymbolError> {
        let mut hmac = HmacSha512::new_from_slice(hmac_key).unwrap();
        hmac.update(data);

        let result = hmac.finalize().into_bytes();
        let (private_key, chain_code) = result.split_at(PrivateKey::SIZE);
        Ok(Bip32Node {
            private_key: PrivateKey::from_bytes(private_key.try_into()?),
            chain_code: chain_code.try_into()?,
        })
    }
    pub fn from_seed(seed: &[u8]) -> Result<Self, SymbolError> {
        Self::new(Self::ROOT_HMAC_KEY, seed)
    }
    pub fn derive_one(&self, identifier: u32) -> Result<Self, SymbolError> {
        let child_data_len = 1 + PrivateKey::SIZE + 4;
        let mut child_data = vec![0; child_data_len]; // child_data.len() == 37
        child_data[0] = 0;
        child_data[child_data_len - 4] = 128;
        let private_key = self.private_key.to_bytes();

        for i in 0..4 {
            child_data[child_data_len - 1 - i] |= (identifier >> (8 * i)) as u8;
            // & 0xFF
        }

        for i in 0..PrivateKey::SIZE {
            child_data[1 + i] = private_key[i];
        }

        Self::new(&self.chain_code, &child_data)
    }
    pub fn derive_path(&self, path: &[u32]) -> Result<Self, SymbolError> {
        let mut next_node = self.clone();
        for identifier in path {
            next_node = next_node.derive_one(*identifier)?;
        }
        Ok(next_node)
    }
    pub fn public_key(&self) -> PublicKey {
        self.private_key.pubilc_key()
    }
}
