use crate::prelude::validator::*;
#[cfg(feature = "ffi")]
use crate::prelude::ExitInfoBuilder;
use crate::prelude::{AccountId, ChainId, GetBytes, SubAccountId, TokenId, ZkLinkAddress};
use crate::tx_type::{TxTrait, ZkSignatureTrait};
use serde::{Deserialize, Serialize};
use validator::Validate;
use zklink_sdk_signers::zklink_signer::ZkLinkSignature;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Validate)]
pub struct ExitInfo {
    #[validate(custom = "chain_id_validator")]
    pub chain_id: ChainId,
    #[validate(custom = "account_validator")]
    pub withdrawal_account_id: AccountId,
    #[validate(custom = "zklink_address_validator")]
    pub received_address: ZkLinkAddress,

    #[validate(custom = "sub_account_validator")]
    pub sub_account_id: SubAccountId,
    #[validate(custom = "token_validator")]
    pub l2_source_token: TokenId,
    #[validate(custom = "token_validator")]
    pub l1_target_token: TokenId,

    pub signature: ZkLinkSignature,
}

impl ExitInfo {
    #[cfg(feature = "ffi")]
    pub fn new(builder: ExitInfoBuilder) -> Self {
        builder.build()
    }
}

impl GetBytes for ExitInfo {
    fn get_bytes(&self) -> Vec<u8> {
        let bytes_len = self.bytes_len();
        let mut out = Vec::with_capacity(bytes_len);
        out.push(*self.chain_id);
        out.extend(self.withdrawal_account_id.to_be_bytes());
        out.push(*self.sub_account_id);
        out.extend(self.received_address.to_fixed_bytes());
        out.extend((*self.l1_target_token as u16).to_be_bytes());
        out.extend((*self.l2_source_token as u16).to_be_bytes());
        assert_eq!(out.len(), bytes_len);
        out
    }

    fn bytes_len(&self) -> usize {
        42
    }
}

impl TxTrait for ExitInfo {}
impl ZkSignatureTrait for ExitInfo {
    fn set_signature(&mut self, signature: ZkLinkSignature) {
        self.signature = signature;
    }

    fn internal_signature(&self) -> ZkLinkSignature {
        self.signature.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exit_info_get_bytes() {
        let tx = ExitInfo::default();
        let bytes = tx.get_bytes();
        assert_eq!(tx.bytes_len(), bytes.len())
    }
}
