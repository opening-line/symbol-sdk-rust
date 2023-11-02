use ed25519_dalek::Signer;
use symbol::symbol::models::*;

fn main() {
    let a = Address::default();
    println!("{}", a.to_string());
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Address(pub [u8; 24]);
impl Address {
    const SIZE: usize = 24;
    pub fn new(address: [u8; 24]) -> Self {
        Self(address)
    }
    pub fn default() -> Self {
        Self([0; 24])
    }
    pub fn from_str(hex_str: &str) -> Option<Self> {
        let mut bytes = [0; 24];
        if let Err(_) = hex::decode_to_slice(hex_str, &mut bytes) {
            None
        } else {
            Some(Self::new(bytes))
        }
    }
    pub fn size(&self) -> usize {
        Self::SIZE
    }
    pub fn deserialize(payload: &[u8]) -> Option<(Self, &[u8])> {
        if payload.len() < 24 {
            return None;
        }
        let mut bytes = [0u8; 24];
        bytes.copy_from_slice(payload);
        Some((Self::new(bytes), &payload[..24]))
    }
    pub fn serialize(&self) -> Vec<u8> {
        self.0.to_vec()
    }
    pub fn to_string(&self) -> String {
        format!("0x{}", hex::encode(self.0))
    }
}
