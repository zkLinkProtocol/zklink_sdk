use serde_json::json;
use std::ops::Deref;
use std::str::FromStr;
use std::time::Duration;
use zklink_provider::network::Network;
use zklink_provider::rpc::ZkLinkRpcClient;
use zklink_provider::ZkLinkRpcProvider;
use zklink_signers::eth_signer::eth_signature::TxEthSignature;
use zklink_signers::eth_signer::pk_signer::EthSigner;
use zklink_signers::eth_signer::H256;
use zklink_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_signers::zklink_signer::signature::ZkLinkSignature;
use zklink_wallet::basic_types::{
    AccountId, BigUint, ChainId, Nonce, SubAccountId, TimeStamp, TokenId, ZkLinkAddress,
};
use zklink_wallet::signer::Signer;
use zklink_wallet::tx_type::zklink_tx::ZkLinkTx;
use zklink_wallet::{AccountType, Wallet};

#[tokio::main]
async fn main() {
    let eth_pk = H256::repeat_byte(5);
    assert_eq!(
        format!("{:0x}", eth_pk),
        "0505050505050505050505050505050505050505050505050505050505050505"
    );
    let address = ZkLinkAddress::from_str("0xd09Ad14080d4b257a819a4f579b8485Be88f086c").unwrap();

    let signer = Signer::from_eth(&eth_pk).unwrap();

    let signature = signer
        .sign_withdraw(
            AccountId(1),
            ChainId(3),
            SubAccountId(1),
            ZkLinkAddress::from_str("0x3d809e414ba4893709c85f242ba3617481bc4126").unwrap(),
            TokenId(1),
            "USD".into(),
            TokenId(17),
            BigUint::from_str("99995900000000000000").unwrap(),
            BigUint::from_str("4100000000000000").unwrap(),
            Nonce(85),
            true,
            50,
            TimeStamp(1649749979),
        )
        .unwrap();

    let eth_sign = signature
        .eth_signature
        .clone()
        .expect("withdraw must has eth signature");
    assert_eq!(eth_sign.as_hex(), "0x2499120b362bd835b456f2a8e3e6c4ccef6d0ebbe76fd64d452d5bba600ad574713d6b6af043a8f070c532d1ba879c712235bf8e9af6291aa8bdfb1cbaaa4dc21b");

    if let ZkLinkTx::Withdraw(zk_sign) = signature.tx.clone() {
        assert_eq!(hex::encode(zk_sign.signature.signature.as_bytes()), "6d782453d4cda0eacda13b53fa5471942ad75ea5010e086df845886ba5407bac82f3c7c04ba58045f7115df52d091a232701c8613d5a8fe31fdbee1846d87f00");
    } else {
        panic!("must is withdraw")
    }

    // You might get this object from the front end or from Example 04 `tx` field of signature

    // Most of the time, the submitter and the signer in the transaction are not the same person, but they can be
    let submitter_private_key =
        "0xb32593e347bf09436b058fbeabc17ebd2c7c1fa42e542f5f78fc3580faef83b7";
    let zk_signer = ZkLinkSigner::new_from_hex_eth_signer(submitter_private_key).unwrap();

    let provider = ZkLinkRpcProvider::new(Network::MainNet, Duration::from_secs(3));
    // let wallet = Wallet::new(provider.deref(), signer, address, AccountType::ECDSA).await.unwrap();
    // wallet.submit_tx(signature, zk_signer).await.unwrap();
}