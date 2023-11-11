use ed25519_dalek::Signer;

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

pub trait ExtentionPublicKey
where
    Self: Sized,
{
    fn from_str(str: &str) -> Result<Self, SymbolError>;
}

impl ExtentionPublicKey for PublicKey {
    fn from_str(str: &str) -> Result<Self, SymbolError> {
        Ok(Self::from_bytes(hex::decode(str)?.as_slice().try_into()?)?)
    }
}

pub trait ExtentionSigningKey<T>
where
    Self: Sized,
    T: TraitMessage + TraitSignature,
{
    fn sign_transaction(&self, transaction: T) -> T;
    fn verify_transaction(&self, transaction: &T) -> Result<(), SymbolError>;
}

impl<T> ExtentionSigningKey<T> for SigningKey
where
    T: TraitMessage + TraitSignature,
{
    fn sign_transaction(&self, mut transaction: T) -> T {
        let signature = self.sign(transaction.get_message());
        transaction.set_signature(signature);
        transaction
    }
    fn verify_transaction(&self, transaction: &T) -> Result<(), SymbolError> {
        Ok(self.verify(transaction.get_message(), transaction.get_signature())?)
    }
}

// fn tmp() {

//     let init32 = [0; 32];
//     let init64 = [0; 64];
//     let signing_key = SigningKey::from_bytes(&init32);
//     let verifying_key = signing_key.verifying_key();
//     let signature = Signature::from_bytes(&init64);
//     let a = signature.to_bytes();

//     let a = verifying_key.verify("msg".as_bytes(), &signature).is_ok();

//     impl SignTrait for SigningKey {

//     }
// }
