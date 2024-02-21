use symbol::symbol::models_extensions::*;

// accountのaddressの入手方法が不明, 公開鍵からもアドレスを入手可能？
// UnresolvedAddressにfrom_public_key。
// merkleHash ????
// messageの暗号化,AES-GCM形式
// deadlineの便利なメソッド　https://github.com/xembook/quick_learning_symbol/blob/main/04_transaction.md#%E6%9C%89%E5%8A%B9%E6%9C%9F%E9%99%90
// 暗号化した事を示すフラグ？　https://github.com/xembook/quick_learning_symbol/blob/main/04_transaction.md#%E6%9A%97%E5%8F%B7%E6%96%87%E3%83%A1%E3%83%83%E3%82%BB%E3%83%BC%E3%82%B8
// ノードとの通信関係
// mosaicsのsort

// TODO
// https://github.com/symbol/symbol/blob/fd5d13c72c134e1878754780c52f3015a781f29f/sdk/python/examples/docs/__main__.py#L136 の再現
// https://github.com/symbol/symbol/blob/fd5d13c72c134e1878754780c52f3015a781f29f/sdk/python/examples/docs/__main__.py#L149 の再現

fn main() {
    let t = TransferTransactionV1::new(
        VerifyingKey::from_str("A59277D56E9F4FA46854F5EFAAA253B09F8AE69A473565E01FD9E6A738E4AB74")
            .unwrap(),
        NetworkType::TESTNET,
        Amount(186),
        Timestamp(41998024783),
        UnresolvedAddress::from_str("TCHBDENCLKEBILBPWP3JPB2XNY64OE7PYHHE32I").unwrap(),
        vec![UnresolvedMosaic::new(
            UnresolvedMosaicId(0x7CDF3B117A3C40CC),
            Amount(1000000),
        )],
        vec![],
    );
    println!("{:#?}", t.signature);

    let s = SigningKey::from_bytes(
        "A59277D56E9F4FA46854F5EFAAA253B0"
            .as_bytes()
            .try_into()
            .unwrap(),
    );

    let t = s.sign_transaction(t);

    println!("{:#?}", t.signature);

    let v = s.verify_transaction(&t).is_ok();
    println!("{}", v);

    println!(
        "{:#?}",
        UnresolvedAddress::from_str(
            "D4933FC1E4C56F9DF9314E9E0533173E1AB727BDB2A04B59F048124E93BEFBD2"
        )
    );

    let _m = MosaicDefinitionTransactionV1::new(
        VerifyingKey::default(),
        NetworkType::TESTNET,
        Amount(0),
        Timestamp(10),
        MosaicId::default(),
        BlockDuration(1),
        MosaicNonce(1),
        MosaicFlags::RESTRICTABLE,
        3,
    );

    // let et = EmbeddedTransaction::new(VerifyingKey::default(), NetworkType::TESTNET);

    // let at = AggregateCompleteTransactionV2::new(
    //     VerifyingKey::default(),
    //     NetworkType::TESTNET,
    //     Amount(0),
    //     Timestamp(1),
    //     Hash256::default(),
    //     vec![],
    //     vec![],
    // );

    // let tt = TransferTransactionV1::new(
    //     VerifyingKey::default(),
    //     NetworkType::TESTNET,
    //     Amount(0),
    //     Timestamp(1),
    //     UnresolvedAddress::default(),
    //     vec![],
    //     "hello".into(),
    // );
}
