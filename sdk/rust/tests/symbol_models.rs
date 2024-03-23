use hex::{decode, encode};
use serde::Deserialize;
use std::{fs::read_to_string, str::FromStr};

use symbol::symbol::prelude::*;

const TEST_VECTERS_PATH: &str = "../../tests/vectors/symbol";

fn get_hash<Hasher: digest::Digest>(data: impl AsRef<[u8]>) -> Vec<u8> {
    Hasher::new().chain_update(data).finalize().to_vec()
}

#[test]
fn test_transaction() {
    // use sha3::Keccak256;

    // #[derive(Deserialize)]
    // struct Test {
    //     hash: String,
    //     length: usize,
    //     data: String,
    // }

    // let tests_path = TEST_VECTERS_PATH.to_string() + "/models/blocks.json";
    // let tests_json_str = read_to_string(tests_path).unwrap();
    // let tests: Vec<Test> = serde_json::from_str(&tests_json_str).unwrap();

    // for test in tests {
    //     let data = decode(test.data).expect("Decoding failed");
    //     assert_eq!(data.len(), test.length);

    //     let hash = get_hash::<Keccak256>(&data);
    //     let hash_hex = encode(hash);
    //     assert_eq!(hash_hex, test.hash.to_lowercase());
    // }
}

#[test]
#[cfg(not(feature = "private_network"))]
fn my_test() {
    let mut a = AccountAddressRestrictionTransactionV1::new(
        PublicKey::from_str("D294E5E650ACC2A911B548BE2A1806FF4717621BCE3EC1007295219AFFC17B82")
            .unwrap(),
        NetworkType::TESTNET,
        Amount(18370164183782063840),
        Timestamp(8207562320463688160),
        AccountRestrictionFlags::ADDRESS,
        vec![
            UnresolvedAddress::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ").unwrap(),
            UnresolvedAddress::from_str("TD2ASJ2LKL5LX66PPZ67PYQN4HIMH5SX7OCZLQI").unwrap(),
        ],
        vec![UnresolvedAddress::from_str("TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I").unwrap()],
    );
    let signature = Signature::from_str("7695D855CBB6CB83D5BD08E9D76145674F805D741301883387B7101FD8CA84329BB14DBF2F0B4CD58AA84CF31AC6899D134FC38FAB0E7A76F6216ACD60914ACB").unwrap();
    a.set_signature(signature);

    let payload = "D0000000000000007695D855CBB6CB83D5BD08E9D76145674F805D741301883387B7101FD8CA84329BB14DBF2F0B4CD58AA84CF31AC6899D134FC38FAB0E7A76F6216ACD60914ACBD294E5E650ACC2A911B548BE2A1806FF4717621BCE3EC1007295219AFFC17B820000000001985041E0FEEEEFFEEEEFFEE0711EE7711EE77101000201000000009841E5B8E40781CF74DABF592817DE48711D778648DEAFB298F409274B52FABBFBCF7E7DF7E20DE1D0C3F657FB8595C1989059321905F681BCF47EA33BBF5E6F8298B5440854FDED";
    let b = a.serialize();
    dbg!(a.size(), b.len(), decode(payload).unwrap().len());
    dbg!(a);
    assert_eq!(b, decode(payload).unwrap());
}
