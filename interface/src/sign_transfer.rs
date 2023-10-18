use crate::error::SignError;
#[cfg(feature = "ffi")]
use std::sync::Arc;
#[cfg(feature = "web")]
use zklink_sdk_signers::eth_signer::json_rpc_signer::JsonRpcSigner;
#[cfg(not(feature = "web"))]
use zklink_sdk_signers::eth_signer::pk_signer::EthSigner;
use zklink_sdk_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_sdk_types::prelude::TxSignature;
use zklink_sdk_types::tx_type::transfer::Transfer;
use zklink_sdk_types::tx_type::TxTrait;

#[cfg(not(feature = "web"))]
pub fn sign_transfer(
    eth_signer: &EthSigner,
    zklink_syner: &ZkLinkSigner,
    mut tx: Transfer,
    token_symbol: &str,
) -> Result<TxSignature, SignError> {
    tx.signature = zklink_syner.sign_musig(&tx.get_bytes())?;
    let message = tx.get_eth_sign_msg(token_symbol).as_bytes().to_vec();
    let eth_signature = eth_signer.sign_message(&message)?;

    Ok(TxSignature {
        tx: tx.into(),
        eth_signature: Some(eth_signature),
    })
}

#[cfg(feature = "web")]
pub async fn sign_transfer(
    eth_signer: &JsonRpcSigner,
    zklink_syner: &ZkLinkSigner,
    mut tx: Transfer,
    token_symbol: &str,
) -> Result<TxSignature, SignError> {
    tx.signature = zklink_syner.sign_musig(&tx.get_bytes())?;
    let message = tx.get_eth_sign_msg(token_symbol).as_bytes().to_vec();
    let eth_signature = eth_signer.sign_message(&message).await?;

    Ok(TxSignature {
        tx: tx.into(),
        eth_signature: Some(eth_signature),
    })
}

#[cfg(feature = "ffi")]
pub fn create_signed_transfer(
    zklink_syner: Arc<ZkLinkSigner>,
    tx: Arc<Transfer>,
) -> Result<Arc<Transfer>, SignError> {
    let mut tx = (*tx).clone();
    tx.signature = zklink_syner.sign_musig(&tx.get_bytes())?;
    Ok(Arc::new(tx))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use zklink_sdk_types::basic_types::BigUint;
    use zklink_sdk_types::prelude::*;

    #[test]
    fn test_sign_transfer() {
        let eth_pk = H256::repeat_byte(5);
        let eth_signer = eth_pk.into();
        let zk_signer = ZkLinkSigner::new_from_eth_signer(&eth_signer).unwrap();
        let builder = TransferBuilder {
            account_id: AccountId(1),
            from_sub_account_id: SubAccountId(1),
            to_sub_account_id: SubAccountId(1),
            to_address: ZkLinkAddress::from_str("0x0000000000000000000000000000000000000000")
                .unwrap(),
            token: TokenId(1),
            amount: BigUint::from_str("1000000000000000000").unwrap(),
            fee: BigUint::from_str("10000000000").unwrap(),
            nonce: Nonce(1),
            timestamp: TimeStamp(1646101085),
        };
        let tx = Transfer::new(builder);

        let signature = sign_transfer(&eth_signer, &zk_signer, tx, "USD").unwrap();
        let eth_sign = signature
            .eth_signature
            .expect("transfer must has eth signature");
        assert_eq!(eth_sign.as_hex(), "0x08c9cd25416c871a153e9d51385c28413311e8ed055a195e4f5e8c229244e1a05bab15a9e6eb1cff9a5d237d878c41553215341742779745574a631d89e09a831b");

        if let ZkLinkTx::Transfer(zk_sign) = signature.tx {
            assert_eq!(zk_sign.signature.signature.as_hex(), "0x2aa6ebe4695f2c57e79fc284f87098ffefed9d4a53adadcd601b69bc3825511e5c859a5345526e52a77660e993dd92322fef64ad4521847ecd0215b556487902");
        } else {
            panic!("must is transfer")
        }
    }
}
