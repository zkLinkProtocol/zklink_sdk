use serde_json::json;
use std::str::FromStr;
use zklink_signers::eth_signer::H256;
use zklink_wallet::basic_types::{AccountId, BigUint, Nonce, SlotId, SubAccountId, TokenId};
use zklink_wallet::signer::Signer;
use zklink_wallet::tx_type::zklink_tx::ZkLinkTx;

fn main() {
    let eth_pk = H256::repeat_byte(5);
    assert_eq!(
        format!("{:0x}", eth_pk),
        "0505050505050505050505050505050505050505050505050505050505050505"
    );
    let signer = Signer::from_eth(&eth_pk).unwrap();
    let order_maker = signer
        .sign_order(
            AccountId(6),
            SubAccountId(1),
            SlotId(1),
            Nonce(1),
            TokenId(32),
            TokenId(1),
            BigUint::from_str("100000000000000000000").unwrap(),
            BigUint::from_str("1500000000000000000").unwrap(),
            false,
            5,
            10,
        )
        .unwrap();

    let order_taker = signer
        .sign_order(
            AccountId(6),
            SubAccountId(1),
            SlotId(3),
            Nonce(0),
            TokenId(32),
            TokenId(1),
            BigUint::from_str("1000000000000000000").unwrap(),
            BigUint::from_str("1500000000000000000").unwrap(),
            true,
            5,
            10,
        )
        .unwrap();

    let signature = json!(
        {
            "pubKey": "0x77aa48808967258ac4c115ab14249a4d0b9888360bfb0079ab981822195b3d0c",
            "signature": "2cd57a2c1b3477994b224e9c3bc04e913d31b94b540d7c0bb918b2f54430da18f309a1613e97d5b7168b6a8b176d0cc6c0b444cb03918be187b2d7d97265af03"
        }
    );
    assert_eq!(
        serde_json::to_value(order_maker.signature.clone()).unwrap(),
        signature
    );

    let signature = json!(
        {
            "pubKey": "0x77aa48808967258ac4c115ab14249a4d0b9888360bfb0079ab981822195b3d0c",
            "signature": "d5fb216a16a2de103f8f8281b631825c5ecc923012269f88fa3eb170dd20628f48a540ef7baf29ac9de2ee6e552c504bc5c34623a9f1782ed229b1930c99a900"
        }
    );
    assert_eq!(
        serde_json::to_value(order_taker.signature.clone()).unwrap(),
        signature
    );

    let signature = signer
        .sign_order_matching(
            AccountId(6),
            SubAccountId(1),
            order_taker,
            order_maker,
            TokenId(1),
            BigUint::from_str("0").unwrap(),
            BigUint::from_str("1000000000000000000").unwrap(),
            BigUint::from_str("1500000000000000000").unwrap(),
        )
        .unwrap();

    assert!(signature.eth_signature.is_none());

    if let ZkLinkTx::OrderMatching(zk_sign) = signature.tx {
        assert_eq!(hex::encode(zk_sign.signature.signature.as_bytes()), "7f8126c3e032cba9f0877f0ad7016b4c14e7171de50aa387f97f89611f2a11976a8ff34ed3bfa1678b52365416f62d9a4f94e29026cb1f23e0d81335c87f6601");
    } else {
        panic!("must is ordermatching")
    }
}
