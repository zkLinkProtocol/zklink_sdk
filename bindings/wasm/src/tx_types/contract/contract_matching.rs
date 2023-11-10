use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use zklink_sdk_types::basic_types::BigUint;
use zklink_sdk_types::tx_builder::{
    ContractBuilder as TxContractBuilder, ContractMatchingBuilder as TxContractMatchingBuilder,
};
use zklink_sdk_types::tx_type::contract::{
    Contract as InnerContract, ContractMatching as ContractMatchingTx,
};

#[wasm_bindgen]
pub struct ContractMatching {
    inner: ContractMatchingTx,
}

#[wasm_bindgen]
impl ContractMatching {
    #[wasm_bindgen(js_name=jsValue)]
    pub fn json_value(&self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.inner)?)
    }
}

#[wasm_bindgen]
pub struct ContractMatchingBuilder {
    inner: TxContractMatchingBuilder,
}

#[wasm_bindgen]
impl ContractMatchingBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new(
        account_id: u32,
        sub_account_id: u8,
        taker: JsValue,
        maker: Vec<JsValue>,
        fee: String,
        fee_token: u16,
    ) -> Result<ContractMatchingBuilder, JsValue> {
        let taker = serde_wasm_bindgen::from_value(taker).unwrap();
        let maker = maker
            .iter()
            .map(|p| serde_wasm_bindgen::from_value(p.clone()).unwrap())
            .collect::<Vec<InnerContract>>();
        let inner = TxContractMatchingBuilder {
            account_id: account_id.into(),
            sub_account_id: sub_account_id.into(),
            taker,
            maker,
            fee: BigUint::from_str(&fee).unwrap(),
            fee_token: fee_token.into(),
        };
        Ok(ContractMatchingBuilder { inner })
    }

    #[wasm_bindgen]
    pub fn build(self) -> ContractMatching {
        ContractMatching {
            inner: self.inner.build(),
        }
    }
}

#[wasm_bindgen(js_name=newContractMatching)]
pub fn new_contract_matching(builder: ContractMatchingBuilder) -> ContractMatching {
    builder.build()
}

#[wasm_bindgen]
pub struct Contract {
    inner: InnerContract,
}

#[wasm_bindgen]
impl Contract {
    #[wasm_bindgen(js_name=jsValue)]
    pub fn json_value(&self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.inner)?)
    }
}

#[wasm_bindgen]
pub struct ContractBuilder {
    inner: TxContractBuilder,
}

#[wasm_bindgen]
impl ContractBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new(
        account_id: u32,
        sub_account_id: u8,
        slot_id: u32,
        nonce: u32,
        pair_id: u16,
        size: String,
        price: String,
        direction: bool,
        maker_fee_rate: u8,
        taker_fee_rate: u8,
        has_subsidy: bool,
    ) -> Result<ContractBuilder, JsValue> {
        let inner = TxContractBuilder {
            account_id: account_id.into(),
            sub_account_id: sub_account_id.into(),
            slot_id: slot_id.into(),
            nonce: nonce.into(),
            pair_id: pair_id.into(),
            size: BigUint::from_str(&size).unwrap(),
            price: BigUint::from_str(&price).unwrap(),
            direction,
            maker_fee_rate,
            taker_fee_rate,
            has_subsidy,
        };
        Ok(ContractBuilder { inner })
    }

    #[wasm_bindgen]
    pub fn build(self) -> Contract {
        Contract {
            inner: self.inner.build(),
        }
    }
}

#[wasm_bindgen(js_name=newContract)]
pub fn new_contract(builder: ContractBuilder) -> Contract {
    builder.build()
}
