use crate::error::SignError;
use crate::sign_forced_exit::sign_forced_exit;
#[cfg(not(feature = "ffi"))]
use crate::sign_order::create_signed_order;
use crate::sign_order_matching::sign_order_matching;
use crate::sign_transfer::sign_transfer;
use crate::sign_withdraw::sign_withdraw;
use zklink_sdk_types::prelude::TxSignature;

use crate::do_submitter_signature;
use crate::sign_change_pubkey::{
    do_sign_change_pubkey_with_create2data_auth, do_sign_change_pubkey_with_eth_ecdsa_auth,
    do_sign_change_pubkey_with_onchain_auth_data,
};
#[cfg(feature = "ffi")]
use std::sync::Arc;
use zklink_sdk_signers::eth_signer::error::EthSignerError;
use zklink_sdk_signers::eth_signer::pk_signer::EthSigner;
use zklink_sdk_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_sdk_signers::zklink_signer::signature::ZkLinkSignature;
use zklink_sdk_types::basic_types::ZkLinkAddress;
use zklink_sdk_types::tx_type::change_pubkey::{ChangePubKey, Create2Data};
use zklink_sdk_types::tx_type::forced_exit::ForcedExit;
#[cfg(not(feature = "ffi"))]
use zklink_sdk_types::tx_type::order_matching::Order;
use zklink_sdk_types::tx_type::order_matching::OrderMatching;
use zklink_sdk_types::tx_type::transfer::Transfer;
use zklink_sdk_types::tx_type::withdraw::Withdraw;
use zklink_sdk_types::tx_type::zklink_tx::ZkLinkTx;

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

    #[inline]
    pub fn sign_change_pubkey_with_create2data_auth(
        &self,
        #[cfg(feature = "ffi")] tx: Arc<ChangePubKey>,
        #[cfg(not(feature = "ffi"))] tx: ChangePubKey,
        create2data: Create2Data,
    ) -> Result<TxSignature, SignError> {
        #[cfg(feature = "ffi")]
        let tx = (*tx).clone();
        do_sign_change_pubkey_with_create2data_auth(tx, create2data, &self.zklink_signer)
    }

    #[inline]
    pub fn sign_change_pubkey_with_onchain_auth_data(
        &self,
        #[cfg(feature = "ffi")] tx: Arc<ChangePubKey>,
        #[cfg(not(feature = "ffi"))] tx: ChangePubKey,
    ) -> Result<TxSignature, SignError> {
        #[cfg(feature = "ffi")]
        let tx = (*tx).clone();
        do_sign_change_pubkey_with_onchain_auth_data(tx, &self.zklink_signer)
    }

    #[cfg(not(feature = "web"))]
    #[inline]
    pub fn sign_change_pubkey_with_eth_ecdsa_auth(
        &self,
        #[cfg(feature = "ffi")] tx: Arc<ChangePubKey>,
        #[cfg(not(feature = "ffi"))] tx: ChangePubKey,
        l1_client_id: u32,
        main_contract_address: ZkLinkAddress,
    ) -> Result<TxSignature, SignError> {
        #[cfg(feature = "ffi")]
        let tx = (*tx).clone();
        do_sign_change_pubkey_with_eth_ecdsa_auth(
            &self.eth_signer,
            &self.zklink_signer,
            tx,
            l1_client_id,
            main_contract_address,
        )
    }

    #[cfg(not(feature = "web"))]
    pub fn sign_transfer(
        &self,
        #[cfg(feature = "ffi")] tx: Arc<Transfer>,
        #[cfg(not(feature = "ffi"))] tx: Transfer,
        token_symbol: &str,
    ) -> Result<TxSignature, SignError> {
        #[cfg(feature = "ffi")]
        let tx = (*tx).clone();
        sign_transfer(&self.eth_signer, &self.zklink_signer, tx, token_symbol)
    }

    #[cfg(not(feature = "web"))]
    pub fn sign_withdraw(
        &self,
        #[cfg(feature = "ffi")] tx: Arc<Withdraw>,
        #[cfg(not(feature = "ffi"))] tx: Withdraw,
        l2_source_token_symbol: &str,
    ) -> Result<TxSignature, SignError> {
        #[cfg(feature = "ffi")]
        let tx = (*tx).clone();
        sign_withdraw(
            &self.eth_signer,
            &self.zklink_signer,
            tx,
            l2_source_token_symbol,
        )
    }

    pub fn sign_forced_exit(
        &self,
        #[cfg(feature = "ffi")] tx: Arc<ForcedExit>,
        #[cfg(not(feature = "ffi"))] tx: ForcedExit,
    ) -> Result<TxSignature, SignError> {
        #[cfg(feature = "ffi")]
        let tx = (*tx).clone();
        let signature = sign_forced_exit(&self.zklink_signer, tx)?;
        Ok(signature)
    }

    #[inline]
    #[cfg(not(feature = "ffi"))]
    pub fn create_signed_order(&self, order: &Order) -> Result<Order, SignError> {
        let signed_order = create_signed_order(&self.zklink_signer, order)?;
        Ok(signed_order)
    }

    pub fn sign_order_matching(
        &self,
        #[cfg(feature = "ffi")] tx: Arc<OrderMatching>,
        #[cfg(not(feature = "ffi"))] tx: OrderMatching,
    ) -> Result<TxSignature, SignError> {
        #[cfg(feature = "ffi")]
        let tx = (*tx).clone();
        let signature = sign_order_matching(&self.zklink_signer, tx)?;
        Ok(signature)
    }

    #[inline]
    pub fn submitter_signature(&self, zklink_tx: &ZkLinkTx) -> Result<ZkLinkSignature, SignError> {
        do_submitter_signature(&self.zklink_signer, zklink_tx)
    }
}
