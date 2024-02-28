use hex::{decode, encode};
use serde::Deserialize;
use std::fs::read_to_string;

use symbol::symbol::models_extensions::*;

const TEST_VECTERS_PATH: &str = "../../tests/vectors/symbol";

#[cfg(test)]
fn get_hash<Hasher: digest::Digest> (data: impl AsRef<[u8]>) -> Vec<u8> {
    Hasher::new().chain_update(data).finalize().to_vec()
}

#[test]
fn test0_keccak_256() {
    #[derive(Deserialize)]
    struct Test {
        hash: String,
        length: usize,
        data: String,
    }

    let tests_path = TEST_VECTERS_PATH.to_string() + "/crypto/0.test-keccak-256.json";
    let tests_json_str = read_to_string(tests_path).unwrap();
    let tests: Vec<Test> = serde_json::from_str(&tests_json_str).unwrap();

    for test in tests {
        let data = decode(test.data).expect("Decoding failed");
        assert_eq!(data.len(), test.length);

        let hash = get_hash::<sha3::Keccak256>(&data);
        let hash_hex = encode(hash);
        assert_eq!(hash_hex, test.hash.to_lowercase());
    }
}

#[test]
fn test0_sha3_256() {
    #[derive(Deserialize)]
    struct Test {
        hash: String,
        length: usize,
        data: String,
    }

    let tests_path = TEST_VECTERS_PATH.to_string() + "/crypto/0.test-sha3-256.json";
    let tests_json_str = read_to_string(tests_path).unwrap();
    let tests: Vec<Test> = serde_json::from_str(&tests_json_str).unwrap();

    for test in tests {
        let data = decode(test.data).expect("Decoding failed");
        assert_eq!(data.len(), test.length);

        let hash = get_hash::<sha3::Sha3_256>(&data);
        let hash_hex = encode(hash);
        assert_eq!(hash_hex, test.hash.to_lowercase());
    }
}

#[test]
fn test1_keys() {
    #[derive(Deserialize, Debug)]
    #[allow(non_snake_case)]
    struct Test {
        privateKey: String,
        publicKey: String,
    }

    let tests_path = TEST_VECTERS_PATH.to_string() + "/crypto/1.test-keys.json";
    let tests_json_str = read_to_string(tests_path).unwrap();
    let tests: Vec<Test> = serde_json::from_str(&tests_json_str).unwrap();

    for test in tests {
        let private_key = PrivateKey::from_str(&test.privateKey).unwrap();
        let public_key = PublicKey::from_str(&test.publicKey).unwrap();
        assert_eq!(private_key.pubilc_key(), public_key);
    }
}

#[test]
fn test1_address() {
    #[derive(Deserialize, Debug)]
    #[allow(non_snake_case)]
    struct Test {
        publicKey: String,
        address_Public: String,
        address_PublicTest: String,
        // address_Private: String, // TODO:
        // address_PrivateTest: String, // TODO:
    }

    let tests_path = TEST_VECTERS_PATH.to_string() + "/crypto/1.test-address.json";
    let tests_json_str = read_to_string(tests_path).unwrap();
    let tests: Vec<Test> = serde_json::from_str(&tests_json_str).unwrap();

    for test in tests {
        let pubilc_key = PublicKey::from_str(&test.publicKey).unwrap();

        let address_public = pubilc_key.address(MAINNET);
        assert_eq!(address_public.to_string(), test.address_Public);

        let address_public_test = pubilc_key.address(TESTNET);
        assert_eq!(address_public_test.to_string(), test.address_PublicTest);
    }
}

#[test]
fn test2_sign() {
    #[derive(Deserialize, Debug)]
    #[allow(non_snake_case)]
    struct Test {
        privateKey: String,
        publicKey: String,
        length: usize,
        data: String,
        signature: String,
    }

    let tests_path = TEST_VECTERS_PATH.to_string() + "/crypto/2.test-sign.json";
    let tests_json_str = read_to_string(tests_path).unwrap();
    let tests: Vec<Test> = serde_json::from_str(&tests_json_str).unwrap();

    for test in tests {
        let private_key = PrivateKey::from_str(&test.privateKey).unwrap();
        let public_key = PublicKey::from_str(&test.publicKey).unwrap();
        assert_eq!(private_key.pubilc_key(), public_key);

        let data = decode(test.data).expect("Decoding failed");
        assert_eq!(data.len(), test.length);

        let signature = private_key.sign(&data);
        assert_eq!(signature.to_string(), test.signature);

        public_key.verify(&data, &signature).unwrap();
    }
}

#[test]
fn test3_derive_hkdf() {
    use hkdf::Hkdf;
    use sha2::Sha512;
    use sha2::Sha256;
    use curve25519_dalek::scalar::Scalar;
    use curve25519_dalek::edwards::CompressedEdwardsY;

    #[derive(Deserialize, Debug)]
    #[allow(non_snake_case)]
    struct Test {
        privateKey: String,
        otherPublicKey: String,
        sharedKey: String,
        // scalarMulResult: String,
        // Ignoring the variable scalarMulResult is allowed.
        // For details, see https://github.com/symbol/symbol/tree/dev/tests/vectors/README.md.
    }

    let tests_path = TEST_VECTERS_PATH.to_string() + "/crypto/3.test-derive-hkdf.json";
    let tests_json_str = read_to_string(tests_path).unwrap();
    let tests: Vec<Test> = serde_json::from_str(&tests_json_str).unwrap();

    for test in tests {
        let private_key = PrivateKey::from_str(&test.privateKey).unwrap();
        let other_public_key = PublicKey::from_str(&test.otherPublicKey).unwrap();
        assert_ne!(private_key.pubilc_key(), other_public_key);

        let unpacked_pub = CompressedEdwardsY(other_public_key.to_bytes()).decompress().unwrap();

        let mut scalar = get_hash::<Sha512>(private_key.as_bytes())[..32].to_vec();
        scalar[0] &= 248;
        scalar[31] &= 127;
        scalar[31] |= 64;

        #[allow(deprecated)]
        // This scalar is used only for multiplication with EdwardsPoint.
        let scalar = Scalar::from_bits(scalar.as_slice().try_into().unwrap());

        let shared_secret = (scalar * unpacked_pub).compress();

        let hkdf = Hkdf::<Sha256>::new(None, &shared_secret.to_bytes());
        let mut shared_key = [0u8; 32];
        hkdf.expand(b"catapult", &mut shared_key).unwrap();

        let expected = decode(test.sharedKey).unwrap();
        assert_eq!(expected, shared_key);
    }
}

