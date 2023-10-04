#[cfg(feature = "ffi")]
use std::sync::Arc;
use zklink_sdk_signers::zklink_signer::error::ZkSignerError;
use zklink_sdk_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_sdk_types::prelude::TxSignature;
use zklink_sdk_types::tx_type::forced_exit::ForcedExit;
use zklink_sdk_types::tx_type::TxTrait;

pub fn sign_forced_exit(
    zklink_signer: &ZkLinkSigner,
    mut tx: ForcedExit,
) -> Result<TxSignature, ZkSignerError> {
    tx.signature = zklink_signer.sign_musig(&tx.get_bytes())?;
    Ok(TxSignature {
        tx: tx.into(),
        eth_signature: None,
    })
}

#[cfg(feature = "ffi")]
pub fn create_signed_forced_exit(
    zklink_signer: Arc<ZkLinkSigner>,
    tx: Arc<ForcedExit>,
) -> Result<Arc<ForcedExit>, ZkSignerError> {
    let mut tx = (*tx).clone();
    tx.signature = zklink_signer.sign_musig(&tx.get_bytes())?;
    Ok(Arc::new(tx))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use zklink_sdk_types::basic_types::BigUint;
    use zklink_sdk_types::prelude::*;

    #[test]
    fn test_sign_forced_exit() {
        let eth_pk = H256::repeat_byte(5);
        let eth_signer = eth_pk.into();
        let zk_signer = ZkLinkSigner::new_from_eth_signer(&eth_signer).unwrap();
        let builder = ForcedExitBuilder {
            to_chain_id: ChainId(1),
            initiator_account_id: AccountId(1),
            target_sub_account_id: SubAccountId(0),
            target: ZkLinkAddress::from_str("0x3498F456645270eE003441df82C718b56c0e6666").unwrap(),
            initiator_sub_account_id: SubAccountId(0),
            l2_source_token: TokenId(1),
            l1_target_token: TokenId(17),
            initiator_nonce: Nonce(85),
            exit_amount: BigUint::from_str("4100000000000000").unwrap(),
            timestamp: TimeStamp(1649749979),
        };
        let tx = ForcedExit::new(builder);

        let signature = sign_forced_exit(&zk_signer, tx).unwrap();
        assert!(signature.eth_signature.is_none());

        if let ZkLinkTx::ForcedExit(zk_sign) = signature.tx {
            assert_eq!(zk_sign.signature.signature.as_hex(), "0xda738109b1864b162eba33a3e8a1a9c142dcadfd5d11c0fda37f6a4b0e12cea70f15e605b5f90c655b5a5b0e4e367f62d30d3d70157047db21dd2c70d482d302");
        } else {
            panic!("must be forcedExit")
        }
    }
}
