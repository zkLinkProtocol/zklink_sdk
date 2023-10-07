use jsonrpsee::types::request::Request;
use jsonrpsee::types::response::Response;
use zklink_sdk_types::tx_type::zklink_tx::ZkLinkTx;
use zklink_sdk_types::signatures::TxLayer1Signature;
use zklink_sdk_signers::zklink_signer::ZkLinkSignature;
use jsonrpsee::core::params::ArrayParams;
use zklink_sdk_types::prelude::TxHash;
use crate::error::RpcError;
use wasm_bindgen::JsValue;
use jsonrpsee::core::traits::ToRpcParams;
use jsonrpsee::types::Id;
use std::collections::HashMap;
use crate::response::{TokenResp, AccountSnapshotResp, AccountQuery};
use zklink_sdk_types::basic_types::{TokenId, SubAccountId, BlockNumber};

impl From<RpcError> for JsValue {
    fn from(error: RpcError) -> Self {
        JsValue::from_str(&format!("error: {error}"))
    }
}

pub struct WasmRpcClient {
    pub server_url: String,
}

impl WasmRpcClient {
    pub fn new(server_url: String) ->Self {
        Self {
            server_url
        }
    }

    pub async fn tokens(
        &self,
    ) -> Result<HashMap<TokenId,TokenResp>,RpcError> {
        let request = Request::new("getSupportTokens".into(),
                                   None,Id::Number(1));
        let res = reqwest::Client::new()
            .post(&self.server_url)
            .json(&request)
            .send()
            .await
            .map_err(RpcError::RequestError)?
            .json::<HashMap<String,serde_json::Value>>()
            .await
            .map_err(RpcError::ResponseError)?;
        if let Some(&ref result) = res.get("result") {
            let resp: HashMap<TokenId,TokenResp> = serde_json::from_value(result.clone())
                .map_err(|_e| RpcError::ParseJsonError)?;
            Ok(resp)
        } else {
            Err(RpcError::ParseJsonError)
        }
    }

    pub async fn account_query(
        &self,
        account_query: AccountQuery,
        sub_account_id: Option<SubAccountId>,
        block_number: Option<BlockNumber>,
    ) -> Result<AccountSnapshotResp,RpcError> {
        let mut builder = ArrayParams::new();
        builder.insert(account_query);
        builder.insert(sub_account_id);
        builder.insert(block_number);
        let params = builder.to_rpc_params().map_err(RpcError::ParseParamsError)?;
        let request = Request::new("getAccountSnapshot".into(),
                                   params.as_ref().map(|p| p.as_ref()),Id::Number(1));
        let res = reqwest::Client::new()
            .post(&self.server_url)
            .json(&request)
            .send()
            .await
            .map_err(RpcError::RequestError)?
            .json::<HashMap<String,serde_json::Value>>()
            .await
            .map_err(RpcError::ResponseError)?;
        if let Some(&ref result) = res.get("result") {
            let resp: AccountSnapshotResp = serde_json::from_value(result.clone())
                .map_err(|_e| RpcError::ParseJsonError)?;
            Ok(resp)
        } else {
            Err(RpcError::ParseJsonError)
        }
    }

    pub async fn send_transaction(
        &self,
        tx: ZkLinkTx,
        eth_signature: Option<TxLayer1Signature>,
        submitter_signature: Option<ZkLinkSignature>,
    ) -> Result<TxHash,RpcError> {
        let mut builder = ArrayParams::new();
        builder.insert(tx);
        builder.insert(eth_signature);
        builder.insert(submitter_signature);
        let params = builder.to_rpc_params().map_err(RpcError::ParseParamsError)?;
        let request = Request::new("sendTransaction".into(),
                               params.as_ref().map(|p| p.as_ref()),Id::Number(0));
        let res = reqwest::Client::new()
            .post(&self.server_url)
            .json(&request)
            .send()
            .await
            .map_err(RpcError::RequestError)?
            .json::<HashMap<String,serde_json::Value>>()
            .await
            .map_err(RpcError::ResponseError)?;
        if let Some(&ref result) = res.get("result") {
            let resp: TxHash = serde_json::from_value(result.clone())
                .map_err(|_e| RpcError::ParseJsonError)?;
            Ok(resp)
        } else {
            Err(RpcError::ParseJsonError)
        }
    }
}

#[cfg(test)]
mod test {
    use super::WasmRpcClient;
    use zklink_sdk_signers::eth_signer::EthSigner;
    use zklink_sdk_signers::zklink_signer::{ZkLinkSigner, PubKeyHash};
    use zklink_sdk_types::tx_builder::{ChangePubKeyBuilder, TransferBuilder};
    use zklink_sdk_types::basic_types::{ChainId, AccountId, SubAccountId, TokenId, Nonce, TimeStamp, ZkLinkAddress};
    use zklink_sdk_types::tx_type::change_pubkey::ChangePubKey;
    use std::str::FromStr;
    use zklink_sdk_types::tx_type::zklink_tx::ZkLinkTx;
    use zklink_sdk_types::basic_types::BigUint;
    use crate::rpc::ZkLinkRpcClient;
    use crate::ZkLinkRpcProvider;
    use crate::network::Network;
    use std::time::Duration;
    use zklink_sdk_types::tx_type::transfer::Transfer;
    use zklink_sdk_types::tx_type::ZkSignatureTrait;
    use zklink_sdk_types::signatures::TxLayer1Signature;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_tokens() {
        let client = WasmRpcClient::new("https://api-v1.zk.link".to_owned());
        let ret = client.tokens().await.unwrap();
        println!("{:?}",ret);
    }

    #[tokio::test]
    async fn test_send_change_pubkey() {
        let private_key = "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
        let eth_signer = EthSigner::try_from(private_key).unwrap();
        let zklink_signer = ZkLinkSigner::new_from_hex_eth_signer(private_key).unwrap();
        let main_contract = "0x5505a8cD4594Dbf79d8C59C0Df1414AB871CA896";
        let l1_client_id = 80001;
        let new_pubkey_hash = "0xd8d5fb6a6caef06aa3dc2abdcdc240987e5330fe";
        let ts  = 1696595303;
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
            timestamp: TimeStamp(ts)
        };
        let mut change_pubkey = ChangePubKey::new(builder);
        let message = change_pubkey.to_eip712_request_payload(
            l1_client_id,
            &ZkLinkAddress::from_str(&main_contract).unwrap()).unwrap();
        let signature = eth_signer.sign_message(message.raw_data.as_bytes()).unwrap();
        let builder_with_sig = ChangePubKeyBuilder {
            chain_id: ChainId(1),
            account_id: AccountId(10),
            sub_account_id: SubAccountId(1),
            new_pubkey_hash: PubKeyHash::from_hex(new_pubkey_hash).unwrap(),
            fee_token: TokenId(18),
            fee: BigUint::from(100000000000000u64),
            nonce: Nonce(1),
            eth_signature: Some(signature),
            timestamp: TimeStamp(ts)
        };
        let mut tx = ChangePubKey::new(builder_with_sig);
        tx.sign(&zklink_signer).unwrap();

        // for wasm
        // let client = WasmRpcClient::new("https://aws-gw-v2.zk.link".to_owned());
        // client.send_transaction(ZkLinkTx::ChangePubKey(Box::new(tx.clone())),None,Some(tx.signature.clone())).await.unwrap();

        //use jsonrpsee
        let client = ZkLinkRpcProvider::new(Network::TestNet, Duration::from_secs(3));
        let ret = client.tx_submit(
            ZkLinkTx::ChangePubKey(Box::new(tx.clone())),
            None,
            Some(tx.signature.clone())
        )
            .await.unwrap();
    }

    #[tokio::test]
    async fn test_send_transfer() {
        let private_key = "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
        let eth_signer = EthSigner::try_from(private_key).unwrap();
        let zklink_signer = ZkLinkSigner::new_from_hex_eth_signer(private_key).unwrap();
        let to_address = "0x5505a8cD4594Dbf79d8C59C0Df1414AB871CA896";
        let l1_client_id = 80001;
        let ts  = 1696595303;
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
            amount: BigUint::from(1000000000000000u64)
        };
        let mut transfer = Transfer::new(builder);
        let eth_signature = eth_signer.sign_message(
            transfer.get_eth_sign_msg("USDT").as_bytes()).unwrap();
        transfer.sign(&zklink_signer).unwrap();

        let submiiter_signature = transfer.submitter_signature(Arc::new(zklink_signer)).unwrap();
        // for wasm
        let client = WasmRpcClient::new("https://aws-gw-v2.zk.link".to_owned());
        client.send_transaction(ZkLinkTx::Transfer(Box::new(transfer.clone())),
                                Some(TxLayer1Signature::EthereumSignature(eth_signature)),Some(submiiter_signature)).await.unwrap();

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