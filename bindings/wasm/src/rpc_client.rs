use crate::rpc_type_converter::{AccountQuery, TxLayer1Signature, TxZkLinkSignature};
use getrandom::getrandom;
use jsonrpsee::core::params::ArrayParams;
use jsonrpsee::core::traits::ToRpcParams;
use jsonrpsee::types::request::Request;
use jsonrpsee::types::Id;
use std::convert::TryFrom;
use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use zklink_sdk_provider::error::RpcError;
use zklink_sdk_provider::network::Network;
use zklink_sdk_provider::response::AccountQuery as RpcAccountQuery;
use zklink_sdk_provider::web_socket::ws_message::message::client_msg::ClientOffset;
use zklink_sdk_provider::web_socket::ws_message::topic::Topic;
use zklink_sdk_signers::zklink_signer::ZkLinkSignature;
use zklink_sdk_types::basic_types::tx_hash::TxHash;
use zklink_sdk_types::basic_types::{AccountId, BlockNumber, SubAccountId, TokenId};
use zklink_sdk_types::prelude::ZkLinkAddress;
use zklink_sdk_types::signatures::TxLayer1Signature as TypesTxLayer1Signature;
use zklink_sdk_types::tx_type::zklink_tx::ZkLinkTx;
use zklink_sdk_types::tx_type::zklink_tx::ZkLinkTxType;

macro_rules! rpc_request {
    ($method:expr,$builder:expr, $server_url:expr, $resp_type: ty) => {{
        let params = $builder
            .to_rpc_params()
            .map_err(RpcError::InvalidArgument)?;
        let request = Request::new(
            $method.into(),
            params.as_ref().map(|p| p.as_ref()),
            Id::Str(uuid_str().into()),
        );
        let res = reqwest::Client::new()
            .post($server_url)
            .json(&request)
            .send()
            .await
            .map_err(RpcError::RequestError)?
            .json::<serde_json::Value>()
            .await
            .map_err(RpcError::ResponseError)?;
        Ok(serde_wasm_bindgen::to_value(&res.to_string())?)
    }};
}

pub fn uuid_str() -> String {
    let mut bytes = [0; 16];
    getrandom(&mut bytes).expect("RNG failure!");

    let uuid = uuid::Builder::from_bytes(bytes)
        .set_variant(uuid::Variant::RFC4122)
        .set_version(uuid::Version::Random)
        .build();

    uuid.to_string()
}

#[wasm_bindgen]
pub struct RpcClient {
    server_url: String,
}

#[wasm_bindgen]
impl RpcClient {
    #[wasm_bindgen(constructor)]
    pub fn new(network: &str, custom_url: Option<String>) -> Result<RpcClient, JsValue> {
        let server_url = if let Ok(network) = Network::from_str(network) {
            network.url().to_owned()
        } else {
            custom_url.ok_or(RpcError::InvalidNetwork)?
        };
        Ok(RpcClient { server_url })
    }

    #[wasm_bindgen(js_name=getSupportTokens)]
    pub async fn tokens(&self) -> Result<JsValue, JsValue> {
        let builder = ArrayParams::new();
        rpc_request!("getSupportTokens",builder,&self.server_url,HashMap<TokenId, TokenResp>)
    }

    #[wasm_bindgen(js_name=getAccountSnapshot)]
    pub async fn account_query(
        &self,
        account_query: AccountQuery,
        sub_account_id: Option<u8>,
        block_number: Option<u32>,
    ) -> Result<JsValue, JsValue> {
        let mut builder = ArrayParams::new();
        let _ = builder.insert(RpcAccountQuery::from(account_query));
        let _ = builder.insert(sub_account_id.map(|id| SubAccountId(id)));
        let _ = builder.insert(block_number.map(|number| BlockNumber(number)));
        rpc_request!(
            "getAccountSnapshot",
            builder,
            &self.server_url,
            AccountSnapshotResp
        )
    }

    #[wasm_bindgen(js_name=sendTransaction)]
    pub async fn send_transaction(
        &self,
        tx: JsValue,
        l1_signature: Option<TxLayer1Signature>,
        l2_signature: Option<TxZkLinkSignature>,
    ) -> Result<JsValue, JsValue> {
        let mut builder = ArrayParams::new();
        let zklink_tx: ZkLinkTx =
            serde_wasm_bindgen::from_value(tx).map_err(|_e| RpcError::InvalidInputParameter)?;
        let l1_signature = if let Some(s) = l1_signature {
            Some(TypesTxLayer1Signature::try_from(s)?)
        } else {
            None
        };
        let _ = builder.insert(zklink_tx);
        let _ = builder.insert(l1_signature);
        let _ = builder.insert(l2_signature.map(|s| ZkLinkSignature::from(s)));
        rpc_request!("sendTransaction", builder, &self.server_url, TxHash)
    }

    #[wasm_bindgen(js_name=getSupportChains)]
    pub async fn get_support_chains(&self) -> Result<JsValue, JsValue> {
        let builder = ArrayParams::new();
        rpc_request!(
            "getSupportChains",
            builder,
            &self.server_url,
            Vec<ChainResp>
        )
    }

    #[wasm_bindgen(js_name=getLatestBlockNumber)]
    pub async fn block_info(&self) -> Result<JsValue, JsValue> {
        let builder = ArrayParams::new();
        rpc_request!(
            "getLatestBlockNumber",
            builder,
            &self.server_url,
            BlockNumberResp
        )
    }

    #[wasm_bindgen(js_name=getBlockByNumber)]
    pub async fn block_detail(
        &self,
        block_number: Option<u32>,
        include_tx: bool,
        include_update: bool,
    ) -> Result<JsValue, JsValue> {
        let mut builder = ArrayParams::new();
        let _ = builder.insert(block_number.map(|b| BlockNumber(b)));
        let _ = builder.insert(include_tx);
        let _ = builder.insert(include_update);
        rpc_request!("getBlockByNumber", builder, &self.server_url, BlockResp)
    }

    #[wasm_bindgen(js_name=getPendingBlock)]
    pub async fn pending_block_detail(
        &self,
        last_tx_timestamp_micro: u64,
        include_tx: bool,
        include_update: bool,
        limit: Option<usize>,
    ) -> Result<JsValue, JsValue> {
        let mut builder = ArrayParams::new();
        let _ = builder.insert(last_tx_timestamp_micro);
        let _ = builder.insert(include_tx);
        let _ = builder.insert(include_update);
        let _ = builder.insert(limit);
        rpc_request!(
            "getPendingBlock",
            builder,
            &self.server_url,
            Vec<TxHashOrDetailResp>
        )
    }

    #[wasm_bindgen(js_name=getBlockOnChainByNumber)]
    pub async fn block_onchain_detail(&self, block_number: u32) -> Result<JsValue, JsValue> {
        let mut builder = ArrayParams::new();
        let _ = builder.insert(BlockNumber(block_number));
        rpc_request!(
            "getBlockOnChainByNumber",
            builder,
            &self.server_url,
            BlockOnChainResp
        )
    }

    #[wasm_bindgen(js_name=getAccount)]
    pub async fn account_info(&self, account_query: AccountQuery) -> Result<JsValue, JsValue> {
        let mut builder = ArrayParams::new();
        let _ = builder.insert(RpcAccountQuery::from(account_query));
        rpc_request!("getAccount", builder, &self.server_url, AccountInfoResp)
    }

    #[wasm_bindgen(js_name=getAccountBalances)]
    pub async fn account_balances(
        &self,
        account_id: u32,
        sub_account_id: Option<u8>,
    ) -> Result<JsValue, JsValue> {
        let mut builder = ArrayParams::new();
        let _ = builder.insert(AccountId(account_id));
        let _ = builder.insert(sub_account_id.map(|id| SubAccountId(id)));
        rpc_request!(
            "getAccountBalances",
            builder,
            &self.server_url,
            SubAccountBalances
        )
    }

    #[wasm_bindgen(js_name=getAccountOrderSlots)]
    pub async fn account_order_slots(
        &self,
        account_id: u32,
        sub_account_id: Option<u8>,
    ) -> Result<JsValue, JsValue> {
        let mut builder = ArrayParams::new();
        let _ = builder.insert(AccountId(account_id));
        let _ = builder.insert(sub_account_id.map(|id| SubAccountId(id)));
        rpc_request!(
            "getAccountOrderSlots",
            builder,
            &self.server_url,
            SubAccountOrders
        )
    }

    #[wasm_bindgen(js_name=getTokenReserve)]
    pub async fn token_remain(&self, token_id: u32, mapping: bool) -> Result<JsValue, JsValue> {
        let mut builder = ArrayParams::new();
        let _ = builder.insert(TokenId(token_id));
        let _ = builder.insert(mapping);
        rpc_request!("getTokenReserve",builder,&self.server_url,HashMap<ChainId, BigUintSerdeWrapper>)
    }

    #[wasm_bindgen(js_name=getTransactionByHash)]
    pub async fn tx_info(&self, hash: String, include_update: bool) -> Result<JsValue, JsValue> {
        let hash = TxHash::from_hex(&hash).map_err(|_e| RpcError::InvalidInputParameter)?;
        let mut builder = ArrayParams::new();
        let _ = builder.insert(hash);
        let _ = builder.insert(include_update);
        rpc_request!("getTransactionByHash", builder, &self.server_url, TxResp)
    }

    #[wasm_bindgen(js_name=getAccountTransactionHistory)]
    pub async fn tx_history(
        &self,
        tx_type: ZkLinkTxType,
        address: String,
        page_index: u64,
        page_size: u32,
    ) -> Result<JsValue, JsValue> {
        let address =
            ZkLinkAddress::from_hex(&address).map_err(|_e| RpcError::InvalidInputParameter)?;
        let mut builder = ArrayParams::new();
        let _ = builder.insert(tx_type);
        let _ = builder.insert(address);
        let _ = builder.insert(page_index);
        let _ = builder.insert(page_size);
        rpc_request!(
            "getAccountTransactionHistory",
            builder,
            &self.server_url,
            Page<ZkLinkTxHistory>
        )
    }

    #[wasm_bindgen(js_name=getFastWithdrawTxs)]
    pub async fn tx_fast_withdraw(
        &self,
        last_tx_timestamp: u64,
        max_txs: u32,
    ) -> Result<JsValue, JsValue> {
        let mut builder = ArrayParams::new();
        let _ = builder.insert(last_tx_timestamp);
        let _ = builder.insert(max_txs);
        rpc_request!(
            "getFastWithdrawTxs",
            builder,
            &self.server_url,
            Vec<FastWithdrawTxResp>
        )
    }

    #[wasm_bindgen(js_name=pullForwardTxs)]
    pub async fn pull_forward_txs(
        &self,
        sub_account_id: u8,
        offset_id: i64,
        limit: i64,
    ) -> Result<JsValue, JsValue> {
        let mut builder = ArrayParams::new();
        let _ = builder.insert(SubAccountId(sub_account_id));
        let _ = builder.insert(offset_id);
        let _ = builder.insert(limit);
        rpc_request!(
            "pullForwardTxs",
            builder,
            &self.server_url,
            Vec<ForwardTxResp>
        )
    }

    #[wasm_bindgen(js_name=confirmFullExit)]
    pub async fn confirm_full_exit(
        &self,
        tx_hash: String,
        submitter_signature: String,
    ) -> Result<JsValue, JsValue> {
        let hash = TxHash::from_hex(&tx_hash).map_err(|_e| RpcError::InvalidInputParameter)?;
        let mut builder = ArrayParams::new();
        let _ = builder.insert(hash);
        let _ = builder.insert(submitter_signature);
        rpc_request!("confirmFullExit", builder, &self.server_url, bool)
    }

    #[wasm_bindgen(js_name=getWebSocketEvents)]
    pub async fn get_websocket_events(
        &self,
        topic: String,
        from_topic_index_included: f64,
        limit: Option<usize>,
    ) -> Result<JsValue, JsValue> {
        let topic = Topic::from_str(&topic).map_err(|_e| RpcError::InvalidInputParameter)?;
        let client_offset = ClientOffset {
            from_topic_index_included: from_topic_index_included as i64,
            limit,
        };
        let mut builder = ArrayParams::new();
        let _ = builder.insert(topic);
        let _ = builder.insert(client_offset);
        rpc_request!("getWebSocketEvents", builder, &self.server_url, bool)
    }
}
