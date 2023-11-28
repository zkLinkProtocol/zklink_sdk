use crate::error::SignError;
use crate::sign_auto_deleveraging::sign_auto_deleveraging;
use crate::sign_forced_exit::sign_forced_exit;
use crate::sign_liquidation::sign_liquidation;
use crate::sign_transfer::sign_transfer;
use crate::sign_withdraw::sign_withdraw;
use zklink_sdk_types::prelude::{PubKeyHash, TxSignature};

use crate::do_submitter_signature;
use crate::sign_change_pubkey::{
    do_sign_change_pubkey_with_create2data_auth, do_sign_change_pubkey_with_eth_ecdsa_auth,
    do_sign_change_pubkey_with_onchain_auth_data,
};
use crate::sign_contract_matching::sign_contract_matching;
use crate::sign_funding::sign_funding;
use crate::sign_order_matching::sign_order_matching;
use cfg_if::cfg_if;
#[cfg(feature = "ffi")]
use std::sync::Arc;
use zklink_sdk_signers::eth_signer::error::EthSignerError;
use zklink_sdk_signers::eth_signer::pk_signer::EthSigner;
use zklink_sdk_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_sdk_signers::zklink_signer::signature::ZkLinkSignature;
#[cfg(not(feature = "ffi"))]
use zklink_sdk_types::prelude::{Contract, GetBytes, Order};
use zklink_sdk_types::tx_type::change_pubkey::Create2Data;
use zklink_sdk_types::tx_type::zklink_tx::ZkLinkTx;

cfg_if! {
    if #[cfg(feature = "ffi")] {
        type ChangePubKey = Arc<zklink_sdk_types::prelude::ChangePubKey>;
        type Withdraw = Arc<zklink_sdk_types::prelude::Withdraw>;
        type Transfer = Arc<zklink_sdk_types::prelude::Transfer>;
        type OrderMatching = Arc<zklink_sdk_types::prelude::OrderMatching>;
        type ForcedExit = Arc<zklink_sdk_types::prelude::ForcedExit>;
        type AutoDeleveraging = Arc<zklink_sdk_types::prelude::AutoDeleveraging>;
        type ContractMatching = Arc<zklink_sdk_types::prelude::ContractMatching>;
        type Funding = Arc<zklink_sdk_types::prelude::Funding>;
        type Liquidation = Arc<zklink_sdk_types::prelude::Liquidation>;
    } else {
        use zklink_sdk_types::prelude::{AutoDeleveraging, ChangePubKey, Withdraw, Transfer, OrderMatching, ForcedExit,ContractMatching, Funding, Liquidation};
    }
}

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
    pub fn pubkey_hash(&self) -> PubKeyHash {
        self.zklink_signer.public_key().public_key_hash()
    }

    #[inline]
    pub fn sign_change_pubkey_with_create2data_auth(
        &self,
        tx: ChangePubKey,
        create2data: Create2Data,
    ) -> Result<TxSignature, SignError> {
        #[cfg(feature = "ffi")]
        let tx = (*tx).clone();
        do_sign_change_pubkey_with_create2data_auth(tx, create2data, &self.zklink_signer)
    }

    #[inline]
    pub fn sign_change_pubkey_with_onchain_auth_data(
        &self,
        tx: ChangePubKey,
    ) -> Result<TxSignature, SignError> {
        #[cfg(feature = "ffi")]
        let tx = (*tx).clone();
        do_sign_change_pubkey_with_onchain_auth_data(tx, &self.zklink_signer)
    }

    #[cfg(not(feature = "web"))]
    #[inline]
    pub fn sign_change_pubkey_with_eth_ecdsa_auth(
        &self,
        tx: ChangePubKey,
    ) -> Result<TxSignature, SignError> {
        #[cfg(feature = "ffi")]
        let tx = (*tx).clone();
        do_sign_change_pubkey_with_eth_ecdsa_auth(&self.eth_signer, &self.zklink_signer, tx)
    }

    #[cfg(not(feature = "web"))]
    pub fn sign_transfer(
        &self,
        tx: Transfer,
        token_symbol: &str,
    ) -> Result<TxSignature, SignError> {
        #[cfg(feature = "ffi")]
        let tx = (*tx).clone();
        sign_transfer(&self.eth_signer, &self.zklink_signer, tx, token_symbol)
    }

    #[cfg(not(feature = "web"))]
    pub fn sign_withdraw(
        &self,
        tx: Withdraw,
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

    pub fn sign_forced_exit(&self, tx: ForcedExit) -> Result<TxSignature, SignError> {
        #[cfg(feature = "ffi")]
        let tx = (*tx).clone();
        let signature = sign_forced_exit(&self.zklink_signer, tx)?;
        Ok(signature)
    }

    pub fn sign_order_matching(&self, tx: OrderMatching) -> Result<TxSignature, SignError> {
        #[cfg(feature = "ffi")]
        let tx = (*tx).clone();
        let signature = sign_order_matching(&self.zklink_signer, tx)?;
        Ok(signature)
    }

    pub fn sign_auto_deleveraging(&self, tx: AutoDeleveraging) -> Result<TxSignature, SignError> {
        #[cfg(feature = "ffi")]
        let tx = (*tx).clone();
        let signature = sign_auto_deleveraging(&self.zklink_signer, tx)?;
        Ok(signature)
    }

    pub fn sign_contract_matching(&self, tx: ContractMatching) -> Result<TxSignature, SignError> {
        #[cfg(feature = "ffi")]
        let tx = (*tx).clone();
        let signature = sign_contract_matching(&self.zklink_signer, tx)?;
        Ok(signature)
    }

    pub fn sign_funding(&self, tx: Funding) -> Result<TxSignature, SignError> {
        #[cfg(feature = "ffi")]
        let tx = (*tx).clone();
        let signature = sign_funding(&self.zklink_signer, tx)?;
        Ok(signature)
    }

    pub fn sign_liquidation(&self, tx: Liquidation) -> Result<TxSignature, SignError> {
        #[cfg(feature = "ffi")]
        let tx = (*tx).clone();
        let signature = sign_liquidation(&self.zklink_signer, tx)?;
        Ok(signature)
    }

    #[inline]
    #[cfg(not(feature = "ffi"))]
    pub fn create_signed_order(&self, order: &Order) -> Result<Order, SignError> {
        let mut order = order.clone();
        order.signature = self.zklink_signer.sign_musig(&order.get_bytes())?;
        Ok(order)
    }

    #[inline]
    #[cfg(not(feature = "ffi"))]
    pub fn create_signed_contract(&self, contract: &Contract) -> Result<Contract, SignError> {
        let mut contract = contract.clone();
        contract.signature = self.zklink_signer.sign_musig(&contract.get_bytes())?;
        Ok(contract)
    }

    #[inline]
    pub fn submitter_signature(&self, zklink_tx: &ZkLinkTx) -> Result<ZkLinkSignature, SignError> {
        do_submitter_signature(&self.zklink_signer, zklink_tx)
    }
}
