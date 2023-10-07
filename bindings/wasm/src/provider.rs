use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use zklink_provider::network::Network;
use std::time::Duration;
use std::collections::HashMap;
use zklink_sdk_types::basic_types::{TokenId, BlockNumber, SubAccountId};
use zklink_provider::response::{TokenResp, AccountSnapshotResp};
use zklink_provider::rpc_wasm::WasmRpcClient;
use crate::tx_types::change_pubkey::ChangePubKey;
use crate::tx_types::transfer::Transfer;
use crate::rpc::{AccountQueryParam, TxL1Signature, SignedTransaction};
use zklink_sdk_signers::zklink_signer::ZkLinkSignature;
use zklink_sdk_types::signatures::TxLayer1Signature;
use zklink_sdk_types::basic_types::tx_hash::TxHash;

#[wasm_bindgen]
pub struct Provider {
    client: WasmRpcClient,
}

#[wasm_bindgen]
impl Provider {
    #[wasm_bindgen(constructor)]
    pub fn new(network: &str) -> Provider{
        Provider {
            client: WasmRpcClient {
                server_url: Network::from_str(network).url().to_owned()
            }
        }
    }

    #[wasm_bindgen]
    pub async fn tokens(&self) -> Result<JsValue,JsValue> {
        let result: HashMap<TokenId, TokenResp> = self.client.tokens().await?;
        Ok(serde_wasm_bindgen::to_value(&result)?)
    }

    #[wasm_bindgen(js_name=accountQuery)]
    pub async fn account_query(&self,
                               account_query:AccountQueryParam,
                               sub_account_id: Option<u8>,
                               block_number:Option<u32>) ->Result<JsValue,JsValue> {
        let result:AccountSnapshotResp = self.client.account_query(
            account_query.into(),
            sub_account_id.map(|id| SubAccountId(id)),
            block_number.map(|number| BlockNumber(number))
        ).await?;
        Ok(serde_wasm_bindgen::to_value(&result)?)
    }

    #[wasm_bindgen(js_name=sendTransaction)]
    pub async fn send_transaction(&self,
                                  tx: SignedTransaction,
                                  l1_signature:Option<TxL1Signature>,
                                  l2_signature: Option<String>) -> Result<String,JsValue>{
        let result:TxHash = self.client.send_transaction(
            tx.into(),
            l1_signature.map(|t| t.into()),
            l2_signature.map(|s| ZkLinkSignature::from_hex(&s).unwrap())
        ).await?;
        Ok(result.as_hex())
    }
}