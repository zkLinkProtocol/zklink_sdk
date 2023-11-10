use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use zklink_sdk_types::basic_types::BigUint;
use zklink_sdk_types::tx_type::contract::{
    ContractPrice as InnerContractPrice, SpotPriceInfo as InnerSpotPriceInfo,
};

#[wasm_bindgen]
pub struct ContractPrice {
    inner: InnerContractPrice,
}

#[wasm_bindgen]
impl ContractPrice {
    #[wasm_bindgen(constructor)]
    pub fn new(pair_id: u16, market_price: String) -> ContractPrice {
        ContractPrice {
            inner: InnerContractPrice {
                pair_id: pair_id.into(),
                market_price: BigUint::from_str(&market_price).unwrap(),
            },
        }
    }

    #[wasm_bindgen(js_name=jsonValue)]
    pub fn json_value(&self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.inner)?)
    }
}
#[wasm_bindgen]
pub struct SpotPriceInfo {
    inner: InnerSpotPriceInfo,
}

#[wasm_bindgen]
impl SpotPriceInfo {
    #[wasm_bindgen(constructor)]
    pub fn new(token_id: u32, price: String) -> SpotPriceInfo {
        SpotPriceInfo {
            inner: InnerSpotPriceInfo {
                token_id: token_id.into(),
                price: BigUint::from_str(&price).unwrap(),
            },
        }
    }

    #[wasm_bindgen(js_name=jsonValue)]
    pub fn json_value(&self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.inner)?)
    }
}

pub struct OraclePrices {}
