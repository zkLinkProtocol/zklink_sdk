use super::funding::FundingRate;
use crate::basic_types::{AccountId, ChainId, GetBytes, MarginId, PairId, SubAccountId, TokenId};
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
    FeeAccount { fee_account_id: AccountId },
    /// modify the insurance fund account
    InsuranceFundAccount { insurance_account_id: AccountId },
    /// modify the margin info in the specified index.
    MarginInfo {
        margin_id: MarginId,
        token_id: TokenId,
        ratio: u8,
    },
    /// update the funding rates to accumulated funding rates of the Global Vars for all position(contract pair) in this period
    FundingRates { funding_rates: Vec<FundingRate> },
    /// modify the initial margin rate of every margin
    InitialMarginRate { pair_id: PairId, rate: u16 },
    /// modify the maintenance margin rate of every margin
    MaintenanceMarginRate { pair_id: PairId, rate: u16 },
}

impl Parameter {
    pub const FEE_ACCOUNT_PARAM_TYPE: u8 = 0;
    pub const INSURANCE_FUND_PARAM_TYPE: u8 = 1;
    // margin
    pub const MARGIN_INFO_PARAM_TYPE: u8 = 2;
    // contract trading pair
    pub const INITIAL_MARGIN_RATE_PARAM_TYPE: u8 = 3;
    pub const MAINTENANCE_MARGIN_RATE_PARAM_TYPE: u8 = 4;
    pub const FUNDING_RATE_PARAM_TYPE: u8 = 5;

    pub const PARAM_TYPE_NUM: usize = 6;

    pub fn parameter_type(&self) -> u8 {
        match self {
            Parameter::FeeAccount { .. } => Self::FEE_ACCOUNT_PARAM_TYPE,
            Parameter::InsuranceFundAccount { .. } => Self::INSURANCE_FUND_PARAM_TYPE,
            Parameter::MarginInfo { .. } => Self::MARGIN_INFO_PARAM_TYPE,
            Parameter::InitialMarginRate { .. } => Self::INITIAL_MARGIN_RATE_PARAM_TYPE,
            Parameter::MaintenanceMarginRate { .. } => Self::MAINTENANCE_MARGIN_RATE_PARAM_TYPE,
            Parameter::FundingRates { .. } => Self::FUNDING_RATE_PARAM_TYPE,
        }
    }

    pub fn is_initial_margin_rate(&self) -> bool {
        matches!(self, Parameter::InitialMarginRate { .. })
    }
}

impl GetBytes for Parameter {
    fn get_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![self.parameter_type()];
        bytes.extend(match self {
            Parameter::FeeAccount { fee_account_id } => fee_account_id.to_be_bytes().to_vec(),
            Parameter::InsuranceFundAccount {
                insurance_account_id,
            } => insurance_account_id.get_bytes(),
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
            Parameter::InitialMarginRate { pair_id, rate }
            | Parameter::MaintenanceMarginRate { pair_id, rate } => vec![(**pair_id as u8)]
                .into_iter()
                .chain(rate.to_be_bytes())
                .collect(),
            Parameter::FundingRates { funding_rates } => funding_rates.get_bytes(),
        });
        bytes
    }

    fn bytes_len(&self) -> usize {
        unreachable!()
    }
}
