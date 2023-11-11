use symbol::symbol::models_extensions::*;
fn main() {
    let a = AggregateCompleteTransactionV1::new(
        PublicKey::default(),
        NetworkType::TESTNET,
        Amount(3),
        Timestamp(3),
        Hash256::default(),
        vec![EmbeddedAccountKeyLinkTransactionV1::default().into()],
        vec![Cosignature::default()],
    );

    println!("{:#?}", a);
}
