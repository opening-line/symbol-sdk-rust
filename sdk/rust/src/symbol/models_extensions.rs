pub use ed25519_dalek::{Signer, Verifier};
use hex;
use ripemd::{Digest, Ripemd160};
use sha3::Sha3_256;

pub use crate::symbol::models::*;

pub const MAINNET: NetworkType = NetworkType::MAINNET;
pub const TESTNET: NetworkType = NetworkType::TESTNET;

pub trait ExtentionVerifyingKey
where
    Self: Sized,
{
    fn from_str(str: &str) -> Result<Self, SymbolError>;
    fn address(&self, network: NetworkType) -> Address;
    fn verify_transaction<T: TraitMessage + TraitSignature>(
        &self,
        transaction: &T,
    ) -> Result<(), SymbolError>;
}

impl ExtentionVerifyingKey for PublicKey {
    fn from_str(str: &str) -> Result<Self, SymbolError> {
        Ok(Self::from_bytes(hex::decode(str)?.as_slice().try_into()?)?)
    }
    fn address(&self, network_type: NetworkType) -> Address {
        let mut part_one_hasher = Sha3_256::new();
        part_one_hasher.update(&self);
        let part_one_hash = part_one_hasher.finalize();

        let mut part_two_hasher = Ripemd160::new();
        part_two_hasher.update(part_one_hash);
        let part_two_hash = part_two_hasher.finalize();
        let mut version = network_type.serialize();
        version.append(&mut part_two_hash.to_vec());

        let mut part_three_hasher = Sha3_256::new();
        part_three_hasher.update(&version);
        let checksum = part_three_hasher.finalize()[0..4].to_vec();

        let mut address = version;
        address.append(&mut checksum[0..3].to_vec());

        Address::new(address.try_into().unwrap())
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
    fn verify_transaction<T: TraitMessage + TraitSignature>(
        &self,
        transaction: &T,
    ) -> Result<(), SymbolError>;
    fn pubilc_key(&self) -> PublicKey;
}

impl ExtentionSigningKey for PrivateKey {
    fn from_str(str: &str) -> Result<Self, SymbolError> {
        Ok(Self::from_bytes(hex::decode(str)?.as_slice().try_into()?))
    }
    fn sign_transaction<T: TraitMessage + TraitSignature>(&self, mut transaction: T) -> T {
        let signature = self.sign(transaction.get_message());
        transaction.set_signature(signature);
        transaction
    }
    fn verify_transaction<T: TraitMessage + TraitSignature>(
        &self,
        transaction: &T,
    ) -> Result<(), SymbolError> {
        self.verifying_key().verify_transaction(transaction)
    }
    fn pubilc_key(&self) -> PublicKey {
        self.verifying_key()
    }
}

pub trait ExtentionAddress
where
    Self: Sized,
{
    fn to_string(&self) -> String;
}

impl ExtentionAddress for Address {
    fn to_string(&self) -> String {
        base32::encode(base32::Alphabet::RFC4648 { padding: false }, &self.0)
    }
}
