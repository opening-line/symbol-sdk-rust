use crate::symbol::models::*;

pub trait ExtentionUnresolvedAddress
where
    Self: Sized,
{
    fn to_string(&self) -> String;
    fn from_str(s: &str) -> Result<UnresolvedAddress, SymbolError>;
    fn mosaic_id(&self, nonce: &MosaicNonce) -> MosaicId;
}

impl ExtentionUnresolvedAddress for UnresolvedAddress {
    fn to_string(&self) -> String {
        base32::encode(base32::Alphabet::RFC4648 { padding: false }, &self.0)
    }
    fn from_str(s: &str) -> Result<UnresolvedAddress, SymbolError> {
        match base32::decode(base32::Alphabet::RFC4648 { padding: false }, s) {
            Some(bytes) => {
                if bytes.len() == UnresolvedAddress::SIZE {
                    let mut arr = [0u8; UnresolvedAddress::SIZE];
                    arr.copy_from_slice(&bytes[0..UnresolvedAddress::SIZE]);
                    Ok(UnresolvedAddress(arr))
                } else {
                    Err(SymbolError::SizeError {
                        expect: vec![UnresolvedAddress::SIZE],
                        real: bytes.len(),
                    })
                }
            }
            None => Err(SymbolError::Base32DecodeError(
                "Base32 decoding failed".to_string(),
            )),
        }
    }
    fn mosaic_id(&self, nonce: &MosaicNonce) -> MosaicId {
        use sha3::Sha3_256;

        let mut data = nonce.0.to_le_bytes().to_vec();
        data.extend_from_slice(&self.0);

        let hash = get_hash::<Sha3_256>(data).to_vec();
        let hash = u64::from_le_bytes(hash[0..MosaicId::SIZE].try_into().unwrap());
        let mosaic_id = hash & 0x7FFF_FFFF_FFFF_FFFF;
        MosaicId(mosaic_id)
    }
}
