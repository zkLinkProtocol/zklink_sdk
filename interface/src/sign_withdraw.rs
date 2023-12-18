use crate::error::SignError;
// #[cfg(feature = "ffi")]
// use std::sync::Arc;
#[cfg(feature = "web")]
use zklink_sdk_signers::eth_signer::json_rpc_signer::JsonRpcSigner;
#[cfg(not(feature = "web"))]
use zklink_sdk_signers::eth_signer::pk_signer::EthSigner;
#[cfg(feature = "web")]
use zklink_sdk_signers::starknet_signer::starknet_json_rpc_signer::StarknetJsonRpcSigner;
use zklink_sdk_signers::starknet_signer::typed_data::message::TypedDataMessage;
#[cfg(not(feature = "web"))]
use zklink_sdk_signers::starknet_signer::typed_data::TypedData;
#[cfg(not(feature = "web"))]
use zklink_sdk_signers::starknet_signer::StarkSigner;
use zklink_sdk_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_sdk_types::prelude::TxSignature;
use zklink_sdk_types::tx_type::withdraw::Withdraw;
use zklink_sdk_types::tx_type::ZkSignatureTrait;
#[cfg(not(feature = "web"))]
pub fn sign_starknet_withdraw(
    signer: &StarkSigner,
    zklink_singer: &ZkLinkSigner,
    mut tx: Withdraw,
    l2_source_token_symbol: &str,
    chain_id: &str,
    addr: &str,
) -> Result<TxSignature, SignError> {
    tx.sign(zklink_singer)?;
    let message = tx.get_starknet_sign_msg(l2_source_token_symbol);
    // #[cfg(feature = "ffi")]
    // let message = Arc::new(message);
    let typed_data = TypedData::new(
        TypedDataMessage::Transaction { message },
        chain_id.to_string(),
    );
    let signature = signer.sign_message(&typed_data, addr)?;

    Ok(TxSignature {
        tx: tx.into(),
        layer1_signature: Some(signature.into()),
    })
}

#[cfg(not(feature = "web"))]
pub fn sign_eth_withdraw(
    eth_signer: &EthSigner,
    zklink_singer: &ZkLinkSigner,
    mut tx: Withdraw,
    l2_source_token_symbol: &str,
) -> Result<TxSignature, SignError> {
    tx.sign(zklink_singer)?;
    let message = tx.get_eth_sign_msg(l2_source_token_symbol);
    let signature = eth_signer.sign_message(message.as_bytes())?;

    Ok(TxSignature {
        tx: tx.into(),
        layer1_signature: Some(signature.into()),
    })
}

#[cfg(feature = "web")]
pub async fn sign_eth_withdraw(
    eth_signer: &JsonRpcSigner,
    zklink_singer: &ZkLinkSigner,
    mut tx: Withdraw,
    l2_source_token_symbol: &str,
) -> Result<TxSignature, SignError> {
    tx.sign(zklink_singer)?;
    let message = tx.get_eth_sign_msg(l2_source_token_symbol);
    let eth_signature = eth_signer.sign_message(message.as_bytes()).await?;

    Ok(TxSignature {
        tx: tx.into(),
        layer1_signature: Some(eth_signature.into()),
    })
}

#[cfg(feature = "web")]
pub async fn sign_starknet_withdraw(
    stark_signer: &StarknetJsonRpcSigner,
    zklink_singer: &ZkLinkSigner,
    mut tx: Withdraw,
    l2_source_token_symbol: &str,
) -> Result<TxSignature, SignError> {
    tx.sign(zklink_singer)?;
    let message = tx.get_starknet_sign_msg(l2_source_token_symbol);
    let stark_signature = stark_signer
        .sign_message(TypedDataMessage::Transaction { message })
        .await?;

    Ok(TxSignature {
        tx: tx.into(),
        layer1_signature: Some(stark_signature.into()),
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use zklink_sdk_types::basic_types::BigUint;
    use zklink_sdk_types::prelude::*;

    #[test]
    fn test_sign_withdraw() {
        let eth_pk = H256::repeat_byte(5);
        let builder = WithdrawBuilder {
            account_id: AccountId(1),
            to_chain_id: ChainId(3),
            sub_account_id: SubAccountId(1),
            to_address: ZkLinkAddress::from_str("0x3d809e414ba4893709c85f242ba3617481bc4126")
                .unwrap(),
            l2_source_token: TokenId(1),
            l1_target_token: TokenId(17),
            amount: BigUint::from_str("99995900000000000000").unwrap(),
            fee: BigUint::from_str("4100000000000000").unwrap(),
            nonce: Nonce(85),
            withdraw_to_l1: false,
            withdraw_fee_ratio: 50,
            timestamp: TimeStamp(1649749979),
        };
        let tx = builder.build();
        let eth_signer = eth_pk.into();
        let zk_signer = ZkLinkSigner::new_from_eth_signer(&eth_signer).unwrap();
        let signature = sign_eth_withdraw(&eth_signer, &zk_signer, tx, "USD").unwrap();

        // let eth_sign = signature
        //     .layer1_signature
        //     .expect("withdraw must has eth signature");
        // assert_eq!(eth_sign.as_hex(), "0x2499120b362bd835b456f2a8e3e6c4ccef6d0ebbe76fd64d452d5bba600ad574713d6b6af043a8f070c532d1ba879c712235bf8e9af6291aa8bdfb1cbaaa4dc21b");

        if let ZkLinkTx::Withdraw(zk_sign) = signature.tx {
            assert_eq!(zk_sign.signature.signature.as_hex(), "0x7e5545c84e463ffedf8812068452d8555c477b9fda608996fb2f982d5b8dee2d922d79815ae8459d2630e3bf307a1b41346e653881e647861d9e7aca0961f503");
        } else {
            panic!("signature type must be withdraw")
        }
    }
}
