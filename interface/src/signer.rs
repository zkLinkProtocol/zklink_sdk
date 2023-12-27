use crate::error::SignError;
use crate::sign_auto_deleveraging::sign_auto_deleveraging;
use crate::sign_forced_exit::sign_forced_exit;
use crate::sign_liquidation::sign_liquidation;
use crate::sign_transfer::{sign_eth_transfer, sign_starknet_transfer};
use crate::sign_withdraw::{sign_eth_withdraw, sign_starknet_withdraw};
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
use zklink_sdk_signers::starknet_signer::error::StarkSignerError;
use zklink_sdk_signers::starknet_signer::pk_signer::StarkSigner;
use zklink_sdk_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_sdk_signers::zklink_signer::public_key::PackedPublicKey;
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

pub enum Layer1Sginer {
    EthSigner(EthSigner),
    StarknetSigner(StarkSigner),
}

pub enum L1Type {
    Eth,
    Starknet,
}

pub struct Signer {
    zklink_signer: ZkLinkSigner,
    layer1_signer: Layer1Sginer,
}

pub enum L1SignerType {
    Eth,
    Starknet { chain_id: String, address: String },
}

impl Signer {
    pub fn new(private_key: &str, l1_signer_type: L1SignerType) -> Result<Self, SignError> {
        let (zklink_signer, layer1_signer) = match l1_signer_type {
            L1SignerType::Eth { .. } => {
                let eth_signer = EthSigner::try_from(private_key)
                    .map_err(|_| EthSignerError::InvalidEthSigner)?;
                (
                    ZkLinkSigner::new_from_hex_eth_signer(private_key)?,
                    Layer1Sginer::EthSigner(eth_signer),
                )
            }
            L1SignerType::Starknet {
                chain_id, address, ..
            } => {
                let stark_signer = StarkSigner::new_from_hex_str(private_key)
                    .map_err(|_| StarkSignerError::InvalidStarknetSigner)?;
                (
                    ZkLinkSigner::new_from_hex_stark_signer(private_key, &address, &chain_id)?,
                    Layer1Sginer::StarknetSigner(stark_signer),
                )
            }
        };
        Ok(Self {
            zklink_signer,
            layer1_signer,
        })
    }

    #[inline]
    pub fn pubkey_hash(&self) -> PubKeyHash {
        self.zklink_signer.public_key().public_key_hash()
    }

    #[inline]
    pub fn public_key(&self) -> PackedPublicKey {
        self.zklink_signer.public_key()
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
        if let Layer1Sginer::EthSigner(signer) = &self.layer1_signer {
            do_sign_change_pubkey_with_eth_ecdsa_auth(signer, &self.zklink_signer, tx)
        } else {
            Err(EthSignerError::InvalidEthSigner.into())
        }
    }

    #[cfg(not(feature = "web"))]
    pub fn sign_transfer(
        &self,
        tx: Transfer,
        token_symbol: &str,
        starknet_chain_id: Option<String>,
        starknet_addr: Option<String>,
    ) -> Result<TxSignature, SignError> {
        #[cfg(feature = "ffi")]
        let tx = (*tx).clone();
        match &self.layer1_signer {
            Layer1Sginer::EthSigner(signer) => {
                sign_eth_transfer(signer, &self.zklink_signer, tx, token_symbol)
            }
            Layer1Sginer::StarknetSigner(signer) => {
                let chain_id = starknet_chain_id.ok_or(SignError::StarkSigningError(
                    StarkSignerError::SignError("Invalid starknet_chain_id".to_string()),
                ))?;
                let addr = starknet_addr.ok_or(SignError::StarkSigningError(
                    StarkSignerError::SignError("Invalid starknet_addr".to_string()),
                ))?;
                sign_starknet_transfer(
                    signer,
                    &self.zklink_signer,
                    tx,
                    token_symbol,
                    &chain_id,
                    &addr,
                )
            }
        }
    }

    #[cfg(not(feature = "web"))]
    pub fn sign_withdraw(
        &self,
        tx: Withdraw,
        l2_source_token_symbol: &str,
        starknet_chain_id: Option<String>,
        starknet_addr: Option<String>,
    ) -> Result<TxSignature, SignError> {
        #[cfg(feature = "ffi")]
        let tx = (*tx).clone();
        match &self.layer1_signer {
            Layer1Sginer::EthSigner(signer) => {
                sign_eth_withdraw(signer, &self.zklink_signer, tx, l2_source_token_symbol)
            }
            Layer1Sginer::StarknetSigner(signer) => {
                let chain_id = starknet_chain_id.ok_or(SignError::StarkSigningError(
                    StarkSignerError::SignError("Invalid starknet_chain_id".to_string()),
                ))?;
                let addr = starknet_addr.ok_or(SignError::StarkSigningError(
                    StarkSignerError::SignError("Invalid starknet_addr".to_string()),
                ))?;
                sign_starknet_withdraw(
                    signer,
                    &self.zklink_signer,
                    tx,
                    l2_source_token_symbol,
                    &chain_id,
                    &addr,
                )
            }
        }
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
