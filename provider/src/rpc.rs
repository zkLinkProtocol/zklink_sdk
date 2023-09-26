use jsonrpsee::core::RpcResult;
use jsonrpsee::proc_macros::rpc;
use std::collections::HashMap;

use super::response::*;
use zklink_sdk_signers::zklink_signer::signature::ZkLinkSignature;
use zklink_sdk_types::basic_types::tx_hash::TxHash;
use zklink_sdk_types::basic_types::{
    AccountId, BlockNumber, ChainId, SubAccountId, TokenId, ZkLinkAddress,
};
use zklink_sdk_types::prelude::BigUintSerdeWrapper;
use zklink_sdk_types::signatures::TxLayer1Signature;
use zklink_sdk_types::tx_type::zklink_tx::{ZkLinkTx, ZkLinkTxType};

#[rpc(client)]
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

    #[method(name = "getTokenReserve")]
    async fn token_remain(
        &self,
        token_id: TokenId,
        mapping: bool,
    ) -> RpcResult<HashMap<ChainId, BigUintSerdeWrapper>>;

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

    #[method(name = "getFastWithdrawTxs")]
    async fn tx_fast_withdraw(
        &self,
        last_tx_timestamp: u64,
        max_txs: u32,
    ) -> RpcResult<Vec<FastWithdrawTxResp>>;

    #[method(name = "pullForwardTxs")]
    async fn pull_forward_txs(
        &self,
        sub_account_id: SubAccountId,
        offset_id: i64,
        limit: i64,
    ) -> RpcResult<Vec<ForwardTxResp>>;

    #[method(name = "estimateTransactionFee")]
    #[deprecated(note = "This rpc will be removed in a future release")]
    async fn get_tx_fee(&self, tx: ZkLinkTx) -> RpcResult<BigUintSerdeWrapper>;

    #[method(name = "sendTransaction")]
    async fn tx_submit(
        &self,
        tx: ZkLinkTx,
        eth_signature: Option<TxLayer1Signature>,
        submitter_signature: Option<ZkLinkSignature>,
    ) -> RpcResult<TxHash>;

    #[method(name = "confirmFullExit")]
    async fn confirm_full_exit(
        &self,
        tx_hash: TxHash,
        submitter_signature: ZkLinkSignature,
    ) -> RpcResult<bool>;
}
