const NAMESPACE_FLAG: u64 = 1 << 63;

use crate::symbol::models::*;
use binascii::b32decode;
use sha3::{Digest, Sha3_256};

pub struct Address {
    pub value: Vec<u8>,
}

impl Address {
    const SIZE: usize = 24;
    fn new(address: Vec<u8>) -> Self {
        Self { value: address }
    }
    pub fn from_str(address: &str) -> Self {
        let mut output_buffer = [0u8; 200];
        Self {
            value: b32decode(&address.as_bytes(), &mut output_buffer)
                .unwrap()
                .to_vec(),
        }
    }
}

impl MosaicId {
    pub fn from_address_nonce(owner_address: Address, nonce: u32) -> Self {
        let mut hasher = Sha3_256::new();
        hasher.update(nonce.to_le_bytes());
        hasher.update(owner_address.value);
        let digest = hasher.finalize();

        let bytes: [u8; 8] = digest[0..8].try_into().expect("Wrong length");
        let result = u64::from_le_bytes(bytes);
        assert_eq!(result & NAMESPACE_FLAG, 0);
        // let result = u64::from_le_bytes(bytes) & (!NAMESPACE_FLAG);

        Self::new(result)
    }
}

impl NamespaceId {
    pub fn from_name_parent(name: &str, parent_namespace_id: NamespaceId) -> Self {
        let mut hasher = Sha3_256::new();
        hasher.update(parent_namespace_id.0.to_le_bytes());
        hasher.update(name);
        let digest = hasher.finalize();
        let bytes: [u8; 8] = digest[0..8].try_into().expect("Wrong length");
        let result = u64::from_le_bytes(bytes);
        assert_eq!(result & NAMESPACE_FLAG, NAMESPACE_FLAG);

        Self::new(result)
    }
    pub fn from_name(name: &str) -> Self {
        Self::from_name_parent(name, NamespaceId::default())
    }
}

pub struct NamespaceName {
    value: String,
}

impl NamespaceName {
    fn new(name: &str) -> Self {
        Self {
            value: name.to_string(),
        }
    }
    pub fn from_str(name: &str) -> Self {
        use regex::Regex;
        assert!(Regex::new(r"^[0-9a-z][0-9a-z\-_]*$")
            .unwrap()
            .is_match(name));
        Self::new(name)
    }
}

#[test]
fn test_generate_mosaic_id() {
    let address = Address::from_str("TATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA37JGO5Q");
    let nonce = 812613930;
    let mosaic_id = MosaicId::from_address_nonce(address, nonce);
    assert_eq!(mosaic_id.0, 0x570FB3ED9379624C);
}
