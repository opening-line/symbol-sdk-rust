use std::fs::read_to_string;
use serde::Deserialize;
use serde_json;
use hex::{encode, decode};

use symbol::symbol::models_extensions::*;

const TEST_VECTERS_PATH: &str = "../../tests/vectors/symbol";

// #[test]
// fn test_keccak_256() {
    
// }

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