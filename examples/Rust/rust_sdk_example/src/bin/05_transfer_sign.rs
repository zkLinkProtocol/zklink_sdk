use std::str::FromStr;
use zklink_sdk_signers::eth_signer::H256;
use zklink_wallet::basic_types::{
    AccountId, BigUint, Nonce, SubAccountId, TimeStamp, TokenId, ZkLinkAddress,
};
use zklink_wallet::signer::Signer;
use zklink_wallet::tx_type::zklink_tx::ZkLinkTx;

fn main() {
    let eth_pk = H256::repeat_byte(5);
    let signer = Signer::from_eth(&eth_pk).unwrap();

    let signature = signer
        .sign_transfer(
            AccountId(1),
            SubAccountId(1),
            ZkLinkAddress::from_str("0x0000000000000000000000000000000000000000").unwrap(),
            SubAccountId(1),
            TokenId(1),
            "USD".into(),
            BigUint::from_str("1000000000000000000").unwrap(),
            BigUint::from_str("10000000000").unwrap(),
            Nonce(1),
            TimeStamp(1646101085),
        )
        .unwrap();

    let eth_sign = signature
        .eth_signature
        .expect("transfer must has eth signature");
    assert_eq!(eth_sign.as_hex(), "0x08c9cd25416c871a153e9d51385c28413311e8ed055a195e4f5e8c229244e1a05bab15a9e6eb1cff9a5d237d878c41553215341742779745574a631d89e09a831b");

    if let ZkLinkTx::Transfer(zk_sign) = signature.tx {
        assert_eq!(hex::encode(zk_sign.signature.signature.as_bytes()), "2aa6ebe4695f2c57e79fc284f87098ffefed9d4a53adadcd601b69bc3825511e5c859a5345526e52a77660e993dd92322fef64ad4521847ecd0215b556487902");
    } else {
        panic!("must is transfer")
    }
}
