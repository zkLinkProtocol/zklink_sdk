use jsonrpsee::proc_macros::rpc;
use std::collections::HashMap;

use super::response::*;
use crate::web_socket::ws_message::message::request::ClientOffset;
use crate::web_socket::ws_message::message::response::TxTopicEvent;
use crate::web_socket::ws_message::topic::Topic;
use jsonrpsee::core::RpcResult;
use zklink_sdk_types::basic_types::tx_hash::TxHash;
use zklink_sdk_types::basic_types::{AccountId, BlockNumber, ChainId, SubAccountId, TokenId, ZkLinkAddress};
use zklink_sdk_types::prelude::BigUintSerdeWrapper;
use zklink_sdk_types::signatures::TxLayer1Signature;
use zklink_sdk_types::tx_type::zklink_tx::{ZkLinkTx, ZkLinkTxType};

#[rpc(client, server)]
pub trait ZkLinkRpc {
    #[method(name = "getSupportChains")]
    async fn get_support_chains(&self) -> RpcResult<Vec<ChainResp>>;

    #[method(name = "getSupportTokens")]
    async fn tokens(&self) -> RpcResult<HashMap<TokenId, TokenResp>>;

    #[method(name = "getLatestBlockNumber")]
    async fn block_info(&self) -> RpcResult<BlockNumberResp>;

    #[method(name = "getBlockByNumber")]
    async fn block_detail(
        &self,
        block_number: Option<BlockNumber>,
        include_tx: bool,
        include_update: bool,
    ) -> RpcResult<BlockResp>;

    #[method(name = "getPendingBlock")]
    async fn pending_block_detail(
        &self,
        last_tx_timestamp_micro: u64,
        include_tx: bool,
        include_update: bool,
        limit: Option<usize>,
    ) -> RpcResult<Vec<TxHashOrDetailResp>>;

    #[method(name = "getBlockOnChainByNumber")]
    async fn block_onchain_detail(&self, block_number: BlockNumber) -> RpcResult<BlockOnChainResp>;

    #[method(name = "getAccount")]
    async fn account_info(&self, account_query: AccountQuery) -> RpcResult<AccountInfoResp>;

    #[method(name = "getSubAccountGlobalVars")]
    async fn global_vars_info(&self, sub_account_query: SubAccountId) -> RpcResult<GlobalVarsResp>;

    #[method(name = "getAccountBalances")]
    async fn account_balances(
        &self,
        account_id: AccountId,
        sub_account_id: Option<SubAccountId>,
    ) -> RpcResult<SubAccountBalances>;

    #[method(name = "getAccountOrderSlots")]
    async fn account_order_slots(
        &self,
        account_id: AccountId,
        sub_account_id: Option<SubAccountId>,
    ) -> RpcResult<SubAccountOrders>;

    #[method(name = "getAccountPositions")]
    async fn account_positions(
        &self,
        account_id: AccountId,
        sub_account_id: Option<SubAccountId>,
    ) -> RpcResult<SubAccountPositions>;

    #[method(name = "getTokenReserve")]
    async fn token_remain(&self, token_id: TokenId, mapping: bool) -> RpcResult<HashMap<ChainId, BigUintSerdeWrapper>>;

    #[method(name = "getAccountSnapshot")]
    async fn account_snapshot(
        &self,
        account_query: AccountQuery,
        sub_account_id: Option<SubAccountId>,
        block_number: Option<BlockNumber>,
    ) -> RpcResult<AccountSnapshotResp>;

    #[method(name = "getTransactionByHash")]
    async fn tx_info(&self, hash: TxHash, include_update: bool) -> RpcResult<TxResp>;

    #[method(name = "getAccountTransactionHistory")]
    async fn tx_history(
        &self,
        tx_type: ZkLinkTxType,
        address: ZkLinkAddress,
        page_index: u64,
        page_size: u32,
    ) -> RpcResult<Page<ZkLinkTxHistory>>;

    #[method(name = "getWithdrawTxs")]
    async fn tx_withdraw(&self, last_tx_timestamp_micro: u64, max_txs: u32) -> RpcResult<Vec<WithdrawTxResp>>;

    #[method(name = "getWebSocketEvents")]
    async fn get_websocket_events(&self, topic: Topic, offset: ClientOffset) -> RpcResult<Vec<TxTopicEvent>>;

    #[method(name = "getChangePubkeyChainId")]
    async fn get_change_pubkey_chain_id(&self) -> RpcResult<ChainId>;

    #[method(name = "getEthProperty")]
    async fn get_eth_property(&self) -> RpcResult<EthPropertyResp>;

    #[method(name = "sendTransaction")]
    async fn tx_submit(&self, tx: ZkLinkTx, l1_signature: Option<TxLayer1Signature>) -> RpcResult<TxHash>;
}
