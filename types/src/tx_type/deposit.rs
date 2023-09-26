use super::validator::*;
use crate::basic_types::{ChainId, SubAccountId, TokenId, ZkLinkAddress};
use crate::tx_builder::DepositBuilder;
use crate::tx_type::TxTrait;
use num::BigUint;
use serde::{Deserialize, Serialize};
use validator::Validate;
use zklink_sdk_signers::eth_signer::H256;
use zklink_sdk_utils::serde::BigUintSerdeAsRadix10Str;

/// `Mapping` transaction performs a move of funds from one zklink account to another.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Deposit {
    /// Layer1 address of the transaction initiator's L1 account.
    pub from: ZkLinkAddress,
    /// Layer1 address of L2 account to deposit funds to.
    #[validate(custom = "zklink_address_validator")]
    pub to: ZkLinkAddress,
    /// The source chain ID of the transaction.
    #[validate(custom = "chain_id_validator")]
    pub from_chain_id: ChainId,
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
    /// serial id for unique tx_hash
    pub serial_id: u64,
    pub eth_hash: H256,
}

impl Deposit {
    /// Creates transaction from all the required fields.
    ///
    /// While `signature` field is mandatory for new transactions, it may be `None`
    /// in some cases (e.g. when restoring the network state from the L1 contract data).
    pub fn new(builder: DepositBuilder) -> Self {
        Self {
            from: builder.from_address,
            to: builder.to_address,
            from_chain_id: builder.from_chain_id,
            sub_account_id: builder.sub_account_id,
            l2_target_token: builder.l2_target_token,
            l1_source_token: builder.l1_source_token,
            amount: builder.amount,
            serial_id: builder.serial_id,
            eth_hash: builder.eth_hash,
        }
    }
}

impl TxTrait for Deposit {
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
        let builder = DepositBuilder {
            from_address: address.clone(),
            to_address: address.clone(),
            from_chain_id: ChainId(1),
            sub_account_id: SubAccountId(1),
            l2_target_token: TokenId(18),
            l1_source_token: TokenId(18),
            amount: BigUint::from(100u32),
            serial_id: 32001,
            eth_hash,
        };
        let deposit = Deposit::new(builder);
        let bytes = deposit.get_bytes();
        let excepted_bytes = [
            0, 0, 0, 0, 0, 0, 125, 1, 227, 95, 58, 57, 213, 66, 246, 210, 118, 194, 242, 3, 232,
            253, 100, 252, 184, 191, 93, 176, 98, 183, 28, 202, 207, 69, 213, 236, 217, 212, 86,
            243,
        ];
        assert_eq!(bytes, excepted_bytes);
    }
}
