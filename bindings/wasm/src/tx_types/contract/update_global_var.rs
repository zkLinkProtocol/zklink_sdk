use serde::Deserialize;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use zklink_sdk_types::tx_builder::UpdateGlobalVarBuilder as TxUpdateGlobalVarBuilder;
use zklink_sdk_types::tx_type::contract::{
    FundingRate as InnerFundingRate, Parameter as ContractParameter,
    UpdateGlobalVar as UpdateGlobalVarTx,
};

#[wasm_bindgen]
pub struct UpdateGlobalVar {
    inner: UpdateGlobalVarTx,
}

#[wasm_bindgen]
impl UpdateGlobalVar {
    #[wasm_bindgen(js_name = jsonValue)]
    pub fn json_value(&self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.inner)?)
    }
}

#[wasm_bindgen]
pub struct UpdateGlobalVarBuilder {
    inner: TxUpdateGlobalVarBuilder,
}

#[wasm_bindgen]
impl UpdateGlobalVarBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new(
        from_chain_id: u8,
        sub_account_id: u8,
        parameter: Parameter,
        serial_id: f64,
    ) -> Result<UpdateGlobalVarBuilder, JsValue> {
        let inner = TxUpdateGlobalVarBuilder {
            from_chain_id: from_chain_id.into(),
            sub_account_id: sub_account_id.into(),
            parameter: parameter.into(),
            serial_id: serial_id as u64,
        };
        Ok(UpdateGlobalVarBuilder { inner })
    }

    #[wasm_bindgen]
    pub fn build(self) -> UpdateGlobalVar {
        UpdateGlobalVar {
            inner: self.inner.build(),
        }
    }
}

#[wasm_bindgen(js_name=newUpdateGlobalVar)]
pub fn new_update_global_var(builder: UpdateGlobalVarBuilder) -> UpdateGlobalVar {
    builder.build()
}

#[wasm_bindgen]
pub enum ParameterType {
    FeeAccount,
    InsuranceFundAccount,
    MarginInfo,
    FundingRates,
    InitialMarginRate,
    MaintenanceMarginRate,
}

#[wasm_bindgen]
pub struct Parameter {
    parameter_type: ParameterType,
    parameter_value: JsValue,
}
#[wasm_bindgen]
impl Parameter {
    #[wasm_bindgen(constructor)]
    pub fn new(parameter_type: ParameterType, parameter_value: JsValue) -> Parameter {
        Parameter {
            parameter_type,
            parameter_value,
        }
    }
}

#[wasm_bindgen]
#[derive(Deserialize)]
pub struct MarginInfo {
    margin_id: u8,
    token_id: u32,
    ratio: u8,
}

#[wasm_bindgen]
impl MarginInfo {
    #[wasm_bindgen(constructor)]
    pub fn new(margin_id: u8, token_id: u32, ratio: u8) -> MarginInfo {
        MarginInfo {
            margin_id,
            token_id,
            ratio,
        }
    }
}

#[wasm_bindgen]
#[derive(Deserialize)]
pub struct MarginRate {
    pair_id: u16,
    rate: u16,
}

#[wasm_bindgen]
impl MarginRate {
    #[wasm_bindgen(constructor)]
    pub fn new(pair_id: u16, rate: u16) -> MarginRate {
        MarginRate { pair_id, rate }
    }
}

impl From<Parameter> for ContractParameter {
    fn from(parameter: Parameter) -> ContractParameter {
        match parameter.parameter_type {
            ParameterType::FeeAccount => {
                let value = parameter.parameter_value.as_f64().unwrap() as u32;
                ContractParameter::FeeAccount {
                    fee_account_id: value.into(),
                }
            }
            ParameterType::InsuranceFundAccount => {
                let value = parameter.parameter_value.as_f64().unwrap() as u32;
                ContractParameter::InsuranceFundAccount {
                    insurance_account_id: value.into(),
                }
            }
            ParameterType::MarginInfo => {
                let value: MarginInfo =
                    serde_wasm_bindgen::from_value(parameter.parameter_value).unwrap();
                ContractParameter::MarginInfo {
                    margin_id: value.margin_id.into(),
                    token_id: value.token_id.into(),
                    ratio: value.ratio,
                }
            }
            ParameterType::FundingRates => {
                let value: Vec<InnerFundingRate> =
                    serde_wasm_bindgen::from_value(parameter.parameter_value).unwrap();
                ContractParameter::FundingRates {
                    funding_rates: value,
                }
            }
            ParameterType::InitialMarginRate => {
                let value: MarginRate =
                    serde_wasm_bindgen::from_value(parameter.parameter_value).unwrap();
                ContractParameter::InitialMarginRate {
                    pair_id: value.pair_id.into(),
                    rate: value.rate,
                }
            }
            ParameterType::MaintenanceMarginRate => {
                let value: MarginRate =
                    serde_wasm_bindgen::from_value(parameter.parameter_value).unwrap();
                ContractParameter::MaintenanceMarginRate {
                    pair_id: value.pair_id.into(),
                    rate: value.rate,
                }
            }
        }
    }
}
