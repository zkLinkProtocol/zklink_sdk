use crate::do_submitter_signature;
use crate::error::SignError;
use crate::sign_change_pubkey::do_sign_change_pubkey_with_create2data_auth;
use crate::sign_forced_exit::sign_forced_exit;
use crate::sign_order_matching::sign_order_matching;
use crate::sign_transfer::sign_transfer;
use crate::sign_withdraw::sign_withdraw;
use zklink_sdk_signers::eth_signer::json_rpc_signer::{
    JsonRpcSigner as EthJsonRpcSigner, Provider,
};
use zklink_sdk_signers::zklink_signer::{ZkLinkSignature, ZkLinkSigner};
use zklink_sdk_types::prelude::PackedEthSignature;
use zklink_sdk_types::signatures::TxSignature;
use zklink_sdk_types::tx_type::change_pubkey::{ChangePubKey, ChangePubKeyAuthData, Create2Data};
use zklink_sdk_types::tx_type::forced_exit::ForcedExit;
use zklink_sdk_types::tx_type::order_matching::{Order, OrderMatching};
use zklink_sdk_types::tx_type::transfer::Transfer;
use zklink_sdk_types::tx_type::withdraw::Withdraw;
use zklink_sdk_types::tx_type::zklink_tx::ZkLinkTx;
use zklink_sdk_types::tx_type::ZkSignatureTrait;

pub struct JsonRpcSigner {
    zklink_signer: ZkLinkSigner,
    eth_signer: EthJsonRpcSigner,
}

impl JsonRpcSigner {
    pub fn new(provider: Provider) -> Result<Self, SignError> {
        let eth_json_rpc_signer = EthJsonRpcSigner::new(provider)?;
        let default_zklink_signer = ZkLinkSigner::new()?;
        Ok(Self {
            zklink_signer: default_zklink_signer,
            eth_signer: eth_json_rpc_signer,
        })
    }

    pub async fn init_zklink_signer(&mut self, signature: Option<String>) -> Result<(), SignError> {
        let signature = if let Some(s) = signature {
            Some(PackedEthSignature::from_hex(&s)?)
        } else {
            None
        };
        let zklink_signer =
            ZkLinkSigner::new_from_eth_rpc_signer(&self.eth_signer, signature).await?;
        self.zklink_signer = zklink_signer;
        Ok(())
    }

    pub async fn sign_transfer(
        &self,
        tx: Transfer,
        token_symbol: &str,
    ) -> Result<TxSignature, SignError> {
        sign_transfer(&self.eth_signer, &self.zklink_signer, tx, token_symbol).await
    }

    #[inline]
    pub fn sign_change_pubkey_with_create2data_auth(
        &self,
        tx: ChangePubKey,
        create2data: Create2Data,
    ) -> Result<TxSignature, SignError> {
        do_sign_change_pubkey_with_create2data_auth(tx, create2data, &self.zklink_signer)
    }

    #[inline]
    pub async fn sign_change_pubkey_with_eth_ecdsa_auth(
        &self,
        mut tx: ChangePubKey,
    ) -> Result<TxSignature, SignError> {
        tx.sign(&self.zklink_signer)?;
        let should_valid = tx.is_signature_valid();
        assert!(should_valid);

        // create auth data
        let eth_sign_msg = ChangePubKey::get_eth_sign_msg(&tx.new_pk_hash, tx.nonce, tx.account_id);
        let eth_signature = self
            .eth_signer
            .sign_message(eth_sign_msg.as_bytes())
            .await?;

        tx.eth_auth_data = ChangePubKeyAuthData::EthECDSA { eth_signature };

        Ok(TxSignature {
            tx: tx.into(),
            layer1_signature: None,
        })
    }

    pub async fn sign_withdraw(
        &self,
        tx: Withdraw,
        l2_source_token_symbol: &str,
    ) -> Result<TxSignature, SignError> {
        sign_withdraw(
            &self.eth_signer,
            &self.zklink_signer,
            tx,
            l2_source_token_symbol,
        )
        .await
    }

    #[inline]
    pub fn sign_forced_exit(&self, tx: ForcedExit) -> Result<TxSignature, SignError> {
        let signature = sign_forced_exit(&self.zklink_signer, tx)?;
        Ok(signature)
    }

    #[inline]
    pub fn create_signed_order(&self, order: &Order) -> Result<Order, SignError> {
        let signed_order = order.create_signed_order(&self.zklink_signer)?;
        Ok(signed_order)
    }

    #[inline]
    pub fn sign_order_matching(&self, tx: OrderMatching) -> Result<TxSignature, SignError> {
        let signature = sign_order_matching(&self.zklink_signer, tx)?;
        Ok(signature)
    }

    #[inline]
    pub fn submitter_signature(&self, zklink_tx: &ZkLinkTx) -> Result<ZkLinkSignature, SignError> {
        do_submitter_signature(&self.zklink_signer, zklink_tx)
    }
}
