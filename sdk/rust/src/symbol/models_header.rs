// use data_encoding::DecodeError;
use ed25519_dalek::ed25519;
use hex::FromHexError;
use std::array::TryFromSliceError;
use super::models::TransactionType;

#[derive(Debug)]
pub enum SymbolError {
    FromHexError(FromHexError),
    Base32DecodeError(String),
    TryFromSliceError(TryFromSliceError),
    SizeError { expect: Vec<usize>, real: usize },
    InvalidLengthError(crypto_common::InvalidLength),
    ReservedIsNotZeroError(u32),
    MismatchError { pattern: Vec<u32>, place: String },
    Ed25519Error(ed25519::Error),
    AesGcmError(aes_gcm::Error),
    OverflowError(String),
    IoError(std::io::Error),
}

impl From<FromHexError> for SymbolError {
    fn from(err: FromHexError) -> Self {
        SymbolError::FromHexError(err)
    }
}
impl From<TryFromSliceError> for SymbolError {
    fn from(err: TryFromSliceError) -> SymbolError {
        SymbolError::TryFromSliceError(err)
    }
}
impl From<ed25519::Error> for SymbolError {
    fn from(err: ed25519::Error) -> SymbolError {
        SymbolError::Ed25519Error(err)
    }
}
impl From<aes_gcm::Error> for SymbolError {
    fn from(err: aes_gcm::Error) -> SymbolError {
        SymbolError::AesGcmError(err)
    }
}
impl From<std::io::Error> for SymbolError {
    fn from(err: std::io::Error) -> SymbolError {
        SymbolError::IoError(err)
    }
}
impl From<crypto_common::InvalidLength> for SymbolError {
    fn from(err: crypto_common::InvalidLength) -> SymbolError {
        SymbolError::InvalidLengthError(err)
    }
}

pub(crate) use crate::symbol::key::*;

pub trait ModelsSignature
where
    Self: Sized,
{
    const SIZE: usize;
    fn new(signature: [u8; 64]) -> Self;
    fn default() -> Self;
    fn size(&self) -> usize;
    fn deserialize(payload: &[u8]) -> Result<(Self, &[u8]), SymbolError>;
    fn serialize(&self) -> Vec<u8>;
}

impl ModelsSignature for Signature {
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
                expect: vec![Self::SIZE],
                real: payload.len(),
            });
        }
        let (bytes, rest) = payload.split_at(Self::SIZE);
        Ok((Self::from_bytes(bytes.try_into()?), rest))
    }
    fn serialize(&self) -> Vec<u8> {
        self.to_vec()
    }
}

pub trait ModelsPublicKey
where
    Self: Sized,
{
    const SIZE: usize;
    fn new(signature: [u8; 32]) -> Self;
    fn size(&self) -> usize;
    fn deserialize(payload: &[u8]) -> Result<(Self, &[u8]), SymbolError>;
    fn serialize(&self) -> Vec<u8>;
}
impl ModelsPublicKey for PublicKey {
    const SIZE: usize = 32;
    fn new(verifyingkey: [u8; 32]) -> PublicKey {
        Self::from_bytes(&verifyingkey).unwrap()
    }
    fn size(&self) -> usize {
        Self::SIZE
    }
    fn deserialize(payload: &[u8]) -> Result<(Self, &[u8]), SymbolError> {
        if payload.len() < Self::SIZE {
            return Err(SymbolError::SizeError {
                expect: vec![Self::SIZE],
                real: payload.len(),
            });
        }
        let (bytes, rest) = payload.split_at(Self::SIZE);
        Ok((Self::from_bytes(bytes.try_into()?)?, rest))
    }
    fn serialize(&self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}

#[cfg(feature = "private_network")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkType(pub u8);

#[cfg(feature = "private_network")]
impl NetworkType {
    pub const SIZE: usize = 1;
    pub fn default() -> Self {
        Self(u8::default())
    }
    pub fn size(&self) -> usize {
        Self::SIZE
    }
    pub fn deserialize(payload: &[u8]) -> Result<(Self, &[u8]), SymbolError> {
        if payload.len() < Self::SIZE {
            return Err(SymbolError::SizeError {
                expect: vec![Self::SIZE],
                real: payload.len(),
            });
        }
        let (bytes, rest) = payload.split_at(Self::SIZE);
        Ok((NetworkType(u8::from_le_bytes(bytes.try_into()?)), rest))
    }
    pub fn serialize(&self) -> Vec<u8> {
        (self.0 as u8).to_le_bytes().to_vec()
    }
}

pub(crate) fn get_hash<Hasher: digest::Digest>(data: impl AsRef<[u8]>) -> Vec<u8> {
    Hasher::new().chain_update(data).finalize().to_vec()
}

pub(crate) fn is_aggregate_transaction(payload:impl AsRef<[u8]>)->Result<bool,SymbolError>{
	let transaction_type=payload
		.as_ref()
		.iter()
		.skip(110)
		.take(2)
		.map(|x|*x)
		.collect::<Vec<u8>>();
	let transaction_type:[u8;2]=<Vec<u8>as AsRef<[u8]>>::as_ref(&transaction_type)
		.as_ref()
		.try_into()
		.map_err(|_|SymbolError::SizeError{expect:vec![2],real:transaction_type.len(),})?;
	let transaction_type=u16::from_le_bytes(transaction_type);
	let return_value=transaction_type==TransactionType::AGGREGATE_COMPLETE as u16;
	let return_value=return_value||transaction_type==TransactionType::AGGREGATE_BONDED as u16;
	Ok(return_value)
}

#[cfg(test)]
#[test]
fn is_aggregate_transaction_returns_true_on_aggregate_complete_transaction_v2(){
	use super::models::*;
	let payload=AggregateBondedTransactionV2::default().serialize();
	let actual=is_aggregate_transaction(payload).unwrap();
	assert_eq!(true,actual);
}

#[cfg(test)]
#[test]
fn is_aggregate_transaction_returns_false_on_vrf_key_link_transaction_v1(){
	use super::models::*;
	let payload=VrfKeyLinkTransactionV1::default().serialize();
	let actual=is_aggregate_transaction(payload).unwrap();
	assert_eq!(false,actual);
}

#[cfg(test)]
#[test]
fn is_aggregate_transaction_raises_error_on_empty_input(){
	let err=is_aggregate_transaction(&[])
		.unwrap_err();
}
