use super::validator::*;
use crate::basic_types::{AccountId, ChainId, SubAccountId, TokenId, ZkLinkAddress};
use ethers::types::H256;
use serde::{Deserialize, Serialize};
use validator::Validate;

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
        chain_id: u8,
        account_id: AccountId,
        sub_account_id: SubAccountId,
        exit_address: ZkLinkAddress,
        l2_source_token: TokenId,
        l1_target_token: TokenId,
        serial_id: u64,
        eth_hash: H256,
    ) -> Self {
        Self {
            to_chain_id: ChainId(chain_id),
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

    pub fn check_correctness(&self) -> bool {
        self.validate().is_ok()
    }
}
