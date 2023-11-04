use symbol::symbol::models_extensions::*;

fn main() {
    let a = TransferTransactionV1::new(
        PublicKey::from_str("A59277D56E9F4FA46854F5EFAAA253B09F8AE69A473565E01FD9E6A738E4AB74")
            .unwrap(),
        NetworkType::TESTNET,
        Amount(0x186A0),
        Timestamp(41998024783),
        UnresolvedAddress::from_str("TCHBDENCLKEBILBPWP3JPB2XNY64OE7PYHHE32I").unwrap(),
        vec![UnresolvedMosaic::new(
            UnresolvedMosaicId(0x7CDF3B117A3C40CC),
            Amount(1000000),
        )],
        vec![],
    );
    println!("{:#?}", a);
}
