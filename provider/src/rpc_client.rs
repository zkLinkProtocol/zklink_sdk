use crate::network::Network;
use crate::response::{
    AccountInfoResp, AccountQuery, AccountSnapshotResp, BlockNumberResp, BlockOnChainResp,
    BlockResp, ChainResp, EthPropertyResp, GlobalVarsResp, Page, SubAccountBalances,
    SubAccountOrders, SubAccountPositions, TokenResp, TxHashOrDetailResp, TxResp, WithdrawTxResp,
    ZkLinkTxHistory,
};
use jsonrpsee_core::client::{ClientT, IdKind};
use jsonrpsee_core::{rpc_params, ClientError};
use jsonrpsee_http_client::transport::HttpBackend;
use jsonrpsee_http_client::HttpClient;
use std::collections::HashMap;
use zklink_sdk_signers::zklink_signer::ZkLinkSignature;
use zklink_sdk_types::prelude::{
    AccountId, BigUintSerdeWrapper, BlockNumber, ChainId, OraclePrices, SubAccountId, TokenId,
    TxHash, TxLayer1Signature, ZkLinkAddress, ZkLinkTx, ZkLinkTxType,
};

pub struct RpcClient {
    inner: HttpClient<HttpBackend>,
}

pub type RpcResult<T> = Result<T, ClientError>;

impl RpcClient {
    pub fn new(network: Network) -> Self {
        let url = network.url();
        let client = HttpClient::<HttpBackend>::builder()
            .id_format(IdKind::String)
            .build(url)
            .unwrap();
        Self { inner: client }
    }

    pub async fn get_support_chains(&self) -> RpcResult<Vec<ChainResp>> {
        let method = "getSupportChains";
        let resp = self.inner.request(method, rpc_params![]).await?;
        Ok(resp)
    }

    pub async fn get_tokens(&self) -> RpcResult<HashMap<TokenId, TokenResp>> {
        let method = "tokens";
        let resp = self.inner.request(method, rpc_params![]).await?;
        Ok(resp)
    }

    pub async fn get_last_block_number(&self) -> RpcResult<BlockNumberResp> {
        let method = "getLatestBlockNumber";
        let resp = self.inner.request(method, rpc_params![]).await?;
        Ok(resp)
    }

    pub async fn block_detail(
        &self,
        block_number: Option<BlockNumber>,
        include_tx: bool,
        include_update: bool,
    ) -> RpcResult<BlockResp> {
        let method = "getBlockByNumber";
        let resp = self
            .inner
            .request(
                method,
                rpc_params![block_number, include_tx, include_update],
            )
            .await?;
        Ok(resp)
    }

    pub async fn pending_block_detail(
        &self,
        last_tx_timestamp_micro: u64,
        include_tx: bool,
        include_update: bool,
        limit: Option<usize>,
    ) -> RpcResult<Vec<TxHashOrDetailResp>> {
        let method = "getPendingBlock";
        let resp = self
            .inner
            .request(
                method,
                rpc_params![last_tx_timestamp_micro, include_tx, include_update, limit],
            )
            .await?;
        Ok(resp)
    }

    pub async fn block_onchain_detail(
        &self,
        block_number: BlockNumber,
    ) -> RpcResult<BlockOnChainResp> {
        let method = "getBlockOnChainByNumber";
        let resp = self
            .inner
            .request(method, rpc_params![block_number])
            .await?;
        Ok(resp)
    }

    pub async fn account_info(&self, account_query: AccountQuery) -> RpcResult<AccountInfoResp> {
        let method = "getAccount";
        let resp = self
            .inner
            .request(method, rpc_params![account_query])
            .await?;
        Ok(resp)
    }

    pub async fn global_vars_info(
        &self,
        sub_account_query: SubAccountId,
    ) -> RpcResult<GlobalVarsResp> {
        let method = "getSubAccountGlobalVars";
        let resp = self
            .inner
            .request(method, rpc_params![sub_account_query])
            .await?;
        Ok(resp)
    }

    pub async fn account_balances(
        &self,
        account_id: AccountId,
        sub_account_id: Option<SubAccountId>,
    ) -> RpcResult<SubAccountBalances> {
        let method = "getAccountBalances";
        let resp = self
            .inner
            .request(method, rpc_params![account_id, sub_account_id])
            .await?;
        Ok(resp)
    }

    pub async fn account_order_slots(
        &self,
        account_id: AccountId,
        sub_account_id: Option<SubAccountId>,
    ) -> RpcResult<SubAccountOrders> {
        let method = "getAccountOrderSlots";
        let resp = self
            .inner
            .request(method, rpc_params![account_id, sub_account_id])
            .await?;
        Ok(resp)
    }

    pub async fn account_positions(
        &self,
        account_id: AccountId,
        sub_account_id: Option<SubAccountId>,
    ) -> RpcResult<SubAccountPositions> {
        let method = "getAccountPositions";
        let resp = self
            .inner
            .request(method, rpc_params![account_id, sub_account_id])
            .await?;
        Ok(resp)
    }

    pub async fn token_remain(
        &self,
        token_id: TokenId,
        mapping: bool,
    ) -> RpcResult<HashMap<ChainId, BigUintSerdeWrapper>> {
        let method = "getTokenReserve";
        let resp = self
            .inner
            .request(method, rpc_params![token_id, mapping])
            .await?;
        Ok(resp)
    }

    pub async fn get_account_snapshot(
        &self,
        account_query: AccountQuery,
        sub_account_id: Option<SubAccountId>,
        block_number: Option<BlockNumber>,
    ) -> RpcResult<AccountSnapshotResp> {
        let method = "getAccountSnapshot";
        let resp = self
            .inner
            .request(
                method,
                rpc_params![account_query, sub_account_id, block_number],
            )
            .await?;
        Ok(resp)
    }

    pub async fn get_tx_info(&self, hash: TxHash, include_update: bool) -> RpcResult<TxResp> {
        let method = "getTransactionByHash";
        let resp = self
            .inner
            .request(method, rpc_params![hash, include_update])
            .await?;
        Ok(resp)
    }

    pub async fn get_account_tx_history(
        &self,
        tx_type: ZkLinkTxType,
        address: ZkLinkAddress,
        page_index: u64,
        page_size: u32,
    ) -> RpcResult<Page<ZkLinkTxHistory>> {
        let method = "getAccountTransactionHistory";
        let resp = self
            .inner
            .request(method, rpc_params![tx_type, address, page_index, page_size])
            .await?;
        Ok(resp)
    }

    pub async fn get_withdraw_txs(
        &self,
        last_tx_timestamp_micro: u64,
        max_txs: u32,
    ) -> RpcResult<Vec<WithdrawTxResp>> {
        let method = "getWithdrawTxs";
        let resp = self
            .inner
            .request(method, rpc_params![last_tx_timestamp_micro, max_txs])
            .await?;
        Ok(resp)
    }

    // async fn get_websocket_events(
    //     &self,
    //     topic: Topic,
    //     offset: ClientOffset,
    // ) -> RpcResult<Vec<TxTopicEvent>> {
    //     let method = "getWebSocketEvents";
    //     let resp = self.inner.request(method, rpc_params![topic, offset]).await?;
    //     Ok(resp)
    // }

    pub async fn get_change_pubkey_chain_id(&self) -> RpcResult<ChainId> {
        let method = "getChangePubkeyChainId";
        let resp = self.inner.request(method, rpc_params![]).await?;
        Ok(resp)
    }

    pub async fn get_eth_property(&self) -> RpcResult<EthPropertyResp> {
        let method = "getEthProperty";
        let resp = self.inner.request(method, rpc_params![]).await?;
        Ok(resp)
    }

    pub async fn tx_submit(
        &self,
        tx: ZkLinkTx,
        l1_signature: Option<TxLayer1Signature>,
        submitter_signature: Option<ZkLinkSignature>,
    ) -> RpcResult<TxHash> {
        let method = "sendTransaction";
        let resp = self
            .inner
            .request(method, rpc_params![tx, l1_signature, submitter_signature])
            .await?;
        Ok(resp)
    }

    pub async fn confirm_full_exit(
        &self,
        tx_hash: TxHash,
        oracle_prices: OraclePrices,
        submitter_signature: ZkLinkSignature,
    ) -> RpcResult<bool> {
        let method = "confirmFullExit";
        let resp = self
            .inner
            .request(
                method,
                rpc_params![tx_hash, oracle_prices, submitter_signature],
            )
            .await?;
        Ok(resp)
    }
}
