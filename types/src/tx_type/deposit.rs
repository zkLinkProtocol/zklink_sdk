use super::validator::*;
use crate::basic_types::{ChainId, SubAccountId, TokenId, ZkLinkAddress};
use num::BigUint;
use serde::{Deserialize, Serialize};
use validator::Validate;
use zklink_sdk_utils::serde::BigUintSerdeAsRadix10Str;
use zklink_signers::eth_signer::H256;

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
        let deposit = Deposit::new(
            ChainId(1),
            address.clone(),
            SubAccountId(1),
            address,
            TokenId(18),
            TokenId(18),
            BigUint::from(100u32),
            32001,
            eth_hash,
        );
        let bytes = deposit.get_bytes();
        let excepted_bytes = [
            0, 0, 0, 0, 0, 0, 125, 1, 227, 95, 58, 57, 213, 66, 246, 210, 118, 194, 242, 3, 232,
            253, 100, 252, 184, 191, 93, 176, 98, 183, 28, 202, 207, 69, 213, 236, 217, 212, 86,
            243,
        ];
        assert_eq!(bytes, excepted_bytes);
    }
}
