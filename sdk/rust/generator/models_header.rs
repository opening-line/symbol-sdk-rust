use data_encoding::DecodeError;
use hex;
use hex::FromHexError;
use std::array::TryFromSliceError;

#[derive(Debug)]
pub enum SymbolError {
    FromHexError(FromHexError),
    Base32DecodeError(DecodeError),
    TryFromSliceError(TryFromSliceError),
    SizeError { expect: usize, real: usize },
    ReservedIsNotZeroError(u32),
    EnumDecodeError(u32)
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