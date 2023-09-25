use std::str::FromStr;
use zklink_sdk_signers::eth_signer::H256;
use zklink_wallet::basic_types::{
    AccountId, BigUint, ChainId, Nonce, SubAccountId, TimeStamp, TokenId, ZkLinkAddress,
};
use zklink_wallet::signer::Signer;
use zklink_wallet::tx_type::zklink_tx::ZkLinkTx;

fn main() {
    let eth_pk = H256::repeat_byte(5);
    assert_eq!(
        format!("{:0x}", eth_pk),
        "0505050505050505050505050505050505050505050505050505050505050505"
    );
    let signer = Signer::from_eth(&eth_pk).unwrap();

    let signature = signer
        .sign_forced_exit(
            AccountId(1),
            ChainId(1),
            SubAccountId(0),
            ZkLinkAddress::from_str("0x3498F456645270eE003441df82C718b56c0e6666").unwrap(),
            SubAccountId(0),
            TokenId(1),
            TokenId(17),
            Nonce(85),
            BigUint::from_str("4100000000000000").unwrap(),
            TimeStamp(1649749979),
        )
        .unwrap();

    assert!(signature.eth_signature.is_none());

    if let ZkLinkTx::ForcedExit(zk_sign) = signature.tx {
        assert_eq!(hex::encode(zk_sign.signature.signature.as_bytes()), "da738109b1864b162eba33a3e8a1a9c142dcadfd5d11c0fda37f6a4b0e12cea70f15e605b5f90c655b5a5b0e4e367f62d30d3d70157047db21dd2c70d482d302");
    } else {
        panic!("must is forcedExit")
    }
}
