use crate::error::SignError;
use crate::sign_change_pubkey::{
    do_sign_change_pubkey_with_create2data_auth, do_sign_change_pubkey_with_onchain_auth_data,
};
use crate::sign_forced_exit::sign_forced_exit;
use crate::sign_order_matching::sign_order_matching;
use crate::sign_transfer::{sign_eth_transfer, sign_starknet_transfer};
use crate::sign_withdraw::{sign_eth_withdraw, sign_starknet_withdraw};
use zklink_sdk_signers::eth_signer::json_rpc_signer::{
    JsonRpcSigner as EthJsonRpcSigner, Provider,
};
use zklink_sdk_signers::starknet_signer::starknet_json_rpc_signer::{
    Signer as StarknetAccountSigner, StarknetJsonRpcSigner,
};

use crate::sign_auto_deleveraging::sign_auto_deleveraging;
use crate::sign_contract_matching::sign_contract_matching;
use crate::sign_funding::sign_funding;
use crate::sign_liquidation::sign_liquidation;
use zklink_sdk_signers::starknet_signer::error::StarkSignerError;
use zklink_sdk_signers::starknet_signer::StarkEip712Signature;
use zklink_sdk_signers::zklink_signer::ZkLinkSigner;
use zklink_sdk_types::basic_types::GetBytes;
use zklink_sdk_types::prelude::{PackedEthSignature, ZkLinkSignature};
use zklink_sdk_types::signatures::TxSignature;
use zklink_sdk_types::tx_type::change_pubkey::{ChangePubKey, ChangePubKeyAuthData, Create2Data};
use zklink_sdk_types::tx_type::contract::{
    AutoDeleveraging, Contract, ContractMatching, Funding, Liquidation,
};
use zklink_sdk_types::tx_type::forced_exit::ForcedExit;
use zklink_sdk_types::tx_type::order_matching::{Order, OrderMatching};
use zklink_sdk_types::tx_type::transfer::Transfer;
use zklink_sdk_types::tx_type::withdraw::Withdraw;
use zklink_sdk_types::tx_type::ZkSignatureTrait;

pub enum JsonRpcProvider {
    Provider(Provider),
    Signer(StarknetAccountSigner),
}
pub enum Layer1JsonRpcSigner {
    EthSigner(EthJsonRpcSigner),
    StarknetSigner(StarknetJsonRpcSigner),
}

pub struct JsonRpcSigner {
    zklink_signer: ZkLinkSigner,
    layer1_signer: Layer1JsonRpcSigner,
    signature_seed: Vec<u8>,
}

impl JsonRpcSigner {
    pub fn new(
        provider: JsonRpcProvider,
        pub_key: Option<String>,
        chain_id: Option<String>,
    ) -> Result<Self, SignError> {
        let eth_json_rpc_signer = match provider {
            JsonRpcProvider::Provider(provider) => {
                Layer1JsonRpcSigner::EthSigner(EthJsonRpcSigner::new(provider))
            }
            JsonRpcProvider::Signer(signer) => Layer1JsonRpcSigner::StarknetSigner(
                StarknetJsonRpcSigner::new(signer, pub_key.unwrap(), chain_id.unwrap()),
            ),
        };
        let default_zklink_signer = ZkLinkSigner::new()?;
        Ok(Self {
            zklink_signer: default_zklink_signer,
            layer1_signer: eth_json_rpc_signer,
            signature_seed: vec![],
        })
    }

    pub async fn init_zklink_signer(&mut self, signature: Option<String>) -> Result<(), SignError> {
        let (zklink_signer, seed) = if let Some(s) = signature {
            match &self.layer1_signer {
                Layer1JsonRpcSigner::EthSigner(_) => {
                    let signature = PackedEthSignature::from_hex(&s)?;
                    let seed = signature.serialize_packed();
                    (ZkLinkSigner::new_from_seed(&seed)?, seed.to_vec())
                }
                Layer1JsonRpcSigner::StarknetSigner(_) => {
                    let signature = StarkEip712Signature::from_hex(&s)?;
                    let seed = signature.signature.to_bytes_be();
                    (ZkLinkSigner::new_from_seed(&seed)?, seed.to_vec())
                }
            }
        } else {
            match &self.layer1_signer {
                Layer1JsonRpcSigner::EthSigner(signer) => {
                    ZkLinkSigner::new_from_eth_rpc_signer(signer).await?
                }
                Layer1JsonRpcSigner::StarknetSigner(signer) => {
                    ZkLinkSigner::new_from_starknet_rpc_signer(signer).await?
                }
            }
        };
        self.zklink_signer = zklink_signer;
        self.signature_seed = seed;
        Ok(())
    }

    pub fn pub_key_hash(&self) -> String {
        let pub_key = self.zklink_signer.public_key();
        pub_key.public_key_hash().as_hex()
    }

    pub fn public_key(&self) -> String {
        let pub_key = self.zklink_signer.public_key();
        pub_key.as_hex()
    }

    pub fn address(&self) -> Option<String> {
        match &self.layer1_signer {
            Layer1JsonRpcSigner::EthSigner(s) => s.address(),
            Layer1JsonRpcSigner::StarknetSigner(s) => Some(s.address()),
        }
    }

    pub fn signature_seed(&self) -> Vec<u8> {
        self.signature_seed.clone()
    }

    pub async fn sign_transfer(
        &self,
        tx: Transfer,
        token_symbol: &str,
    ) -> Result<TxSignature, SignError> {
        match &self.layer1_signer {
            Layer1JsonRpcSigner::EthSigner(signer) => {
                sign_eth_transfer(signer, &self.zklink_signer, tx, token_symbol).await
            }
            Layer1JsonRpcSigner::StarknetSigner(signer) => {
                sign_starknet_transfer(signer, &self.zklink_signer, tx, token_symbol).await
            }
        }
    }

    #[inline]
    pub fn sign_change_pubkey_with_onchain_auth_data(
        &self,
        tx: ChangePubKey,
    ) -> Result<TxSignature, SignError> {
        do_sign_change_pubkey_with_onchain_auth_data(tx, &self.zklink_signer)
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
        let eth_signature = match &self.layer1_signer {
            Layer1JsonRpcSigner::EthSigner(signer) => {
                signer.sign_message(eth_sign_msg.as_bytes()).await?
            }
            Layer1JsonRpcSigner::StarknetSigner(_) => {
                //starknet only support change_pubkey_onchain
                return Err(
                    StarkSignerError::SignError("Not support for starknet".to_string()).into(),
                );
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
        match &self.layer1_signer {
            Layer1JsonRpcSigner::EthSigner(signer) => {
                sign_eth_withdraw(signer, &self.zklink_signer, tx, l2_source_token_symbol).await
            }
            Layer1JsonRpcSigner::StarknetSigner(signer) => {
                sign_starknet_withdraw(signer, &self.zklink_signer, tx, l2_source_token_symbol)
                    .await
            }
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

    pub fn sign_auto_deleveraging(&self, tx: AutoDeleveraging) -> Result<TxSignature, SignError> {
        let signature = sign_auto_deleveraging(&self.zklink_signer, tx)?;
        Ok(signature)
    }

    pub fn sign_contract_matching(&self, tx: ContractMatching) -> Result<TxSignature, SignError> {
        let signature = sign_contract_matching(&self.zklink_signer, tx)?;
        Ok(signature)
    }

    pub fn sign_funding(&self, tx: Funding) -> Result<TxSignature, SignError> {
        let signature = sign_funding(&self.zklink_signer, tx)?;
        Ok(signature)
    }

    pub fn sign_liquidation(&self, tx: Liquidation) -> Result<TxSignature, SignError> {
        let signature = sign_liquidation(&self.zklink_signer, tx)?;
        Ok(signature)
    }

    #[inline]
    pub fn create_signed_contract(&self, contract: &Contract) -> Result<Contract, SignError> {
        let mut contract = contract.clone();
        contract.signature = self.zklink_signer.sign_musig(&contract.get_bytes())?;
        Ok(contract)
    }

    #[inline]
    pub fn sign_musig(&self, msg: Vec<u8>) -> Result<ZkLinkSignature, SignError> {
        Ok(self.zklink_signer.sign_musig(&msg)?)
    }

    pub fn get_zklink_signer(&self) -> ZkLinkSigner {
        self.zklink_signer.clone()
    }
}
