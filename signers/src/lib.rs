/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
use serde::{Deserialize, Serialize};

pub mod eth_signer;
pub mod starknet_signer;
pub mod zklink_signer;

extern crate js_sys;
extern crate wasm_bindgen_futures;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RpcErr {
    pub code: i32,
    pub message: String,
}

impl std::fmt::Display for RpcErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
