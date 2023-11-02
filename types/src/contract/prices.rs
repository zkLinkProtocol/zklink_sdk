use crate::basic_types::pad::pad_front;
use crate::basic_types::{GetBytes, PairId, TokenId};
use crate::params::{
    CONTRACT_PRICE_BYTES, MARGIN_PRICE_BYTES, PRICE_BIT_WIDTH,
};
use crate::prelude::validator::*;
use num::BigUint;
use serde::{Deserialize, Serialize};
use validator::Validate;
use zklink_sdk_utils::serde::BigUintSerdeAsRadix10Str;

/// The current contract price, which is used to handle Liquidation and ADL
#[derive(Default, Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ContractPrice {
    #[validate(custom = "pair_validator")]
    pub pair_id: PairId,
    #[serde(with = "BigUintSerdeAsRadix10Str")]
    #[validate(custom = "external_price_validator")]
    pub market_price: BigUint,
}

impl GetBytes for ContractPrice {
    fn get_bytes(&self) -> Vec<u8> {
        let bytes_len = self.bytes_len();
        let mut contracts_encode = Vec::with_capacity(bytes_len);
        contracts_encode.extend((*self.pair_id as u8).to_be_bytes());
        contracts_encode.extend(pad_front(
            &self.market_price.to_bytes_be(),
            PRICE_BIT_WIDTH / 8,
        ));
        assert_eq!(contracts_encode.len(), bytes_len);
        contracts_encode
    }

    fn bytes_len(&self) -> usize {
        CONTRACT_PRICE_BYTES
    }
}

/// The current margin token price, used to handle Liquidation and ADL
#[derive(Default, Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SpotPriceInfo {
    #[validate(custom = "token_validator")]
    pub token_id: TokenId,
    #[serde(with = "BigUintSerdeAsRadix10Str")]
    #[validate(custom = "external_price_validator")]
    pub price: BigUint,
}

impl GetBytes for SpotPriceInfo {
    fn get_bytes(&self) -> Vec<u8> {
        let bytes_len = self.bytes_len();
        let mut margins_encode = Vec::with_capacity(bytes_len);
        margins_encode.extend((*self.token_id as u16).to_be_bytes());
        margins_encode.extend(pad_front(&self.price.to_bytes_be(), PRICE_BIT_WIDTH / 8));
        assert_eq!(margins_encode.len(), bytes_len);
        margins_encode
    }

    fn bytes_len(&self) -> usize {
        MARGIN_PRICE_BYTES
    }
}

/// The current margin token price, used to handle Liquidation and ADL
#[derive(Default, Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OraclePrices {
    /// The current prices of all contracts
    #[validate]
    #[validate(custom = "contract_prices_validator")]
    pub contract_prices: Vec<ContractPrice>,
    /// The current prices of all margin tokens
    #[validate]
    #[validate(custom = "margin_prices_validator")]
    pub margin_prices: Vec<SpotPriceInfo>,
}

impl OraclePrices {
    pub fn get_contracts_price(&self, pair_id: PairId) -> &BigUint {
        &self.contract_prices[*pair_id as usize].market_price
    }

    pub fn get_spot_price(&self, token_id: TokenId) -> Option<&BigUint> {
        self.margin_prices
            .iter()
            .find(|info| info.token_id == token_id)
            .map(|info| &info.price)
    }
}

impl GetBytes for OraclePrices {
    fn get_bytes(&self) -> Vec<u8> {
        let bytes_len = self.bytes_len();
        let mut margins_encode = Vec::with_capacity(bytes_len);
        margins_encode.extend(self.contract_prices.get_bytes());
        margins_encode.extend(self.margin_prices.get_bytes());
        assert_eq!(margins_encode.len(), bytes_len);
        margins_encode
    }

    fn bytes_len(&self) -> usize {
        self.margin_prices.bytes_len() + self.contract_prices.bytes_len()
    }
}
