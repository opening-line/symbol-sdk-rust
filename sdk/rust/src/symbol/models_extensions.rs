pub use crate::symbol::models::*;

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
