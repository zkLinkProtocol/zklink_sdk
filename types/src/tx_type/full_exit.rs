use super::validator::*;
use crate::basic_types::{AccountId, ChainId, GetBytes, SubAccountId, TokenId, ZkLinkAddress};
#[cfg(feature = "ffi")]
use crate::prelude::FullExitBuilder;
use crate::prelude::OraclePrices;
use crate::tx_type::TxTrait;
use serde::{Deserialize, Serialize};
use validator::Validate;
use zklink_sdk_signers::eth_signer::H256;

/// `Mapping` transaction performs a move of funds from one zklink account to another.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Validate, Eq, PartialEq)]
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
    /// Contains required mark prices for all margin tokens and contracts from Oracle(e.g. ChainLink, Band, Api3)
    #[validate]
    pub oracle_prices: OraclePrices,
    pub serial_id: u64,
    /// Transaction hash of linea/zksync/starket etc
    pub l2_hash: H256,
}

impl FullExit {
    #[cfg(feature = "ffi")]
    pub fn new(builder: FullExitBuilder) -> Self {
        builder.build()
    }
}

impl GetBytes for FullExit {
    fn get_bytes(&self) -> Vec<u8> {
        let bytes_len = self.bytes_len();
        let mut out = Vec::with_capacity(bytes_len);
        out.extend(self.serial_id.to_be_bytes());
        out.extend_from_slice(self.l2_hash.as_bytes());
        assert_eq!(out.len(), bytes_len);
        out
    }
    fn bytes_len(&self) -> usize {
        40
    }
}

impl TxTrait for FullExit {}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::FullExitBuilder;
    use std::str::FromStr;

    #[test]
    fn test_full_exit_get_bytes() {
        let address =
            ZkLinkAddress::from_str("0xAFAFf3aD1a0425D792432D9eCD1c3e26Ef2C42E9").unwrap();
        let l2_hash =
            H256::from_str("0xe35f3a39d542f6d276c2f203e8fd64fcb8bf5db062b71ccacf45d5ecd9d456f3")
                .unwrap();
        let default_oracle_price = OraclePrices::default();
        let builder = FullExitBuilder {
            to_chain_id: ChainId(1),
            account_id: AccountId(10),
            sub_account_id: SubAccountId(1),
            exit_address: address,
            l2_source_token: TokenId(18),
            l1_target_token: TokenId(18),
            contract_prices: default_oracle_price.contract_prices,
            margin_prices: default_oracle_price.margin_prices,
            serial_id: 100,
            l2_hash,
        };
        let full_exit = builder.build();
        let bytes = full_exit.get_bytes();
        let excepted_bytes = [
            0, 0, 0, 0, 0, 0, 0, 100, 227, 95, 58, 57, 213, 66, 246, 210, 118, 194, 242, 3, 232,
            253, 100, 252, 184, 191, 93, 176, 98, 183, 28, 202, 207, 69, 213, 236, 217, 212, 86,
            243,
        ];
        assert_eq!(bytes, excepted_bytes);
    }
}
