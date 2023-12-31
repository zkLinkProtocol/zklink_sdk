use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use zklink_sdk_types::basic_types::pack::{
    closest_packable_fee_amount, closest_packable_token_amount,
};
use zklink_sdk_types::basic_types::BigUint;
use zklink_sdk_types::error::TypeError;

#[wasm_bindgen(js_name=closestPackableTransactionAmount)]
pub fn closest_packable_transaction_amount(amount: &str) -> Result<String, JsValue> {
    let amount =
        BigUint::from_str(amount).map_err(|e| TypeError::InvalidBigIntStr(e.to_string()))?;
    let packable_amount = closest_packable_token_amount(&amount);
    Ok(packable_amount.to_string())
}

#[wasm_bindgen(js_name=closestPackableTransactionFee)]
pub fn closest_packable_transaction_fee(fee: &str) -> Result<String, JsValue> {
    let fee = BigUint::from_str(fee).map_err(|e| TypeError::InvalidBigIntStr(e.to_string()))?;
    let packable_fee = closest_packable_fee_amount(&fee);
    Ok(packable_fee.to_string())
}
