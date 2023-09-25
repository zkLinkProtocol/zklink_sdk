use super::validator::*;
use crate::basic_types::{AccountId, ChainId, SubAccountId, TokenId, ZkLinkAddress};
use crate::tx_builder::FullExitBuilder;
use crate::tx_type::TxTrait;
use serde::{Deserialize, Serialize};
use validator::Validate;
use zklink_sdk_signers::eth_signer::H256;

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
    pub fn new(builder: FullExitBuilder) -> Self {
        Self {
            to_chain_id: builder.to_chain_id,
            account_id: builder.account_id,
            sub_account_id: builder.sub_account_id,
            exit_address: builder.exit_address,
            l2_source_token: builder.l2_source_token,
            l1_target_token: builder.l1_target_token,
            serial_id: builder.serial_id,
            eth_hash: builder.eth_hash,
        }
    }
}

impl TxTrait for FullExit {
    fn get_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.serial_id.to_be_bytes());
        out.extend_from_slice(self.eth_hash.as_bytes());
        out
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
        let builder = FullExitBuilder {
            to_chain_id: ChainId(1),
            account_id: AccountId(10),
            sub_account_id: SubAccountId(1),
            exit_address: address,
            l2_source_token: TokenId(18),
            l1_target_token: TokenId(18),
            serial_id: 100,
            eth_hash,
        };
        let full_exit = FullExit::new(builder);
        let bytes = full_exit.get_bytes();
        let excepted_bytes = [
            0, 0, 0, 0, 0, 0, 0, 100, 227, 95, 58, 57, 213, 66, 246, 210, 118, 194, 242, 3, 232,
            253, 100, 252, 184, 191, 93, 176, 98, 183, 28, 202, 207, 69, 213, 236, 217, 212, 86,
            243,
        ];
        assert_eq!(bytes, excepted_bytes);
    }
}
