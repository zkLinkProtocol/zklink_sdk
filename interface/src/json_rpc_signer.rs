use crate::error::SignError;
use crate::sign_change_pubkey::check_create2data;
use crate::sign_forced_exit::sign_forced_exit;
use crate::sign_order::sign_order;
use crate::sign_order_matching::sign_order_matching;
use crate::sign_transfer::sign_transfer;
use crate::sign_withdraw::sign_withdraw;
use zklink_sdk_signers::eth_signer::json_rpc_signer::JsonRpcSigner as EthJsonRpcSigner;
use zklink_sdk_signers::zklink_signer::{ZkLinkSignature, ZkLinkSigner};
use zklink_sdk_types::basic_types::ZkLinkAddress;
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
    pub fn new() -> Result<Self, SignError> {
        let eth_json_rpc_signer = EthJsonRpcSigner::new()?;
        let default_zklink_signer = ZkLinkSigner::new()?;
        Ok(Self {
            zklink_signer: default_zklink_signer,
            eth_signer: eth_json_rpc_signer,
        })
    }

    pub async fn init_zklink_signer(&mut self) -> Result<(), SignError> {
        let zklink_signer = ZkLinkSigner::new_from_eth_json_rpc_signer(&self.eth_signer).await?;
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

    fn do_sign_change_pubkey_with_create2data_auth(
        &self,
        mut tx: ChangePubKey,
        create2data: Create2Data,
    ) -> Result<TxSignature, SignError> {
        tx.sign(&self.zklink_signer)?;
        let should_valid = tx.is_signature_valid();
        assert!(should_valid);

        // create onchain auth data
        tx.eth_auth_data = ChangePubKeyAuthData::EthCreate2 { data: create2data };
        Ok(TxSignature {
            tx: tx.into(),
            eth_signature: None,
        })
    }

    pub fn sign_change_pubkey_with_create2data_auth(
        &self,
        tx: ChangePubKey,
        create2data: Create2Data,
        from_account: ZkLinkAddress,
    ) -> Result<TxSignature, SignError> {
        check_create2data(&self.zklink_signer, create2data.clone(), from_account)?;
        self.do_sign_change_pubkey_with_create2data_auth(tx, create2data)
    }

    pub fn sign_change_pubkey_with_onchain_auth_data(
        &self,
        mut tx: ChangePubKey,
    ) -> Result<TxSignature, SignError> {
        tx.sign(&self.zklink_signer)?;
        let should_valid = tx.is_signature_valid();
        assert!(should_valid);
        // create onchain auth data
        tx.eth_auth_data = ChangePubKeyAuthData::Onchain;
        Ok(TxSignature {
            tx: tx.into(),
            eth_signature: None,
        })
    }

    pub async fn do_sign_change_pubkey_with_eth_ecdsa_auth(
        &self,
        mut tx: ChangePubKey,
        l1_client_id: u32,
        main_contract_address: ZkLinkAddress,
    ) -> Result<TxSignature, SignError> {
        tx.sign(&self.zklink_signer)?;
        let should_valid = tx.is_signature_valid();
        assert!(should_valid);

        // create auth data
        let typed_data = tx.to_eip712_request_payload(l1_client_id, &main_contract_address)?;
        let eth_signature = self
            .eth_signer
            .sign_message_eip712(typed_data.raw_data.as_bytes())
            .await?;

        tx.eth_auth_data = ChangePubKeyAuthData::EthECDSA { eth_signature };

        Ok(TxSignature {
            tx: tx.into(),
            eth_signature: None,
        })
    }

    pub async fn sign_change_pubkey_with_eth_ecdsa_auth(
        &self,
        tx: ChangePubKey,
        l1_client_id: u32,
        main_contract_address: ZkLinkAddress,
    ) -> Result<TxSignature, SignError> {
        self.do_sign_change_pubkey_with_eth_ecdsa_auth(tx, l1_client_id, main_contract_address)
            .await
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

    pub fn sign_forced_exit(&self, tx: ForcedExit) -> Result<TxSignature, SignError> {
        let signature = sign_forced_exit(&self.zklink_signer, tx)?;
        Ok(signature)
    }

    pub fn sign_order(&self, order: &Order) -> Result<Order, SignError> {
        let signed_order = sign_order(order, &self.zklink_signer)?;
        Ok(signed_order)
    }

    pub fn sign_order_matching(&self, tx: OrderMatching) -> Result<TxSignature, SignError> {
        let signature = sign_order_matching(&self.zklink_signer, tx)?;
        Ok(signature)
    }

    pub fn submitter_signature(&self, zklink_tx: &ZkLinkTx) -> Result<ZkLinkSignature, SignError> {
        let tx_hash = zklink_tx.tx_hash();
        let signature = self.zklink_signer.sign_musig(tx_hash.as_ref())?;
        Ok(signature)
    }
}
