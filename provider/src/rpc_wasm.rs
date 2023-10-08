use crate::error::RpcError;
use crate::response::{
    AccountInfoResp, AccountQuery, AccountSnapshotResp, BlockNumberResp, BlockOnChainResp,
    BlockResp, ChainResp, FastWithdrawTxResp, ForwardTxResp, Page, SubAccountBalances,
    SubAccountOrders, TokenResp, TxHashOrDetailResp, TxResp, ZkLinkTxHistory,
};
use getrandom::getrandom;
use jsonrpsee::core::params::ArrayParams;
use jsonrpsee::core::traits::ToRpcParams;
use jsonrpsee::types::request::Request;
use jsonrpsee::types::Id;
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use zklink_sdk_signers::zklink_signer::ZkLinkSignature;
use zklink_sdk_types::basic_types::bigunit_wrapper::BigUintSerdeWrapper;
use zklink_sdk_types::basic_types::{
    AccountId, BlockNumber, ChainId, SubAccountId, TokenId, ZkLinkAddress,
};
use zklink_sdk_types::prelude::TxHash;
use zklink_sdk_types::signatures::TxLayer1Signature;
use zklink_sdk_types::tx_type::zklink_tx::{ZkLinkTx, ZkLinkTxType};

impl From<RpcError> for JsValue {
    fn from(error: RpcError) -> Self {
        JsValue::from_str(&format!("error: {error}"))
    }
}

macro_rules! make_rpc_request {
    ($method:expr,$builder:expr, $server_url:expr, $resp_type: ty) => {{
        let params = $builder
            .to_rpc_params()
            .map_err(RpcError::ParseParamsError)?;
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
            .json::<HashMap<String, serde_json::Value>>()
            .await
            .map_err(RpcError::ResponseError)?;
        if let Some(&ref result) = res.get("result") {
            let resp: HashMap<TokenId, TokenResp> =
                serde_json::from_value(result.clone()).map_err(|_e| RpcError::ParseJsonError)?;
            Ok(resp)
        } else {
            Err(RpcError::ParseJsonError)
        }
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

pub struct WasmRpcClient {
    pub server_url: String,
}

impl WasmRpcClient {
    pub fn new(server_url: String) -> Self {
        Self { server_url }
    }

    pub async fn tokens(&self) -> Result<HashMap<TokenId, TokenResp>, RpcError> {
        let builder = ArrayParams::new();
        make_rpc_request!("getSupportTokens",builder,&self.server_url,HashMap<TokenId, TokenResp>)
    }

    pub async fn account_query(
        &self,
        account_query: AccountQuery,
        sub_account_id: Option<SubAccountId>,
        block_number: Option<BlockNumber>,
    ) -> Result<AccountSnapshotResp, RpcError> {
        let mut builder = ArrayParams::new();
        let _ = builder.insert(account_query);
        let _ = builder.insert(sub_account_id);
        let _ = builder.insert(block_number);
        make_rpc_request!(
            "getAccountSnapshot",
            builder,
            &self.server_url,
            AccountSnapshotResp
        )
    }

    pub async fn send_transaction(
        &self,
        tx: ZkLinkTx,
        eth_signature: Option<TxLayer1Signature>,
        submitter_signature: Option<ZkLinkSignature>,
    ) -> Result<TxHash, RpcError> {
        let mut builder = ArrayParams::new();
        let _ = builder.insert(tx);
        let _ = builder.insert(eth_signature);
        let _ = builder.insert(submitter_signature);
        make_rpc_request!("sendTransaction", builder, &self.server_url, TxHash)
    }

    pub async fn get_support_chains(&self) -> Result<Vec<ChainResp>, RpcError> {
        let builder = ArrayParams::new();
        make_rpc_request!(
            "getSupportChains",
            builder,
            &self.server_url,
            Vec<ChainResp>
        )
    }

    pub async fn block_info(&self) -> Result<BlockNumberResp, RpcError> {
        let builder = ArrayParams::new();
        make_rpc_request!(
            "getLatestBlockNumber",
            builder,
            &self.server_url,
            BlockNumberResp
        )
    }

    pub async fn block_detail(
        &self,
        block_number: Option<BlockNumber>,
        include_tx: bool,
        include_update: bool,
    ) -> Result<BlockResp, RpcError> {
        let mut builder = ArrayParams::new();
        let _ = builder.insert(block_number);
        let _ = builder.insert(include_tx);
        let _ = builder.insert(include_update);
        make_rpc_request!("getBlockByNumber", builder, &self.server_url, BlockResp)
    }

    pub async fn pending_block_detail(
        &self,
        last_tx_timestamp_micro: u64,
        include_tx: bool,
        include_update: bool,
        limit: Option<usize>,
    ) -> Result<Vec<TxHashOrDetailResp>, RpcError> {
        let mut builder = ArrayParams::new();
        let _ = builder.insert(last_tx_timestamp_micro);
        let _ = builder.insert(include_tx);
        let _ = builder.insert(include_update);
        let _ = builder.insert(limit);
        make_rpc_request!(
            "getPendingBlock",
            builder,
            &self.server_url,
            Vec<TxHashOrDetailResp>
        )
    }

    pub async fn block_onchain_detail(
        &self,
        block_number: BlockNumber,
    ) -> Result<BlockOnChainResp, RpcError> {
        let mut builder = ArrayParams::new();
        let _ = builder.insert(block_number);
        make_rpc_request!(
            "getBlockOnChainByNumber",
            builder,
            &self.server_url,
            BlockOnChainResp
        )
    }

    pub async fn account_info(
        &self,
        account_query: AccountQuery,
    ) -> Result<AccountInfoResp, RpcError> {
        let mut builder = ArrayParams::new();
        let _ = builder.insert(account_query);
        make_rpc_request!("getAccount", builder, &self.server_url, AccountInfoResp)
    }

    pub async fn account_balances(
        &self,
        account_id: AccountId,
        sub_account_id: Option<SubAccountId>,
    ) -> Result<SubAccountBalances, RpcError> {
        let mut builder = ArrayParams::new();
        let _ = builder.insert(account_id);
        let _ = builder.insert(sub_account_id);
        make_rpc_request!(
            "getAccountBalances",
            builder,
            &self.server_url,
            SubAccountBalances
        )
    }

    pub async fn account_order_slots(
        &self,
        account_id: AccountId,
        sub_account_id: Option<SubAccountId>,
    ) -> Result<SubAccountOrders, RpcError> {
        let mut builder = ArrayParams::new();
        let _ = builder.insert(account_id);
        let _ = builder.insert(sub_account_id);
        make_rpc_request!(
            "getAccountOrderSlots",
            builder,
            &self.server_url,
            SubAccountOrders
        )
    }

    pub async fn token_remain(
        &self,
        token_id: TokenId,
        mapping: bool,
    ) -> Result<HashMap<ChainId, BigUintSerdeWrapper>, RpcError> {
        let mut builder = ArrayParams::new();
        let _ = builder.insert(token_id);
        let _ = builder.insert(mapping);
        make_rpc_request!("getTokenReserve",builder,&self.server_url,HashMap<ChainId, BigUintSerdeWrapper>)
    }

    pub async fn tx_info(&self, hash: TxHash, include_update: bool) -> Result<TxResp, RpcError> {
        let mut builder = ArrayParams::new();
        let _ = builder.insert(hash);
        let _ = builder.insert(include_update);
        make_rpc_request!("getTransactionByHash", builder, &self.server_url, TxResp)
    }

    pub async fn tx_history(
        &self,
        tx_type: ZkLinkTxType,
        address: ZkLinkAddress,
        page_index: u64,
        page_size: u32,
    ) -> Result<Page<ZkLinkTxHistory>, RpcError> {
        let mut builder = ArrayParams::new();
        let _ = builder.insert(tx_type);
        let _ = builder.insert(address);
        let _ = builder.insert(page_index);
        let _ = builder.insert(page_size);
        make_rpc_request!(
            "getAccountTransactionHistory",
            builder,
            &self.server_url,
            Page<ZkLinkTxHistory>
        )
    }

    pub async fn tx_fast_withdraw(
        &self,
        last_tx_timestamp: u64,
        max_txs: u32,
    ) -> Result<Vec<FastWithdrawTxResp>, RpcError> {
        let mut builder = ArrayParams::new();
        let _ = builder.insert(last_tx_timestamp);
        let _ = builder.insert(max_txs);
        make_rpc_request!(
            "getFastWithdrawTxs",
            builder,
            &self.server_url,
            Vec<FastWithdrawTxResp>
        )
    }

    pub async fn pull_forward_txs(
        &self,
        sub_account_id: SubAccountId,
        offset_id: i64,
        limit: i64,
    ) -> Result<Vec<ForwardTxResp>, RpcError> {
        let mut builder = ArrayParams::new();
        let _ = builder.insert(sub_account_id);
        let _ = builder.insert(offset_id);
        let _ = builder.insert(limit);
        make_rpc_request!(
            "pullForwardTxs",
            builder,
            &self.server_url,
            Vec<ForwardTxResp>
        )
    }

    pub async fn confirm_full_exit(
        &self,
        tx_hash: TxHash,
        submitter_signature: ZkLinkSignature,
    ) -> Result<bool, RpcError> {
        let mut builder = ArrayParams::new();
        let _ = builder.insert(tx_hash);
        let _ = builder.insert(submitter_signature);
        make_rpc_request!("confirmFullExit", builder, &self.server_url, bool)
    }
}

#[cfg(test)]
mod test {
    use super::WasmRpcClient;
    use crate::network::Network;
    use crate::response::ZkLinkTxHistory;
    use crate::rpc::ZkLinkRpcClient;
    use crate::ZkLinkRpcProvider;
    use std::str::FromStr;
    use std::sync::Arc;
    use std::time::Duration;
    use zklink_sdk_signers::eth_signer::EthSigner;
    use zklink_sdk_signers::zklink_signer::{PubKeyHash, ZkLinkSigner};
    use zklink_sdk_types::basic_types::BigUint;
    use zklink_sdk_types::basic_types::{
        AccountId, ChainId, Nonce, SubAccountId, TimeStamp, TokenId, ZkLinkAddress,
    };
    use zklink_sdk_types::prelude::ZkLinkTxType;
    use zklink_sdk_types::signatures::TxLayer1Signature;
    use zklink_sdk_types::tx_builder::{ChangePubKeyBuilder, TransferBuilder};
    use zklink_sdk_types::tx_type::change_pubkey::ChangePubKey;
    use zklink_sdk_types::tx_type::transfer::Transfer;
    use zklink_sdk_types::tx_type::zklink_tx::ZkLinkTx;
    use zklink_sdk_types::tx_type::ZkSignatureTrait;

    #[tokio::test]
    async fn test_tokens() {
        let client = WasmRpcClient::new("https://api-v1.zk.link".to_owned());
        let ret = client.tokens().await.unwrap();
        println!("{:?}", ret);
    }

    #[tokio::test]
    async fn test_tx_history() {
        let client = WasmRpcClient::new("https://api-v1.zk.link".to_owned());
        let ret = client
            .tx_history(
                ZkLinkTxType::Deposit,
                ZkLinkAddress::from_hex("0x12aFF993702B5d623977A9044686Fa1A2B0c2147").unwrap(),
                0,
                1,
            )
            .await
            .unwrap();
        println!("{:?}", ret);
    }

    #[tokio::test]
    async fn test_send_change_pubkey() {
        let private_key = "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
        let eth_signer = EthSigner::try_from(private_key).unwrap();
        let zklink_signer = ZkLinkSigner::new_from_hex_eth_signer(private_key).unwrap();
        let main_contract = "0x5505a8cD4594Dbf79d8C59C0Df1414AB871CA896";
        let l1_client_id = 80001;
        let new_pubkey_hash = "0xd8d5fb6a6caef06aa3dc2abdcdc240987e5330fe";
        let ts = 1696595303;
        //auth type 'ECDSA'
        let builder = ChangePubKeyBuilder {
            chain_id: ChainId(1),
            account_id: AccountId(10),
            sub_account_id: SubAccountId(1),
            new_pubkey_hash: PubKeyHash::from_hex(new_pubkey_hash).unwrap(),
            fee_token: TokenId(18),
            fee: BigUint::from(100000000000000u64),
            nonce: Nonce(1),
            eth_signature: None,
            timestamp: TimeStamp(ts),
        };
        let mut change_pubkey = ChangePubKey::new(builder);
        let message = change_pubkey
            .to_eip712_request_payload(
                l1_client_id,
                &ZkLinkAddress::from_str(&main_contract).unwrap(),
            )
            .unwrap();
        let signature = eth_signer
            .sign_message(message.raw_data.as_bytes())
            .unwrap();
        let builder_with_sig = ChangePubKeyBuilder {
            chain_id: ChainId(1),
            account_id: AccountId(10),
            sub_account_id: SubAccountId(1),
            new_pubkey_hash: PubKeyHash::from_hex(new_pubkey_hash).unwrap(),
            fee_token: TokenId(18),
            fee: BigUint::from(100000000000000u64),
            nonce: Nonce(1),
            eth_signature: Some(signature),
            timestamp: TimeStamp(ts),
        };
        let mut tx = ChangePubKey::new(builder_with_sig);
        tx.sign(&zklink_signer).unwrap();

        // for wasm
        // let client = WasmRpcClient::new("https://aws-gw-v2.zk.link".to_owned());
        // client.send_transaction(ZkLinkTx::ChangePubKey(Box::new(tx.clone())),None,Some(tx.signature.clone())).await.unwrap();

        //use jsonrpsee
        let client = ZkLinkRpcProvider::new(Network::TestNet, Duration::from_secs(3));
        let ret = client
            .tx_submit(
                ZkLinkTx::ChangePubKey(Box::new(tx.clone())),
                None,
                Some(tx.signature.clone()),
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_send_transfer() {
        let private_key = "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
        let eth_signer = EthSigner::try_from(private_key).unwrap();
        let zklink_signer = ZkLinkSigner::new_from_hex_eth_signer(private_key).unwrap();
        let to_address = "0x5505a8cD4594Dbf79d8C59C0Df1414AB871CA896";
        let l1_client_id = 80001;
        let ts = 1696595303;
        //auth type 'ECDSA'
        let builder = TransferBuilder {
            account_id: AccountId(10),
            to_address: ZkLinkAddress::from_str(to_address).unwrap(),
            from_sub_account_id: SubAccountId(1),
            to_sub_account_id: SubAccountId(1),
            token: TokenId(18),
            fee: BigUint::from(100000000000000u64),
            nonce: Nonce(1),
            timestamp: TimeStamp(ts),
            amount: BigUint::from(1000000000000000u64),
        };
        let mut transfer = Transfer::new(builder);
        let eth_signature = eth_signer
            .sign_message(transfer.get_eth_sign_msg("USDT").as_bytes())
            .unwrap();
        transfer.sign(&zklink_signer).unwrap();

        let submiiter_signature = transfer
            .submitter_signature(Arc::new(zklink_signer))
            .unwrap();
        // for wasm
        let client = WasmRpcClient::new("https://aws-gw-v2.zk.link".to_owned());
        client
            .send_transaction(
                ZkLinkTx::Transfer(Box::new(transfer.clone())),
                Some(TxLayer1Signature::EthereumSignature(eth_signature)),
                Some(submiiter_signature),
            )
            .await
            .unwrap();

        //use jsonrpsee
        // let client = ZkLinkRpcProvider::new(Network::TestNet, Duration::from_secs(3));
        // let ret = client.tx_submit(
        //     ZkLinkTx::Transfer(Box::new(transfer.clone())),
        //     None,
        //     Some(transfer.signature.clone())
        // )
        //     .await.unwrap();
    }
}
