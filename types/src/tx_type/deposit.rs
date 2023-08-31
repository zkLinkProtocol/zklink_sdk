use super::validator::*;
use crate::basic_types::{ChainId, SubAccountId, TokenId, ZkLinkAddress};
use ethers::types::H256;
use num::BigUint;
use serde::{Deserialize, Serialize};
use validator::Validate;
use zklink_sdk_utils::serde::BigUintSerdeAsRadix10Str;

/// `Mapping` transaction performs a move of funds from one zklink account to another.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Deposit {
    /// The source chain ID of the transaction.
    #[validate(custom = "chain_id_validator")]
    pub from_chain_id: ChainId,
    /// Layer1 address of the transaction initiator's L1 account.
    pub from: ZkLinkAddress,
    /// The target sub-account id of depositing amount.
    #[validate(custom = "sub_account_validator")]
    pub sub_account_id: SubAccountId,
    /// Source token and target token of deposited from l1 to l2.
    #[validate(custom = "token_validator")]
    pub l1_source_token: TokenId,
    #[validate(custom = "token_validator")]
    pub l2_target_token: TokenId,
    /// Amount of tokens deposited.
    #[serde(with = "BigUintSerdeAsRadix10Str")]
    #[validate(custom = "amount_unpackable")]
    pub amount: BigUint,
    /// Layer1 address of L2 account to deposit funds to.
    #[validate(custom = "zklink_address_validator")]
    pub to: ZkLinkAddress,
    /// serial id for unique tx_hash
    pub serial_id: u64,
    pub eth_hash: H256,
}

impl Deposit {
    /// Creates transaction from all the required fields.
    ///
    /// While `signature` field is mandatory for new transactions, it may be `None`
    /// in some cases (e.g. when restoring the network state from the L1 contract data).
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        from_chain_id: ChainId,
        from: ZkLinkAddress,
        sub_account_id: SubAccountId,
        to: ZkLinkAddress,
        l2_target_token: TokenId,
        l1_source_token: TokenId,
        amount: BigUint,
        serial_id: u64,
        eth_hash: H256,
    ) -> Self {
        Self {
            from_chain_id,
            from,
            sub_account_id,
            l2_target_token,
            l1_source_token,
            amount,
            to,
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

    pub fn is_validate(&self) -> bool {
        self.validate().is_ok()
    }
}
