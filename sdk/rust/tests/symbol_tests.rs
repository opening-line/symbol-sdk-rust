use symbol::symbol::models_extensions::*;

#[test]
fn test_symbol_1() {
    let tx = TransferTransactionV1::new(
        VerifyingKey::from_str("87DA603E7BE5656C45692D5FC7F6D0EF8F24BB7A5C10ED5FDA8C5CFBC49FCBC8")
            .unwrap(),
        NetworkType::TESTNET,
        Amount(1000000),
        Timestamp(41998024783),
        UnresolvedAddress::from_str("TCHBDENCLKEBILBPWP3JPB2XNY64OE7PYHHE32I").unwrap(),
        vec![UnresolvedMosaic::new(
            UnresolvedMosaicId(0x7CDF3B117A3C40CC),
            Amount(1000000),
        )],
        vec![],
    );

    let s_key = SigningKey::from_bytes(
        "A59277D56E9F4FA46854F5EFAAA253B0"
            .as_bytes()
            .try_into()
            .unwrap(),
    );

    let signed_tx = s_key.sign_transaction(tx);
    s_key.verify_transaction(&signed_tx).unwrap();
}
