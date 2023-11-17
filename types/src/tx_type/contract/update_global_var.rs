use super::funding::FundingInfo;
use crate::basic_types::{AccountId, ChainId, GetBytes, MarginId, PairId, SubAccountId, TokenId};
use crate::params::PAIR_SYMBOL_BYTES;
use crate::prelude::validator::*;
#[cfg(feature = "ffi")]
use crate::tx_builder::UpdateGlobalVarBuilder;
use crate::tx_type::TxTrait;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateGlobalVar {
    /// The chain from which the op is sent down to ZkLink layer2
    #[validate(custom = "chain_id_validator")]
    pub from_chain_id: ChainId,
    #[validate(custom = "sub_account_validator")]
    pub sub_account_id: SubAccountId,
    #[validate(custom = "parameter_validator")]
    pub parameter: Parameter,
    pub serial_id: u64,
}

impl UpdateGlobalVar {
    #[cfg(feature = "ffi")]
    pub fn new(builder: UpdateGlobalVarBuilder) -> Self {
        builder.build()
    }
}

impl TxTrait for UpdateGlobalVar {}

impl GetBytes for UpdateGlobalVar {
    fn get_bytes(&self) -> Vec<u8> {
        let bytes_len = self.bytes_len();
        let param_bytes = self.parameter.get_bytes();
        let mut out = Vec::with_capacity(bytes_len);
        out.push(Self::TX_TYPE);
        out.push(*self.from_chain_id);
        out.push(*self.sub_account_id);
        out.extend(param_bytes);
        out.extend(self.serial_id.to_be_bytes());
        out
    }

    fn bytes_len(&self) -> usize {
        let param_bytes = self.parameter.get_bytes();
        11 + param_bytes.len()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Parameter {
    /// modify the collect-fee account
    #[serde(rename_all = "camelCase")]
    FeeAccount { account_id: AccountId },
    /// modify the insurance fund account
    #[serde(rename_all = "camelCase")]
    InsuranceFundAccount { account_id: AccountId },
    /// modify the margin info in the specified index.
    #[serde(rename_all = "camelCase")]
    MarginInfo {
        margin_id: MarginId,
        token_id: TokenId,
        ratio: u8,
    },
    /// update the funding rates to accumulated funding rates of the Global Vars for all position(contract pair) in this period
    #[serde(rename_all = "camelCase")]
    FundingInfos { infos: Vec<FundingInfo> },
    /// modify the info of every contract pair
    #[serde(rename_all = "camelCase")]
    ContractInfo {
        pair_id: PairId,
        symbol: String,
        initial_margin_rate: u16,
        maintenance_margin_rate: u16,
    },
}

impl Parameter {
    pub const FEE_ACCOUNT_PARAM_TYPE: u8 = 0;
    pub const INSURANCE_FUND_PARAM_TYPE: u8 = 1;
    // margin
    pub const MARGIN_INFO_PARAM_TYPE: u8 = 2;
    // contract trading pair
    pub const CONTRACT_INFO_PARAM_TYPE: u8 = 3;
    pub const FUNDING_RATE_PARAM_TYPE: u8 = 4;

    pub const PARAM_TYPE_NUM: usize = 5;

    pub fn parameter_type(&self) -> u8 {
        match self {
            Parameter::FeeAccount { .. } => Self::FEE_ACCOUNT_PARAM_TYPE,
            Parameter::InsuranceFundAccount { .. } => Self::INSURANCE_FUND_PARAM_TYPE,
            Parameter::MarginInfo { .. } => Self::MARGIN_INFO_PARAM_TYPE,
            Parameter::ContractInfo { .. } => Self::CONTRACT_INFO_PARAM_TYPE,
            Parameter::FundingInfos { .. } => Self::FUNDING_RATE_PARAM_TYPE,
        }
    }

    pub fn is_initial_margin_rate(&self) -> bool {
        matches!(self, Parameter::ContractInfo { .. })
    }
}

impl GetBytes for Parameter {
    fn get_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![self.parameter_type()];
        bytes.extend(match self {
            Parameter::FeeAccount { account_id }
            | Parameter::InsuranceFundAccount { account_id } => account_id.get_bytes(),
            Parameter::MarginInfo {
                margin_id,
                token_id,
                ratio,
            } => {
                let mut bytes = vec![**margin_id];
                bytes.extend((**token_id as u16).to_be_bytes());
                bytes.push(*ratio);
                bytes
            }
            Parameter::ContractInfo {
                pair_id,
                symbol,
                initial_margin_rate,
                maintenance_margin_rate,
            } => {
                let mut bytes = vec![(**pair_id as u8)];
                let mut symbol_bytes = symbol.clone().into_bytes();
                symbol_bytes.resize(PAIR_SYMBOL_BYTES, 0);
                bytes.extend(symbol_bytes);
                bytes.extend(initial_margin_rate.to_be_bytes());
                bytes.extend(maintenance_margin_rate.to_be_bytes());
                bytes
            }
            Parameter::FundingInfos { infos } => infos.get_bytes(),
        });
        bytes
    }

    fn bytes_len(&self) -> usize {
        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::UpdateGlobalVarBuilder;

    #[test]
    fn test_get_bytes() {
        let params = vec![
            Parameter::FundingInfos {
                infos: vec![
                    FundingInfo {
                        pair_id: PairId(0),
                        price: 1000_000_000_000_000_000u128.into(),
                        funding_rate: i16::MAX,
                    },
                    FundingInfo {
                        pair_id: PairId(1),
                        price: 1000_000_000_000_000u128.into(),
                        funding_rate: 0,
                    },
                    FundingInfo {
                        pair_id: PairId(2),
                        price: 1000_000_000_000u128.into(),
                        funding_rate: -1,
                    },
                    FundingInfo {
                        pair_id: PairId(3),
                        price: 1000_000_000u128.into(),
                        funding_rate: 1,
                    },
                ],
            },
            Parameter::FeeAccount {
                account_id: 10.into(),
            },
            Parameter::InsuranceFundAccount {
                account_id: 9.into(),
            },
            Parameter::MarginInfo {
                margin_id: 1.into(),
                token_id: 9.into(),
                ratio: 0,
            },
            Parameter::ContractInfo {
                pair_id: 2.into(),
                symbol: "BTCUSDC".to_string(),
                initial_margin_rate: 6,
                maintenance_margin_rate: 8,
            },
        ];
        let excepted_bytes = [
            vec![
                12, 1, 1, 4, 0, 0, 0, 0, 0, 0, 0, 0, 13, 224, 182, 179, 167, 100, 0, 0, 127, 255,
                1, 0, 0, 0, 0, 0, 0, 0, 0, 3, 141, 126, 164, 198, 128, 0, 0, 0, 2, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 232, 212, 165, 16, 0, 128, 1, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                59, 154, 202, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            vec![12, 1, 1, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![12, 1, 1, 1, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![12, 1, 1, 2, 1, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];
        for (param, excepted_bytes) in params.into_iter().zip(excepted_bytes) {
            let builder = UpdateGlobalVarBuilder {
                from_chain_id: ChainId(1),
                sub_account_id: SubAccountId(1),
                parameter: param,
                serial_id: 0,
            };
            let tx = builder.build();
            let bytes = tx.get_bytes();
            assert_eq!(bytes, excepted_bytes);
        }
    }
}
