use zklink_sdk_signers::zklink_signer::error::ZkSignerError;
use zklink_sdk_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_sdk_types::prelude::{GetBytes, TxSignature};
use zklink_sdk_types::tx_type::forced_exit::ForcedExit;

pub fn sign_forced_exit(zklink_signer: &ZkLinkSigner, mut tx: ForcedExit) -> Result<TxSignature, ZkSignerError> {
    tx.signature = zklink_signer.sign_musig(&tx.get_bytes())?;
    Ok(TxSignature {
        tx: tx.into(),
        layer1_signature: None,
    })
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
            withdraw_to_l1: false,
            timestamp: TimeStamp(1649749979),
        };
        let tx = builder.build();

        let signature = sign_forced_exit(&zk_signer, tx).unwrap();
        assert!(signature.layer1_signature.is_none());

        if let ZkLinkTx::ForcedExit(zk_sign) = signature.tx {
            assert_eq!(zk_sign.signature.signature.as_hex(), "0xff9ee61170cc7ebb16b1061f7434cf82e74cc37d809a16cfc6b7dd6554e5ef8538e76e847b434c10cbd21e09522f642735edc8f76c009901aca1b1672cd0ce03");
        } else {
            panic!("must be forcedExit")
        }
    }
}
