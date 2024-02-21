use hex::{decode, encode};
use serde::Deserialize;
use serde_json;
use std::fs::read_to_string;

use symbol::symbol::models_extensions::*;

const TEST_VECTERS_PATH: &str = "../../tests/vectors/symbol";

#[test]
fn test0_keccak_256() {
    use sha3::{Digest, Keccak256};

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

        let mut hasher = Keccak256::new();
        hasher.update(&data);
        let result = hasher.finalize();
        let result_hex = encode(result);
        assert_eq!(result_hex, test.hash.to_lowercase());
    }
}

#[test]
fn test0_sha3_256() {
    use sha3::{Digest, Sha3_256};

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

        let mut hasher = Sha3_256::new();
        hasher.update(&data);
        let result = hasher.finalize();
        let result_hex = encode(result);
        assert_eq!(result_hex, test.hash.to_lowercase());
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
        assert_eq!(private_key.verifying_key(), public_key);
    }
}

// #[test]
// fn test1_address() {
//     #[derive(Deserialize)]
//     #[derive(Debug)]
//     #[allow(non_snake_case)]
//     struct Test {
//         publicKey: String,
//         address_Public: String,
//         address_PublicTest: String,
//         address_Private: String,
//         address_PrivateTest: String,
//     }

//     let tests_path = TEST_VECTERS_PATH.to_string() + "/crypto/1.test-address.json";
//     let tests_json_str = read_to_string(tests_path).unwrap();
//     let tests: Vec<Test> = serde_json::from_str(&tests_json_str).unwrap();

//     for test in tests {
//         let data = hex::decode(test.address_Public).unwrap();
//         let a = Address::deserialize(&data).unwrap();
//         dbg!(a);
//         assert!(false)
//     }
// }
