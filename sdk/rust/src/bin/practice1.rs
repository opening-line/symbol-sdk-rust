// use ed25519_dalek::Signer;
use symbol::symbol::models::*;

// signatureやその同類はnewでは要らない(defaultでよい)。インターフェースに現れる必要が無い。
// 強力なnewを作成すべきかどうか

fn main() -> Result<(), SymbolError> {

    let a = TransferTransactionV1::new(
        Signature::default(),
        PublicKey::new(
            hex::decode("A59277D56E9F4FA46854F5EFAAA253B09F8AE69A473565E01FD9E6A738E4AB74")?
                .as_slice()
                .try_into()?,
        ),
        NetworkType::TESTNET,
        Amount(0x186A0),
        Timestamp(41998024783),
        UnresolvedAddress::new(
            base32_decode("TCHBDENCLKEBILBPWP3JPB2XNY64OE7PYHHE32I")?
                .as_slice()
                .try_into()?,
        ),
        vec![UnresolvedMosaic::new(
            UnresolvedMosaicId(0x7CDF3B117A3C40CC),
            Amount(1000000),
        )],
        vec![],
    );
    println!("{:#?}", a);
    Ok(())
}

fn base32_decode(input: &str) -> Result<Vec<u8>, data_encoding::DecodeError> {
    use data_encoding::BASE32;
    let input = input.to_string() + &"=".repeat(8 - (input.len() % 8));
    BASE32.decode(&input.as_bytes())
}
