use crate::error::SignError;
use crate::sign_forced_exit::sign_forced_exit;
#[cfg(not(feature = "ffi"))]
use crate::sign_order::sign_order;
use crate::sign_order_matching::sign_order_matching;
use crate::sign_transfer::sign_transfer;
use crate::sign_withdraw::sign_withdraw;
use zklink_sdk_types::prelude::TxSignature;

#[cfg(not(feature = "ffi"))]
use crate::sign_change_pubkey::check_create2data;
#[cfg(feature = "ffi")]
use std::sync::Arc;
use zklink_sdk_signers::eth_signer::error::EthSignerError;
use zklink_sdk_signers::eth_signer::pk_signer::EthSigner;
use zklink_sdk_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_sdk_signers::zklink_signer::signature::ZkLinkSignature;
use zklink_sdk_types::basic_types::ZkLinkAddress;
use zklink_sdk_types::tx_type::change_pubkey::{ChangePubKey, ChangePubKeyAuthData, Create2Data};
use zklink_sdk_types::tx_type::forced_exit::ForcedExit;
#[cfg(not(feature = "ffi"))]
use zklink_sdk_types::tx_type::order_matching::Order;
use zklink_sdk_types::tx_type::order_matching::OrderMatching;
use zklink_sdk_types::tx_type::transfer::Transfer;
use zklink_sdk_types::tx_type::withdraw::Withdraw;
use zklink_sdk_types::tx_type::zklink_tx::ZkLinkTx;
use zklink_sdk_types::tx_type::ZkSignatureTrait;

pub struct Signer {
    zklink_signer: ZkLinkSigner,
    eth_signer: EthSigner,
}

impl Signer {
    pub fn new(private_key: &str) -> Result<Self, SignError> {
        let zklink_signer = ZkLinkSigner::new_from_hex_eth_signer(private_key)?;
        let eth_signer =
            EthSigner::try_from(private_key).map_err(|_| EthSignerError::InvalidEthSigner)?;
        Ok(Self {
            zklink_signer,
            eth_signer,
        })
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

    #[cfg(feature = "ffi")]
    #[inline]
    pub fn sign_change_pubkey_with_create2data_auth(
        &self,
        tx: Arc<ChangePubKey>,
        create2data: Create2Data,
    ) -> Result<TxSignature, SignError> {
        let tx = (*tx).clone();
        self.do_sign_change_pubkey_with_create2data_auth(tx, create2data)
    }

    #[cfg(not(feature = "ffi"))]
    #[inline]
    pub fn sign_change_pubkey_with_create2data_auth(
        &self,
        tx: ChangePubKey,
        create2data: Create2Data,
        from_account: ZkLinkAddress,
    ) -> Result<TxSignature, SignError> {
        check_create2data(&self.zklink_signer, create2data.clone(), from_account)?;
        self.do_sign_change_pubkey_with_create2data_auth(tx, create2data)
    }

    #[cfg(feature = "ffi")]
    pub fn sign_change_pubkey_with_onchain_auth_data(
        &self,
        tx: Arc<ChangePubKey>,
    ) -> Result<TxSignature, SignError> {
        let mut tx = (*tx).clone();
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

    #[cfg(not(feature = "ffi"))]
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

    fn do_sign_change_pubkey_with_eth_ecdsa_auth(
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
        let eth_signature = self.eth_signer.sign_hash(typed_data.data_hash.as_bytes())?;
        tx.eth_auth_data = ChangePubKeyAuthData::EthECDSA { eth_signature };

        Ok(TxSignature {
            tx: tx.into(),
            eth_signature: None,
        })
    }
    #[cfg(feature = "ffi")]
    #[inline]
    pub fn sign_change_pubkey_with_eth_ecdsa_auth(
        &self,
        tx: Arc<ChangePubKey>,
        l1_client_id: u32,
        main_contract_address: ZkLinkAddress,
    ) -> Result<TxSignature, SignError> {
        let tx = (*tx).clone();
        self.do_sign_change_pubkey_with_eth_ecdsa_auth(tx, l1_client_id, main_contract_address)
    }
    #[cfg(not(feature = "ffi"))]
    #[inline]
    pub fn sign_change_pubkey_with_eth_ecdsa_auth(
        &self,
        tx: ChangePubKey,
        l1_client_id: u32,
        main_contract_address: ZkLinkAddress,
    ) -> Result<TxSignature, SignError> {
        self.do_sign_change_pubkey_with_eth_ecdsa_auth(tx, l1_client_id, main_contract_address)
    }

    #[cfg(feature = "ffi")]
    pub fn sign_transfer(
        &self,
        tx: Arc<Transfer>,
        token_symbol: &str,
    ) -> Result<TxSignature, SignError> {
        let tx = (*tx).clone();
        sign_transfer(&self.eth_signer, &self.zklink_signer, tx, token_symbol)
    }

    #[cfg(not(feature = "ffi"))]
    pub fn sign_transfer(
        &self,
        tx: Transfer,
        token_symbol: &str,
    ) -> Result<TxSignature, SignError> {
        sign_transfer(&self.eth_signer, &self.zklink_signer, tx, token_symbol)
    }

    #[cfg(feature = "ffi")]
    pub fn sign_withdraw(
        &self,
        tx: Arc<Withdraw>,
        l2_source_token_symbol: &str,
    ) -> Result<TxSignature, SignError> {
        let tx = (*tx).clone();
        sign_withdraw(
            &self.eth_signer,
            &self.zklink_signer,
            tx,
            l2_source_token_symbol,
        )
    }

    #[cfg(not(feature = "ffi"))]
    pub fn sign_withdraw(
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
    }

    #[cfg(feature = "ffi")]
    pub fn sign_forced_exit(&self, tx: Arc<ForcedExit>) -> Result<TxSignature, SignError> {
        let tx = (*tx).clone();
        let signature = sign_forced_exit(&self.zklink_signer, tx)?;
        Ok(signature)
    }

    #[cfg(not(feature = "ffi"))]
    pub fn sign_forced_exit(&self, tx: ForcedExit) -> Result<TxSignature, SignError> {
        let signature = sign_forced_exit(&self.zklink_signer, tx)?;
        Ok(signature)
    }

    #[cfg(not(feature = "ffi"))]
    pub fn sign_order(&self, order: &Order) -> Result<Order, SignError> {
        let signed_order = sign_order(order, &self.zklink_signer)?;
        Ok(signed_order)
    }

    #[cfg(feature = "ffi")]
    pub fn sign_order_matching(&self, tx: Arc<OrderMatching>) -> Result<TxSignature, SignError> {
        let tx = (*tx).clone();
        let signature = sign_order_matching(&self.zklink_signer, tx)?;
        Ok(signature)
    }

    #[cfg(not(feature = "ffi"))]
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
