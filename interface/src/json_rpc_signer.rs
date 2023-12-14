use crate::do_submitter_signature;
use crate::error::SignError;
use crate::sign_change_pubkey::do_sign_change_pubkey_with_create2data_auth;
use crate::sign_forced_exit::sign_forced_exit;
use crate::sign_order_matching::sign_order_matching;
use crate::sign_transfer::{sign_eth_transfer, sign_starknet_transfer};
use crate::sign_withdraw::{sign_eth_withdraw, sign_starknet_withdraw};
use zklink_sdk_signers::eth_signer::json_rpc_signer::{
    JsonRpcSigner as EthJsonRpcSigner, Provider,
};
use zklink_sdk_signers::starknet_signer::starknet_json_rpc_signer::{
    StarknetJsonRpcSigner,Signer as StarknetAccountSigner
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
use zklink_sdk_signers::starknet_signer::StarkECDSASignature;
use zklink_sdk_signers::starknet_signer::error::StarkSignerError;

pub enum JsonRpcProvider {
    Provider(Provider),
    Signer(StarknetAccountSigner)
}
pub enum Layer1JsonRpcSigner {
    EthSigner(EthJsonRpcSigner),
    StarknetSigner(StarknetJsonRpcSigner),
}

pub struct JsonRpcSigner {
    zklink_signer: ZkLinkSigner,
    eth_signer: Layer1JsonRpcSigner,
}

impl JsonRpcSigner {
    pub fn new(provider: JsonRpcProvider,pub_key: Option<String>) -> Result<Self, SignError> {
        let eth_json_rpc_signer = match provider {
            JsonRpcProvider::Provider(provider) =>
                Layer1JsonRpcSigner::EthSigner(EthJsonRpcSigner::new(provider)),
            JsonRpcProvider::Signer(signer) =>
                Layer1JsonRpcSigner::StarknetSigner(StarknetJsonRpcSigner::new(signer,pub_key.unwrap()))
        };
        let default_zklink_signer = ZkLinkSigner::new()?;
        Ok(Self {
            zklink_signer: default_zklink_signer,
            eth_signer: eth_json_rpc_signer,
        })
    }

    pub async fn init_zklink_signer(&mut self, signature: Option<String>) -> Result<(), SignError> {
        let zklink_signer = if let Some(s) = signature {
            match &self.eth_signer {
                Layer1JsonRpcSigner::EthSigner(_) => {
                    let signature = PackedEthSignature::from_hex(&s)?;
                    let seed = signature.serialize_packed();
                    ZkLinkSigner::new_from_seed(&seed)?
                },
                Layer1JsonRpcSigner::StarknetSigner(_) => {
                    let signature = StarkECDSASignature::from_hex(&s)?;
                    let seed = signature.signature.to_bytes_be();
                    ZkLinkSigner::new_from_seed(&seed)?
                }
            }
        } else  {
            match &self.eth_signer {
                Layer1JsonRpcSigner::EthSigner(signer) =>
                    ZkLinkSigner::new_from_eth_rpc_signer(signer).await?,
                Layer1JsonRpcSigner::StarknetSigner(signer) =>
                    ZkLinkSigner::new_from_starknet_rpc_signer(signer).await?,
            }
        };
        self.zklink_signer = zklink_signer;
        Ok(())
    }

    pub async fn sign_transfer(
        &self,
        tx: Transfer,
        token_symbol: &str,
    ) -> Result<TxSignature, SignError> {
        match &self.eth_signer {
            Layer1JsonRpcSigner::EthSigner(signer) =>
                sign_eth_transfer(signer, &self.zklink_signer, tx, token_symbol).await,
            Layer1JsonRpcSigner::StarknetSigner(signer) =>
                sign_starknet_transfer(signer, &self.zklink_signer, tx, token_symbol).await,
        }
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
        let eth_signature = match &self.eth_signer {
            Layer1JsonRpcSigner::EthSigner(signer) =>
                signer.sign_message(eth_sign_msg.as_bytes()).await?,
            Layer1JsonRpcSigner::StarknetSigner(_) => {
                //starknet only support change_pubkey_onchain
                return Err(StarkSignerError::SignError("Not support for starknet".to_string()).into());
            }
        };

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
        match &self.eth_signer {
            Layer1JsonRpcSigner::EthSigner(signer) =>
                sign_eth_withdraw(signer, &self.zklink_signer, tx, l2_source_token_symbol).await,
            Layer1JsonRpcSigner::StarknetSigner(signer) =>
                sign_starknet_withdraw(signer, &self.zklink_signer, tx, l2_source_token_symbol).await,
        }
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
