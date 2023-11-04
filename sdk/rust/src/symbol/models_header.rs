use data_encoding::DecodeError;
use hex::FromHexError;
use std::array::TryFromSliceError;

#[derive(Debug)]
pub enum SymbolError {
    FromHexError(FromHexError),
    Base32DecodeError(DecodeError),
    TryFromSliceError(TryFromSliceError),
    SizeError { expect: usize, real: usize },
    ReservedIsNotZeroError(u32),
    EnumDecodeError(u32),
}

impl From<FromHexError> for SymbolError {
    fn from(err: FromHexError) -> Self {
        SymbolError::FromHexError(err)
    }
}

impl From<DecodeError> for SymbolError {
    fn from(err: DecodeError) -> Self {
        SymbolError::Base32DecodeError(err)
    }
}

impl From<TryFromSliceError> for SymbolError {
    fn from(err: TryFromSliceError) -> SymbolError {
        SymbolError::TryFromSliceError(err)
    }
}

pub use ed25519_dalek::Signature;
pub use ed25519_dalek::SigningKey;

pub trait ExtentionSignature
where
    Self: Sized,
{
    const SIZE: usize;
    fn new(signature: [u8; 64]) -> Self;
    fn default() -> Self;
    fn size(&self) -> usize;
    fn deserialize(payload: &[u8]) -> Result<(Self, &[u8]), SymbolError>;
    fn serialize(&self) -> Vec<u8>;
    fn to_string(&self) -> String;
}

impl ExtentionSignature for Signature {
    const SIZE: usize = 64;
    fn new(signature: [u8; 64]) -> Self {
        Self::from_bytes(&signature)
    }
    fn default() -> Self {
        Self::from_bytes(&[0; Self::SIZE])
    }
    fn size(&self) -> usize {
        Self::SIZE
    }
    fn deserialize(payload: &[u8]) -> Result<(Self, &[u8]), SymbolError> {
        if payload.len() < Self::SIZE {
            return Err(SymbolError::SizeError {
                expect: Self::SIZE,
                real: payload.len(),
            });
        }
        let (bytes, rest) = payload.split_at(Self::SIZE);
        Ok((Self::from_bytes(bytes.try_into()?), rest))
    }
    fn serialize(&self) -> Vec<u8> {
        self.to_vec()
    }
    fn to_string(&self) -> String {
        format!("0x{}", self)
    }
}

pub trait ExtentionPublicKey
where
    Self: Sized,
{
    const SIZE: usize;
    fn new(signature: [u8; 32]) -> Self;
    fn size(&self) -> usize;
    fn deserialize(payload: &[u8]) -> Result<(Self, &[u8]), SymbolError>;
    fn serialize(&self) -> Vec<u8>;
    fn to_string(&self) -> String;
}
pub type PublicKey = ed25519_dalek::VerifyingKey;
impl ExtentionPublicKey for PublicKey {
    const SIZE: usize = 32;
    fn new(publickey: [u8; 32]) -> PublicKey {
        Self::from_bytes(&publickey).unwrap()
    }
    fn size(&self) -> usize {
        Self::SIZE
    }
    fn deserialize(payload: &[u8]) -> Result<(Self, &[u8]), SymbolError> {
        if payload.len() < Self::SIZE {
            return Err(SymbolError::SizeError {
                expect: Self::SIZE,
                real: payload.len(),
            });
        }
        let (bytes, rest) = payload.split_at(Self::SIZE);
        Ok((Self::from_bytes(bytes.try_into()?).unwrap(), rest))
    }
    fn serialize(&self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
    fn to_string(&self) -> String {
        format!("0x{}", hex::encode(self))
    }
}
