use std::str::FromStr;
use zklink_signers::eth_signer::H256;
use zklink_wallet::basic_types::{
    AccountId, BigUint, ChainId, Nonce, SubAccountId, TimeStamp, TokenId, ZkLinkAddress,
};
use zklink_wallet::signer::Signer;
use zklink_wallet::tx_type::change_pubkey::ChangePubKeyAuthData;
use zklink_wallet::tx_type::zklink_tx::ZkLinkTx;
use zklink_wallet::ChangePubKeyAuthRequest;

fn main() {
    let eth_pk = H256::repeat_byte(5);
    assert_eq!(
        format!("{:0x}", eth_pk),
        "0505050505050505050505050505050505050505050505050505050505050505"
    );
    let signer = Signer::from_eth(&eth_pk).unwrap();

    let signature = signer
        .sign_change_pub_key(
            AccountId(2),
            ChainId(1),
            SubAccountId(1),
            TokenId(1),
            BigUint::from_str("0").unwrap(),
            None,
            Nonce(0),
            ZkLinkAddress::from_str("0x0000000000000000000000000000000000000000").unwrap(),
            1,
            ZkLinkAddress::from_str("0x0000000000000000000000000000000000000000").unwrap(),
            ChangePubKeyAuthRequest::EthECDSA,
            TimeStamp(1654776640),
        )
        .unwrap();

    assert!(signature.eth_signature.is_none());

    if let ZkLinkTx::ChangePubKey(zk_sign) = signature.tx {
        assert_eq!(hex::encode(zk_sign.signature.signature.as_bytes()), "c752b1a5c0059b35e192d8b051efe11beeb3e3cbdd1803c9ede0b1a1a62f4e1eaff3616c93aeaa837b9ac93db03aa43b65c36ac53464ffd827228e15c82f4c01");

        match zk_sign.eth_auth_data {
            ChangePubKeyAuthData::EthECDSA {
                eth_signature: sign,
            } => {
                assert_eq!(sign.as_hex(), "0xa80272603526ee86c5d27413d8968951b8476a781e0f98e5971ca0185a56d6511836eedc465b27d76363713a148a9726dd5639a9d1bbb6db1437c6bfd3858ad21b")
            }
            _ => panic!("expect ecdsa authtype"),
        }
    } else {
        panic!("must is ChangePubKey")
    }
}
