use crate::symbol::models::*;
pub use ed25519_dalek::Signer;
pub use ed25519_dalek::Signature;
use ed25519_dalek::SigningKey;
use ed25519_dalek::VerifyingKey;
use core::ops::{Deref, DerefMut};
use core::fmt;
use core::str::FromStr;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PublicKey(VerifyingKey);
impl PublicKey {
	pub const SIZE:usize=32;
	pub fn from_bytes(bytes:&[u8;32])->Result<Self,SymbolError>{
		Ok(Self(VerifyingKey::from_bytes(bytes)?))
	}
    pub fn address(&self, network_type: NetworkType) -> UnresolvedAddress {
        use ripemd::Ripemd160;
        use sha3::Sha3_256;

        let part_one_hash = get_hash::<Sha3_256>(&self.0.to_bytes());
        let part_two_hash = get_hash::<Ripemd160>(part_one_hash);

        let mut version = network_type.serialize();
        version.extend_from_slice(&part_two_hash);

        let mut checksum = get_hash::<Sha3_256>(&version)[0..3].to_vec();

        let mut address = version;
        address.append(&mut checksum);

        UnresolvedAddress::new(address.try_into().unwrap())
    }
	#[allow(unused_variables)]
    pub fn verify_transaction<T: TraitSignature>(
        &self,
        transaction: &T,
    ) -> Result<(), SymbolError> {
		todo!();
    }
}
impl Deref for PublicKey {
    type Target = VerifyingKey;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for PublicKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<PublicKey> for VerifyingKey {
    fn from(public_key: PublicKey) -> Self {
        public_key.0
    }
}
impl From<VerifyingKey> for PublicKey {
    fn from(verifying_key: VerifyingKey) -> Self {
        Self(verifying_key)
    }
}
impl FromStr for PublicKey {
    type Err = SymbolError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_bytes(hex::decode(s)?.as_slice().try_into()?)?)
    }
}
impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode(self.to_bytes()).to_uppercase())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivateKey(SigningKey);
impl PrivateKey {
	pub const SIZE:usize=32;
	pub fn from_bytes(bytes:&[u8;32])->Self{
		Self(SigningKey::from_bytes(bytes))
	}
	#[allow(unused_mut)]
	#[allow(unused_variables)]
    pub fn sign_transaction<T: TraitSignature>(&self, mut transaction: T) -> T {
		todo!();
    }
	#[allow(unused_variables)]
    pub fn verify_transaction<T: TraitSignature>(
        &self,
        transaction: &T,
    ) -> Result<(), SymbolError> {
		todo!();
    }
    pub fn public_key(&self) -> PublicKey {
        self.0.verifying_key().into()
    }
	pub fn shared_key(&self, other_public_key: PublicKey) -> SharedKey {
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
impl Deref for PrivateKey {
    type Target = SigningKey;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for PrivateKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<PrivateKey> for SigningKey {
    fn from(private_key: PrivateKey) -> Self {
        private_key.0
    }
}
impl From<SigningKey> for PrivateKey {
    fn from(signing_key: SigningKey) -> Self {
        Self(signing_key)
    }
}
impl FromStr for PrivateKey {
    type Err = SymbolError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_bytes(hex::decode(s)?.as_slice().try_into()?))
    }
}
impl fmt::Display for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode(self.to_bytes()).to_uppercase())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedKey(pub [u8; 32]);

impl SharedKey {
    #[inline]
    pub fn to_bytes(&self) -> [u8; 32] {
        self.0
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}
