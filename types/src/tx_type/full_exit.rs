use super::validator::*;
use crate::basic_types::{AccountId, ChainId, SubAccountId, TokenId, ZkLinkAddress};
use serde::{Deserialize, Serialize};
use validator::Validate;
use zklink_signers::eth_signer::H256;

/// `Mapping` transaction performs a move of funds from one zklink account to another.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FullExit {
    #[validate(custom = "chain_id_validator")]
    pub to_chain_id: ChainId,
    #[validate(custom = "account_validator")]
    pub account_id: AccountId,
    #[validate(custom = "sub_account_validator")]
    pub sub_account_id: SubAccountId,
    #[validate(custom = "zklink_address_validator")]
    pub exit_address: ZkLinkAddress,
    /// Source token and target token of withdrawal from l2 to l1.
    #[validate(custom = "token_validator")]
    pub l2_source_token: TokenId,
    #[validate(custom = "token_validator")]
    pub l1_target_token: TokenId,
    pub serial_id: u64,
    pub eth_hash: H256,
}

impl FullExit {
    /// Creates transaction from all the required fields.
    ///
    /// While `signature` field is mandatory for new transactions, it may be `None`
    /// in some cases (e.g. when restoring the network state from the L1 contract data).
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        to_chain_id: ChainId,
        account_id: AccountId,
        sub_account_id: SubAccountId,
        exit_address: ZkLinkAddress,
        l2_source_token: TokenId,
        l1_target_token: TokenId,
        serial_id: u64,
        eth_hash: H256,
    ) -> Self {
        Self {
            to_chain_id,
            account_id,
            sub_account_id,
            exit_address,
            l2_source_token,
            l1_target_token,
            serial_id,
            eth_hash,
        }
    }

    /// Be used to compute hashes to facilitate the frontend to track priority transactions.
    pub fn get_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.serial_id.to_be_bytes());
        out.extend_from_slice(self.eth_hash.as_bytes());
        out
    }

    #[cfg(feature = "ffi")]
    pub fn json_str(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn is_validate(&self) -> bool {
        self.validate().is_ok()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_get_bytes() {
        let address =
            ZkLinkAddress::from_str("0xAFAFf3aD1a0425D792432D9eCD1c3e26Ef2C42E9").unwrap();
        let eth_hash =
            H256::from_str("0xe35f3a39d542f6d276c2f203e8fd64fcb8bf5db062b71ccacf45d5ecd9d456f3")
                .unwrap();
        let full_exit = FullExit::new(
            ChainId(1),
            AccountId(10),
            SubAccountId(1),
            address,
            TokenId(18),
            TokenId(18),
            100,
            eth_hash,
        );
        let bytes = full_exit.get_bytes();
        let excepted_bytes = [
            0, 0, 0, 0, 0, 0, 0, 100, 227, 95, 58, 57, 213, 66, 246, 210, 118, 194, 242, 3, 232,
            253, 100, 252, 184, 191, 93, 176, 98, 183, 28, 202, 207, 69, 213, 236, 217, 212, 86,
            243,
        ];
        assert_eq!(bytes, excepted_bytes);
    }
}
