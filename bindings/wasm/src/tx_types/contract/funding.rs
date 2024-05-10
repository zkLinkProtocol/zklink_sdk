use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use zklink_sdk_types::basic_types::BigUint;
use zklink_sdk_types::error::TypeError::InvalidBigIntStr;
use zklink_sdk_types::tx_builder::FundingBuilder as TxFundingBuilder;
use zklink_sdk_types::tx_type::contract::{
    Funding as InnerFunding, FundingInfo as InnerFundingInfo,
};

#[wasm_bindgen]
pub struct FundingInfo {
    inner: InnerFundingInfo,
}

#[wasm_bindgen]
impl FundingInfo {
    #[wasm_bindgen(constructor)]
    pub fn new(pair_id: u16, funding_rate: i16, price: String) -> Result<FundingInfo, JsValue> {
        Ok(FundingInfo {
            inner: InnerFundingInfo {
                pair_id: pair_id.into(),
                price: BigUint::from_str(&price).map_err(|e| InvalidBigIntStr(e.to_string()))?,
                funding_rate,
            },
        })
    }

    #[wasm_bindgen(js_name=jsValue)]
    pub fn json_value(&self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.inner)?)
    }
}

#[wasm_bindgen]
pub struct Funding {
    inner: InnerFunding,
}

#[wasm_bindgen]
impl Funding {
    #[wasm_bindgen(js_name=jsValue)]
    pub fn json_value(&self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.inner)?)
    }
}

#[wasm_bindgen]
pub struct FundingBuilder {
    inner: TxFundingBuilder,
}

#[wasm_bindgen]
impl FundingBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new(
        account_id: u32,
        sub_account_id: u8,
        sub_account_nonce: u32,
        funding_account_ids: Vec<u32>,
        fee: String,
        fee_token: u16,
    ) -> Result<FundingBuilder, JsValue> {
        let funding_account_ids = funding_account_ids
            .iter()
            .map(|id| (*id).into())
            .collect::<Vec<_>>();
        let inner = TxFundingBuilder {
            account_id: account_id.into(),
            sub_account_id: sub_account_id.into(),
            sub_account_nonce: sub_account_nonce.into(),
            fee: BigUint::from_str(&fee).map_err(|e| InvalidBigIntStr(e.to_string()))?,
            fee_token: fee_token.into(),
            funding_account_ids,
        };
        Ok(FundingBuilder { inner })
    }

    #[wasm_bindgen]
    pub fn build(self) -> Funding {
        Funding {
            inner: self.inner.build(),
        }
    }
}

#[wasm_bindgen(js_name=newFunding)]
pub fn new_funding(builder: FundingBuilder) -> Funding {
    builder.build()
}
