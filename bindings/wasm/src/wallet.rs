use crate::error::ClientError;
use crate::signer::Signer;
use crate::{AccountType, Wallet};
use num::BigUint;
use zklink_interface::{ChangePubKeyAuthRequest, TxSignature};
use zklink_provider::response::{AccountQuery, ChainResp, TokenResp, AccountInfoResp};
use zklink_provider::rpc::ZkLinkRpcClient;
use zklink_sdk_signers::eth_signer::eth_signature::TxEthSignature;
use zklink_sdk_signers::zklink_signer::pk_signer::ZkLinkSigner;

use zklink_sdk_signers::zklink_signer::pubkey_hash::PubKeyHash;
use zklink_sdk_signers::zklink_signer::signature::ZkLinkSignature;
use zklink_sdk_types::basic_types::params::MAIN_SUB_ACCOUNT_ID;
use zklink_sdk_types::basic_types::tx_hash::TxHash;
use zklink_sdk_types::basic_types::{AccountId, ChainId, Nonce, SubAccountId, TokenId, ZkLinkAddress};
use zklink_sdk_types::tx_type::order_matching::Order;
use std::collections::HashMap;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub struct Wallet {
    pub provider: ZkLinkRpcProvider,
    pub address: ZklinkAddress,
    pub account_info: AccountInfoResp,
    pub account_type: AccountType,
    pub chains: HashMap<ChainId, ChainResp>,
    pub tokens: HashMap<TokenId, TokenResp>,
}

#[wasm_bindgen]
impl Wallet {
    #[wasm_bindgen]
    pub async fn new(
        provider: ZkLinkRpcProvider,
        address: ZkLinkAddress,
        account_type: AccountType,
    ) -> Result<Wallet, JsValue> {
        let chains = provider.get_support_chains().await?;
        let chains = chains.iter().map(|c| (c.chain_id, c.clone())).collect();
        let tokens = provider.tokens().await?;
        let account_info = provider
            .account_info(AccountQuery::Address(address.clone()))
            .await?;

        let wallet = Wallet {
            provider,
            address,
            account_info,
            account_type,
            chains,
            tokens,
        };

        Ok(wallet)
    }

    /// Returns the wallet address.
    #[wasm_bindgen]
    pub fn address(&self) -> ZkLinkAddress {
        self.address.clone()
    }

    /// Returns the current account id
    #[wasm_bindgen]
    pub fn account_id(&self) -> AccountId {
        self.account_info.id
    }

    /// Returns the current account pub key hash
    #[wasm_bindgen]
    pub fn account_pubkey_hash(&self) -> PubKeyHash {
        self.account_info.pub_key_hash
    }

    #[wasm_bindgen]
    pub fn account_nonce(&self) -> Nonce {
        self.account_info.nonce
    }

    /// Updates account info stored in the wallet.
    #[wasm_bindgen]
    pub async fn update_account_info(&mut self) -> Result<(), JsValue> {
        self.account_info = self
            .provider
            .account_info(AccountQuery::Address(self.address.clone()))
            .await?;

        Ok(())
    }

    /// Returns `true` if signing key for account was set in zkLink network.
    /// In other words, returns `true` if `ChangePubKey` operation was performed for the
    /// account.
    ///
    /// If this method has returned `false`, one must send a `ChangePubKey` transaction
    /// via `Wallet::start_change_pubkey` method.
    #[wasm_bindgen]
    pub async fn is_signing_key_set(&self) -> Result<bool, JsValue> {
        let signer_pub_key_hash = self.signer.pub_key_hash;

        let key_set = self.account_pubkey_hash() == signer_pub_key_hash;
        Ok(key_set)
    }

    #[wasm_bindgen]
    pub fn get_chain(&self, chain_id: &ChainId) -> Option<ChainResp> {
        self.chains.get(chain_id).cloned()
    }

    #[wasm_bindgen]
    pub fn get_token(&self, token_id: &TokenId) -> Option<TokenResp> {
        self.tokens.get(token_id).cloned()
    }
}
