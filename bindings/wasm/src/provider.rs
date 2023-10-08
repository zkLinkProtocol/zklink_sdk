use crate::rpc::{AccountQueryParam, L2TxType, SignedTransaction, TxL1Signature};
use std::collections::HashMap;
use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use zklink_sdk_provider::error::RpcError;
use zklink_sdk_provider::network::Network;
use zklink_sdk_provider::response::{
    AccountInfoResp, AccountSnapshotResp, BlockNumberResp, BlockOnChainResp, BlockResp, ChainResp,
    FastWithdrawTxResp, ForwardTxResp, Page, SubAccountBalances, SubAccountOrders, TokenResp,
    TxHashOrDetailResp, TxResp, ZkLinkTxHistory,
};
use zklink_sdk_provider::rpc_wasm::WasmRpcClient;
use zklink_sdk_signers::zklink_signer::ZkLinkSignature;
use zklink_sdk_types::basic_types::bigunit_wrapper::BigUintSerdeWrapper;
use zklink_sdk_types::basic_types::tx_hash::TxHash;
use zklink_sdk_types::basic_types::{AccountId, BlockNumber, ChainId, SubAccountId, TokenId};
use zklink_sdk_types::prelude::ZkLinkAddress;

#[wasm_bindgen]
pub struct Provider {
    client: WasmRpcClient,
}

#[wasm_bindgen]
impl Provider {
    #[wasm_bindgen(constructor)]
    pub fn new(network: &str) -> Provider {
        Provider {
            client: WasmRpcClient {
                server_url: Network::from_str(network).unwrap().url().to_owned(),
            },
        }
    }

    #[wasm_bindgen(js_name=getSupportTokens)]
    pub async fn tokens(&self) -> Result<JsValue, JsValue> {
        let result: HashMap<TokenId, TokenResp> = self.client.tokens().await?;
        Ok(serde_wasm_bindgen::to_value(&result)?)
    }

    #[wasm_bindgen(js_name=getAccountSnapshot)]
    pub async fn account_query(
        &self,
        account_query: AccountQueryParam,
        sub_account_id: Option<u8>,
        block_number: Option<u32>,
    ) -> Result<JsValue, JsValue> {
        let result: AccountSnapshotResp = self
            .client
            .account_query(
                account_query.into(),
                sub_account_id.map(|id| SubAccountId(id)),
                block_number.map(|number| BlockNumber(number)),
            )
            .await?;
        Ok(serde_wasm_bindgen::to_value(&result)?)
    }

    #[wasm_bindgen(js_name=sendTransaction)]
    pub async fn send_transaction(
        &self,
        tx: SignedTransaction,
        l1_signature: Option<TxL1Signature>,
        l2_signature: Option<String>,
    ) -> Result<String, JsValue> {
        let result: TxHash = self
            .client
            .send_transaction(
                tx.into(),
                l1_signature.map(|t| t.into()),
                l2_signature.map(|s| ZkLinkSignature::from_hex(&s).unwrap()),
            )
            .await?;
        Ok(result.as_hex())
    }

    #[wasm_bindgen(js_name=getSupportChains)]
    pub async fn get_support_chains(&self) -> Result<JsValue, JsValue> {
        let result: Vec<ChainResp> = self.client.get_support_chains().await?;
        Ok(serde_wasm_bindgen::to_value(&result)?)
    }

    #[wasm_bindgen(js_name=getLatestBlockNumber)]
    pub async fn block_info(&self) -> Result<JsValue, JsValue> {
        let result: BlockNumberResp = self.client.block_info().await?;
        Ok(serde_wasm_bindgen::to_value(&result)?)
    }

    #[wasm_bindgen(js_name=getBlockByNumber)]
    pub async fn block_detail(
        &self,
        block_number: Option<u32>,
        include_tx: bool,
        include_update: bool,
    ) -> Result<JsValue, JsValue> {
        let result: BlockResp = self
            .client
            .block_detail(
                block_number.map(|b| BlockNumber(b)),
                include_tx,
                include_update,
            )
            .await?;
        Ok(serde_wasm_bindgen::to_value(&result)?)
    }

    #[wasm_bindgen(js_name=getPendingBlock)]
    pub async fn pending_block_detail(
        &self,
        last_tx_timestamp_micro: u64,
        include_tx: bool,
        include_update: bool,
        limit: Option<usize>,
    ) -> Result<JsValue, JsValue> {
        let result: Vec<TxHashOrDetailResp> = self
            .client
            .pending_block_detail(last_tx_timestamp_micro, include_tx, include_update, limit)
            .await?;
        Ok(serde_wasm_bindgen::to_value(&result)?)
    }

    #[wasm_bindgen(js_name=getBlockOnChainByNumber)]
    pub async fn block_onchain_detail(&self, block_number: u32) -> Result<JsValue, JsValue> {
        let result: BlockOnChainResp = self
            .client
            .block_onchain_detail(BlockNumber(block_number))
            .await?;
        Ok(serde_wasm_bindgen::to_value(&result)?)
    }

    #[wasm_bindgen(js_name=getAccount)]
    pub async fn account_info(&self, account_query: AccountQueryParam) -> Result<JsValue, JsValue> {
        let result: AccountInfoResp = self.client.account_info(account_query.into()).await?;
        Ok(serde_wasm_bindgen::to_value(&result)?)
    }

    #[wasm_bindgen(js_name=getAccountBalances)]
    pub async fn account_balances(
        &self,
        account_id: u32,
        sub_account_id: Option<u8>,
    ) -> Result<JsValue, JsValue> {
        let result: SubAccountBalances = self
            .client
            .account_balances(
                AccountId(account_id),
                sub_account_id.map(|id| SubAccountId(id)),
            )
            .await?;
        Ok(serde_wasm_bindgen::to_value(&result)?)
    }

    #[wasm_bindgen(js_name=getAccountOrderSlots)]
    pub async fn account_order_slots(
        &self,
        account_id: u32,
        sub_account_id: Option<u8>,
    ) -> Result<JsValue, JsValue> {
        let result: SubAccountOrders = self
            .client
            .account_order_slots(
                AccountId(account_id),
                sub_account_id.map(|id| SubAccountId(id)),
            )
            .await?;
        Ok(serde_wasm_bindgen::to_value(&result)?)
    }

    #[wasm_bindgen(js_name=getTokenReserve)]
    pub async fn token_remain(&self, token_id: u32, mapping: bool) -> Result<JsValue, JsValue> {
        let result: HashMap<ChainId, BigUintSerdeWrapper> =
            self.client.token_remain(TokenId(token_id), mapping).await?;
        Ok(serde_wasm_bindgen::to_value(&result)?)
    }

    #[wasm_bindgen(js_name=getTransactionByHash)]
    pub async fn tx_info(&self, hash: String, include_update: bool) -> Result<JsValue, JsValue> {
        let hash = TxHash::from_hex(&hash).map_err(|_e| RpcError::InvalidInputParameter)?;
        let result: TxResp = self.client.tx_info(hash, include_update).await?;
        Ok(serde_wasm_bindgen::to_value(&result)?)
    }

    #[wasm_bindgen(js_name=getAccountTransactionHistory)]
    pub async fn tx_history(
        &self,
        tx_type: L2TxType,
        address: String,
        page_index: u64,
        page_size: u32,
    ) -> Result<JsValue, JsValue> {
        let address =
            ZkLinkAddress::from_hex(&address).map_err(|_e| RpcError::InvalidInputParameter)?;
        let result: Page<ZkLinkTxHistory> = self
            .client
            .tx_history(tx_type.into(), address, page_index, page_size)
            .await?;
        Ok(serde_wasm_bindgen::to_value(&result)?)
    }

    #[wasm_bindgen(js_name=getFastWithdrawTxs)]
    pub async fn tx_fast_withdraw(
        &self,
        last_tx_timestamp: u64,
        max_txs: u32,
    ) -> Result<JsValue, JsValue> {
        let result: Vec<FastWithdrawTxResp> = self
            .client
            .tx_fast_withdraw(last_tx_timestamp, max_txs)
            .await?;
        Ok(serde_wasm_bindgen::to_value(&result)?)
    }

    #[wasm_bindgen(js_name=pullForwardTxs)]
    pub async fn pull_forward_txs(
        &self,
        sub_account_id: u8,
        offset_id: i64,
        limit: i64,
    ) -> Result<JsValue, JsValue> {
        let result: Vec<ForwardTxResp> = self
            .client
            .pull_forward_txs(SubAccountId(sub_account_id), offset_id, limit)
            .await?;
        Ok(serde_wasm_bindgen::to_value(&result)?)
    }

    #[wasm_bindgen(js_name=confirmFullExit)]
    pub async fn confirm_full_exit(
        &self,
        tx_hash: String,
        submitter_signature: String,
    ) -> Result<bool, JsValue> {
        let hash = TxHash::from_hex(&tx_hash).map_err(|_e| RpcError::InvalidInputParameter)?;
        let signature = ZkLinkSignature::from_hex(&submitter_signature)
            .map_err(|_e| RpcError::InvalidInputParameter)?;
        let result: bool = self.client.confirm_full_exit(hash, signature).await?;
        Ok(result)
    }
}
