use hex::{decode, encode};
use serde::Deserialize;
use std::{fs::read_to_string, str::FromStr};

use symbol::symbol::prelude::*;

const TEST_VECTERS_PATH: &str = "../../tests/vectors/symbol";

#[cfg(test)]
fn get_hash<Hasher: digest::Digest>(data: impl AsRef<[u8]>) -> Vec<u8> {
    Hasher::new().chain_update(data).finalize().to_vec()
}

#[test]
fn test0_keccak_256() {
    use sha3::Keccak256;

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

        let hash = get_hash::<Keccak256>(&data);
        let hash_hex = encode(hash);
        assert_eq!(hash_hex, test.hash.to_lowercase());
    }
}

#[test]
fn test0_sha3_256() {
    use sha3::Sha3_256;

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

        let hash = get_hash::<Sha3_256>(&data);
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
#[cfg(not(feature = "private_network"))]
fn test1_address_with_public_network() {
    #[derive(Deserialize, Debug)]
    #[allow(non_snake_case)]
    struct Test {
        publicKey: String,
        address_Public: String,
        address_PublicTest: String,
    }

    let tests_path = TEST_VECTERS_PATH.to_string() + "/crypto/1.test-address.json";
    let tests_json_str = read_to_string(tests_path).unwrap();
    let tests: Vec<Test> = serde_json::from_str(&tests_json_str).unwrap();

    for test in tests {
        let pubilc_key = PublicKey::from_str(&test.publicKey).unwrap();

        let address_public = pubilc_key.address(NetworkType::MAINNET);
        assert_eq!(address_public.to_string(), test.address_Public);
        assert_eq!(
            address_public,
            UnresolvedAddress::from_str(&test.address_Public).unwrap()
        );

        let address_public_test = pubilc_key.address(NetworkType::TESTNET);
        assert_eq!(address_public_test.to_string(), test.address_PublicTest);
        assert_eq!(
            address_public_test,
            UnresolvedAddress::from_str(&test.address_PublicTest).unwrap()
        );
    }
}

#[test]
#[cfg(feature = "private_network")]
fn test1_address_with_private_network() {
    const MAIN_NETWORKTYPE: NetworkType = NetworkType(0x78);
    const TEST_NETWORKTYPE: NetworkType = NetworkType(0xA8);

    #[derive(Deserialize, Debug)]
    #[allow(non_snake_case)]
    struct Test {
        publicKey: String,
        address_Private: String,
        address_PrivateTest: String,
    }

    let tests_path = TEST_VECTERS_PATH.to_string() + "/crypto/1.test-address.json";
    let tests_json_str = read_to_string(tests_path).unwrap();
    let tests: Vec<Test> = serde_json::from_str(&tests_json_str).unwrap();

    for test in tests {
        let pubilc_key = PublicKey::from_str(&test.publicKey).unwrap();

        let address_public = pubilc_key.address(MAIN_NETWORKTYPE);
        assert_eq!(address_public.to_string(), test.address_Private);
        assert_eq!(
            address_public,
            UnresolvedAddress::from_str(&test.address_Private).unwrap()
        );

        let address_public_test = pubilc_key.address(TEST_NETWORKTYPE);
        assert_eq!(address_public_test.to_string(), test.address_PrivateTest);
        assert_eq!(
            address_public_test,
            UnresolvedAddress::from_str(&test.address_PrivateTest).unwrap()
        );
    }
}

#[test]
fn test2_sign() {
    use ed25519_dalek::{Signer, Verifier};

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
        assert_eq!(signature, Signature::from_str(&test.signature).unwrap());

        public_key.verify(&data, &signature).unwrap();
    }
}

#[test]
fn test3_derive_hkdf() {
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

        let shared_key = private_key.shared_key(other_public_key);

        let expected = SharedKey(
            decode(test.sharedKey)
                .unwrap()
                .as_slice()
                .try_into()
                .unwrap(),
        );
        assert_eq!(expected, shared_key);
    }
}

#[test]
fn test4_cipher() {
    #[derive(Deserialize, Debug)]
    #[allow(non_snake_case)]
    struct Test {
        privateKey: String,
        otherPublicKey: String,
        tag: String,
        iv: String,
        cipherText: String,
        clearText: String,
    }

    let tests_path = TEST_VECTERS_PATH.to_string() + "/crypto/4.test-cipher.json";
    let tests_json_str = read_to_string(tests_path).unwrap();
    let tests: Vec<Test> = serde_json::from_str(&tests_json_str).unwrap();

    for test in tests {
        let private_key = PrivateKey::from_str(&test.privateKey).unwrap();
        let other_public_key = PublicKey::from_str(&test.otherPublicKey).unwrap();
        assert_ne!(private_key.pubilc_key(), other_public_key);

        let shared_key = private_key.shared_key(other_public_key);

        let tag_str = test.tag;
        let iv_str = test.iv;
        let chiper_text_str = test.cipherText;
        let clear_text_str = test.clearText;

        let iv_hex = decode(&iv_str).unwrap();
        let clear_text_hex = decode(&clear_text_str).unwrap();
        let chiper_text_hex = decode(chiper_text_str + &tag_str).unwrap();

        let cipher = Cipher::new(shared_key);

        let encrypted_data = cipher
            .encrypt(&clear_text_hex, &iv_hex)
            .expect("encryption failure");
        let decrypted_data = cipher
            .decrypt(&chiper_text_hex, &iv_hex)
            .expect("decryption failure");

        assert_eq!(chiper_text_hex, encrypted_data);
        assert_eq!(clear_text_hex, decrypted_data);
    }
}

#[test]
#[cfg(not(feature = "private_network"))]
fn test5_mosaic_with_public_network() {
    #[derive(Deserialize, Debug)]
    #[allow(non_snake_case)]
    struct Test {
        mosaicNonce: u32,
        publicKey: String,
        address_Public: String,
        address_PublicTest: String,
        mosaicId_Public: String,
        mosaicId_PublicTest: String,
    }

    let tests_path = TEST_VECTERS_PATH.to_string() + "/crypto/5.test-mosaic-id.json";
    let tests_json_str = read_to_string(tests_path).unwrap();
    let tests: Vec<Test> = serde_json::from_str(&tests_json_str).unwrap();

    for test in tests {
        let mosaic_nonce = MosaicNonce(test.mosaicNonce);
        let public_key = PublicKey::from_str(&test.publicKey).unwrap();
        let address = public_key.address(NetworkType::MAINNET);
        let test_address = public_key.address(NetworkType::TESTNET);

        assert_eq!(
            UnresolvedAddress::from_str(&test.address_Public).unwrap(),
            address
        );
        assert_eq!(
            UnresolvedAddress::from_str(&test.address_PublicTest).unwrap(),
            test_address
        );

        let mosaic_id_public = address.mosaic_id(&mosaic_nonce);
        let mosaic_id_public_test = test_address.mosaic_id(&mosaic_nonce);

        assert_eq!(
            MosaicId::from_str(&test.mosaicId_Public).unwrap(),
            mosaic_id_public
        );
        assert_eq!(
            MosaicId::from_str(&test.mosaicId_PublicTest).unwrap(),
            mosaic_id_public_test
        );
    }
}

#[test]
#[cfg(feature = "private_network")]
fn test5_mosaic_with_private_network() {
    const MAIN_NETWORKTYPE: NetworkType = NetworkType(0x78);
    const TEST_NETWORKTYPE: NetworkType = NetworkType(0x80);

    #[derive(Deserialize, Debug)]
    #[allow(non_snake_case)]
    struct Test {
        mosaicNonce: u32,
        publicKey: String,
        address_Private: String,
        address_PrivateTest: String,
        mosaicId_Private: String,
        mosaicId_PrivateTest: String,
    }

    let tests_path = TEST_VECTERS_PATH.to_string() + "/crypto/5.test-mosaic-id.json";
    let tests_json_str = read_to_string(tests_path).unwrap();
    let tests: Vec<Test> = serde_json::from_str(&tests_json_str).unwrap();

    for test in tests {
        let mosaic_nonce = MosaicNonce(test.mosaicNonce);
        let public_key = PublicKey::from_str(&test.publicKey).unwrap();
        let address = public_key.address(MAIN_NETWORKTYPE);
        let test_address = public_key.address(TEST_NETWORKTYPE);

        assert_eq!(
            UnresolvedAddress::from_str(&test.address_Private).unwrap(),
            address
        );
        assert_eq!(
            UnresolvedAddress::from_str(&test.address_PrivateTest).unwrap(),
            test_address
        );

        let mosaic_id_public = address.mosaic_id(&mosaic_nonce);
        let mosaic_id_public_test = test_address.mosaic_id(&mosaic_nonce);

        assert_eq!(
            MosaicId::from_str(&test.mosaicId_Private).unwrap(),
            mosaic_id_public
        );
        assert_eq!(
            MosaicId::from_str(&test.mosaicId_PrivateTest).unwrap(),
            mosaic_id_public_test
        );
    }
}

#[test]
fn test6_bip32_derivation() {
    #[derive(Deserialize, Debug)]
    #[allow(non_snake_case)]
    struct Test {
        public_net: Vec<NetTest>,
        test_net: Vec<NetTest>,
        test_vectors: Vec<NetTest>,
    }

    #[derive(Deserialize, Debug)]
    #[allow(non_snake_case)]
    struct NetTest {
        seed: String,
        rootPublicKey: String,
        childAccounts: Vec<ChildAccount>,
    }

    #[derive(Deserialize, Debug)]
    #[allow(non_snake_case)]
    struct ChildAccount {
        path: Vec<u32>,
        publicKey: String,
    }

    let tests_path = TEST_VECTERS_PATH.to_string() + "/crypto/6.test-hd-derivation.json";
    let tests_json_str = read_to_string(tests_path).unwrap();
    let tests: Test = serde_json::from_str(&tests_json_str).unwrap();

    for test in tests
        .public_net
        .into_iter()
        .chain(tests.test_net)
        .chain(tests.test_vectors)
    {
        let seed = decode(test.seed).unwrap();

        let root_node = Bip32Node::from_seed(&seed).unwrap();
        assert_eq!(
            root_node.public_key(),
            PublicKey::from_str(&test.rootPublicKey).unwrap()
        );

        for case in test.childAccounts {
            let path = case.path;
            let expected_public_key = PublicKey::from_str(&case.publicKey).unwrap();
            let public_key = root_node.derive_path(&path).unwrap().public_key();

            assert_eq!(public_key, expected_public_key);
        }
    }
}

#[test]
fn test6_bip39_derivation() {
    #[derive(Deserialize, Debug)]
    #[allow(non_snake_case)]
    struct Test {
        public_net: Vec<NetTest>,
        test_net: Vec<NetTest>,
    }

    #[derive(Deserialize, Debug)]
    #[allow(non_snake_case)]
    struct NetTest {
        mnemonic: Option<String>,
        passphrase: Option<String>,
        seed: String,
    }

    let tests_path = TEST_VECTERS_PATH.to_string() + "/crypto/6.test-hd-derivation.json";
    let tests_json_str = read_to_string(tests_path).unwrap();
    let tests: Test = serde_json::from_str(&tests_json_str).unwrap();

    for test in tests.public_net.into_iter().chain(tests.test_net) {
        let mnemonic = test.mnemonic;
        let seed = decode(test.seed).unwrap();
        let passphrase = test.passphrase;

        if mnemonic.is_some() && passphrase.is_some() {
            let mnemonic = Mnemonic::from_str(&mnemonic.unwrap()).unwrap();
            let passphrase = passphrase.unwrap();
            assert_eq!(&seed, &mnemonic.to_seed_normalized(&passphrase));
        }
    }
}

#[test]
fn test7_voting_keys_generation() {
    #[derive(Deserialize, Debug)]
    #[allow(non_snake_case)]
    struct Test {
        name: String,
        rootPrivateKey: String,
        startEpoch: u64,
        endEpoch: u64,
        expectedFileHex: String,
    }

    let tests_path = TEST_VECTERS_PATH.to_string() + "/crypto/7.test-voting-keys-generation.json";
    let tests_json_str = read_to_string(tests_path).unwrap();
    let tests: Vec<Test> = serde_json::from_str(&tests_json_str).unwrap();

    struct SeededPrivateKeyGenerator {
        values: Vec<PrivateKey>,
        next_index: usize,
    }

    impl SeededPrivateKeyGenerator {
        fn new(values: Vec<PrivateKey>) -> Self {
            Self {
                values: values,
                next_index: 0,
            }
        }
    }

    impl KeyGenerator for SeededPrivateKeyGenerator {
        fn generate(&mut self) -> PrivateKey {
            self.next_index += 1;
            self.values[self.next_index - 1].clone()
        }
    }

    struct FibPrivateKeyGenerator {
        fill_private_key: bool,
        value1: u32,
        value2: u32,
    }

    impl FibPrivateKeyGenerator {
        fn new(fill_private_key: bool) -> Self {
            Self {
                fill_private_key: fill_private_key,
                value1: 1,
                value2: 2,
            }
        }
    }

    impl KeyGenerator for FibPrivateKeyGenerator {
        fn generate(&mut self) -> PrivateKey {
            let next_value = self.value1 + self.value2;
            self.value1 = self.value2;
            self.value2 = next_value;

            let seed_value = next_value as u8;

            let mut private_key_buffer = [0u8; PrivateKey::SIZE];
            if self.fill_private_key {
                for i in 0..PrivateKey::SIZE {
                    private_key_buffer[i] = (seed_value as u16 + i as u16) as u8;
                }
            } else {
                private_key_buffer[private_key_buffer.len() - 1] = seed_value;
            }

            PrivateKey::from_bytes(&private_key_buffer)
        }
    }

    for test in tests {
        let name = test.name;
        let root_private_key = PrivateKey::from_str(&test.rootPrivateKey).unwrap();
        let start_epoch = test.startEpoch;
        let end_epoch = test.endEpoch;
        let expected_file_hex = decode(test.expectedFileHex).unwrap();

        let voting_keys_buffer = match name.as_str() {
            "test_vector_1" => {
                VotingKeysGenerator::new(root_private_key, FibPrivateKeyGenerator::new(false))
                    .generate(start_epoch, end_epoch)
            }
            "test_vector_2" => {
                VotingKeysGenerator::new(root_private_key, FibPrivateKeyGenerator::new(true))
                    .generate(start_epoch, end_epoch)
            }
            "test_vector_3" => VotingKeysGenerator::new(
                root_private_key,
                SeededPrivateKeyGenerator::new(vec![
                    PrivateKey::from_str(
                        "12F98B7CB64A6D840931A2B624FB1EACAFA2C25C3EF0018CD67E8D470A248B2F",
                    )
                    .unwrap(),
                    PrivateKey::from_str(
                        "B5593870940F28DAEE262B26367B69143AD85E43048D23E624F4ED8008C0427F",
                    )
                    .unwrap(),
                    PrivateKey::from_str(
                        "6CFC879ABCCA78F5A4C9739852C7C643AEC3990E93BF4C6F685EB58224B16A59",
                    )
                    .unwrap(),
                ]),
            )
            .generate(start_epoch, end_epoch),
            _ => panic!("unexpected."),
        }
        .unwrap();

        assert_eq!(expected_file_hex, voting_keys_buffer);
    }
}
