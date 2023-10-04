use crate::error::SignError;
#[cfg(feature = "ffi")]
use std::sync::Arc;
use zklink_sdk_signers::eth_signer::pk_signer::EthSigner;
use zklink_sdk_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_sdk_types::prelude::TxSignature;
use zklink_sdk_types::tx_type::withdraw::Withdraw;
#[cfg(feature = "ffi")]
use zklink_sdk_types::tx_type::TxTrait;
use zklink_sdk_types::tx_type::ZkSignatureTrait;

pub fn sign_withdraw(
    eth_signer: &EthSigner,
    zklink_singer: &ZkLinkSigner,
    mut tx: Withdraw,
    l2_source_token_symbol: &str,
) -> Result<TxSignature, SignError> {
    tx.sign(zklink_singer)?;
    let message = tx.get_eth_sign_msg(l2_source_token_symbol);
    let eth_signature = eth_signer.sign_message(message.as_bytes())?;

    Ok(TxSignature {
        tx: tx.into(),
        eth_signature: Some(eth_signature),
    })
}

#[cfg(feature = "ffi")]
pub fn create_signed_withdraw(
    zklink_singer: Arc<ZkLinkSigner>,
    tx: Arc<Withdraw>,
) -> Result<Arc<Withdraw>, SignError> {
    let mut tx = (*tx).clone();
    tx.signature = zklink_singer.sign_musig(&tx.get_bytes())?;
    Ok(Arc::new(tx))
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
            fast_withdraw: true,
            withdraw_fee_ratio: 50,
            timestamp: TimeStamp(1649749979),
        };
        let tx = Withdraw::new(builder);
        let eth_signer = eth_pk.into();
        let zk_signer = ZkLinkSigner::new_from_eth_signer(&eth_signer).unwrap();
        let signature = sign_withdraw(&eth_signer, &zk_signer, tx, "USD").unwrap();

        let eth_sign = signature
            .eth_signature
            .expect("withdraw must has eth signature");
        assert_eq!(eth_sign.as_hex(), "0x2499120b362bd835b456f2a8e3e6c4ccef6d0ebbe76fd64d452d5bba600ad574713d6b6af043a8f070c532d1ba879c712235bf8e9af6291aa8bdfb1cbaaa4dc21b");

        if let ZkLinkTx::Withdraw(zk_sign) = signature.tx {
            assert_eq!(zk_sign.signature.signature.as_hex(), "0x6d782453d4cda0eacda13b53fa5471942ad75ea5010e086df845886ba5407bac82f3c7c04ba58045f7115df52d091a232701c8613d5a8fe31fdbee1846d87f00");
        } else {
            panic!("signature type must be withdraw")
        }
    }
}
