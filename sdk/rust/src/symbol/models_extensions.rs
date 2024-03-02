pub use crate::symbol::models::*;

pub const MAINNET: NetworkType = NetworkType::MAINNET;
pub const TESTNET: NetworkType = NetworkType::TESTNET;

fn get_hash<Hasher: digest::Digest>(data: impl AsRef<[u8]>) -> Vec<u8> {
    Hasher::new().chain_update(data).finalize().to_vec()
}

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
        use ripemd::Ripemd160;
        use sha3::Sha3_256;

        let part_one_hash = get_hash::<Sha3_256>(&self);
        let part_two_hash = get_hash::<Ripemd160>(part_one_hash);

        let mut version = network_type.serialize();
        version.append(&mut part_two_hash.to_vec());

        let mut checksum = get_hash::<Sha3_256>(&version)[0..3].to_vec();

        let mut address = version;
        address.append(&mut checksum);

        Address::new(address.try_into().unwrap())
    }
    fn verify_transaction<T: TraitMessage + TraitSignature>(
        &self,
        transaction: &T,
    ) -> Result<(), SymbolError> {
        use ed25519_dalek::Verifier;
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
    fn shared_key(&self, other_public_key: PublicKey) -> SharedKey;
}

impl ExtentionSigningKey for PrivateKey {
    fn from_str(str: &str) -> Result<Self, SymbolError> {
        Ok(Self::from_bytes(hex::decode(str)?.as_slice().try_into()?))
    }
    fn sign_transaction<T: TraitMessage + TraitSignature>(&self, mut transaction: T) -> T {
        use ed25519_dalek::Signer;
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
    fn shared_key(&self, other_public_key: PublicKey) -> SharedKey {
        use curve25519_dalek::{edwards::CompressedEdwardsY, scalar::Scalar};
        use hkdf::Hkdf;
        use sha2::{Sha256, Sha512};

        let private_key = self;

        let unpacked_public_key = CompressedEdwardsY(other_public_key.to_bytes())
            .decompress()
            .unwrap();

        let mut scalar = get_hash::<Sha512>(private_key.as_bytes())[..32].to_vec();
        scalar[0] &= 248;
        scalar[31] &= 127;
        scalar[31] |= 64;

        #[allow(deprecated)]
        // This scalar is used only for multiplication with EdwardsPoint.
        let scalar = Scalar::from_bits(scalar.as_slice().try_into().unwrap());

        let shared_secret = (scalar * unpacked_public_key).compress();

        let hkdf = Hkdf::<Sha256>::new(None, &shared_secret.to_bytes());
        let mut shared_key = [0u8; 32];
        hkdf.expand(b"catapult", &mut shared_key).unwrap();

        SharedKey(shared_key)
    }
}

pub trait ExtentionAddress
where
    Self: Sized,
{
    fn to_string(&self) -> String;
    fn from_str(s: &str) -> Result<Address, String>;
}

impl ExtentionAddress for Address {
    fn to_string(&self) -> String {
        base32::encode(base32::Alphabet::RFC4648 { padding: false }, &self.0)
    }
    fn from_str(s: &str) -> Result<Address, String> {
        match base32::decode(base32::Alphabet::RFC4648 { padding: false }, s) {
            Some(bytes) => {
                if bytes.len() == 24 {
                    let mut arr = [0u8; 24];
                    arr.copy_from_slice(&bytes[0..24]);
                    Ok(Address(arr))
                } else {
                    Err(format!(
                        "Invalid input length: expected 24 bytes, got {}",
                        bytes.len()
                    ))
                }
            }
            None => Err("Base32 decoding failed".to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedKey(pub [u8; 32]);
