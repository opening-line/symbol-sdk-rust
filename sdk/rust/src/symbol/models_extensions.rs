pub use ed25519_dalek::{Signer, Verifier};

pub use crate::symbol::models::*;
fn base32_decode(input: &str) -> Result<Vec<u8>, data_encoding::DecodeError> {
    use data_encoding::BASE32;
    let input = input.to_string() + &"=".repeat(8 - (input.len() % 8));
    BASE32.decode(&input.as_bytes())
}

impl UnresolvedAddress {
    pub fn from_str(str: &str) -> Result<Self, SymbolError> {
        Ok(Self::new(base32_decode(str)?.as_slice().try_into()?))
    }
}

pub trait ExtentionVerifyingKey
where
    Self: Sized,
{
    fn from_str(str: &str) -> Result<Self, SymbolError>;
    fn verify_transaction<T: TraitMessage + TraitSignature>(
        &self,
        transaction: &T,
    ) -> Result<(), SymbolError>;
}

impl ExtentionVerifyingKey for PublicKey {
    fn from_str(str: &str) -> Result<Self, SymbolError> {
        Ok(Self::from_bytes(hex::decode(str)?.as_slice().try_into()?)?)
    }
    fn verify_transaction<T: TraitMessage + TraitSignature>(
        &self,
        transaction: &T,
    ) -> Result<(), SymbolError> {
        Ok(self.verify(transaction.get_message(), transaction.get_signature())?)
    }
}

pub trait ExtentionSigningKey
where
    Self: Sized,
{
    fn from_str(str: &str) -> Result<Self, SymbolError>;
    fn sign_transaction<T: TraitMessage + TraitSignature>(&self, transaction: T) -> T;
    fn verify_transaction<T: TraitMessage + TraitSignature>(&self, transaction: &T) -> Result<(), SymbolError>;
}

impl ExtentionSigningKey for PrivateKey
{
    fn from_str(str: &str) -> Result<Self, SymbolError> {
        Ok(Self::from_bytes(hex::decode(str)?.as_slice().try_into()?))
    }
    fn sign_transaction<T: TraitMessage + TraitSignature>(&self, mut transaction: T) -> T 
    {
        let signature = self.sign(transaction.get_message());
        transaction.set_signature(signature);
        transaction
    }
    fn verify_transaction<T: TraitMessage + TraitSignature>(&self, transaction: &T) -> Result<(), SymbolError> {
        self.verifying_key().verify_transaction(transaction)
    }
}
