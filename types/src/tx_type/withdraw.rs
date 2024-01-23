use num::{BigUint, ToPrimitive};
use serde::{Deserialize, Serialize};
#[cfg(feature = "ffi")]
use std::sync::Arc;
use validator::Validate;
use zklink_sdk_signers::eth_signer::packed_eth_signature::PackedEthSignature;
use zklink_sdk_signers::eth_signer::pk_signer::EthSigner;
use zklink_sdk_signers::zklink_signer::error::ZkSignerError;
use zklink_sdk_signers::zklink_signer::signature::ZkLinkSignature;
use zklink_sdk_utils::serde::BigUintSerdeAsRadix10Str;

use crate::basic_types::pack::pack_fee_amount;
use crate::basic_types::{
    AccountId, ChainId, GetBytes, Nonce, SubAccountId, TimeStamp, TokenId, ZkLinkAddress,
};
use crate::params::{ORDERS_BYTES, TOKEN_MAX_PRECISION};
#[cfg(feature = "ffi")]
use crate::prelude::WithdrawBuilder;
use crate::tx_type::validator::*;
use crate::tx_type::{
    ethereum_sign_message_part, starknet_sign_message_part, TxTrait, ZkSignatureTrait,
};
use zklink_sdk_signers::starknet_signer::typed_data::message::TxMessage;
use zklink_sdk_signers::zklink_signer::utils::rescue_hash_orders;

/// `Withdraw` transaction performs a withdrawal of funds from zklink account to L1 account.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Withdraw {
    /// Target chain of withdraw.
    #[validate(custom = "chain_id_validator")]
    pub to_chain_id: ChainId,
    /// zkLink network account ID of the transaction initiator.
    #[validate(custom = "account_validator")]
    pub account_id: AccountId,
    /// The source sub-account id of withdraw amount.
    #[validate(custom = "sub_account_validator")]
    pub sub_account_id: SubAccountId,
    /// Address of L1 account to withdraw funds to.
    #[validate(custom = "zklink_address_validator")]
    pub to: ZkLinkAddress,
    /// Source token and target token of withdrawal from l2 to l1.
    /// Also represents the token in which fee will be paid.
    #[validate(custom = "token_validator")]
    pub l2_source_token: TokenId,
    #[validate(custom = "token_validator")]
    pub l1_target_token: TokenId,
    /// Amount of funds to withdraw, layer1 can not unpack it, do not packaging
    #[serde(with = "BigUintSerdeAsRadix10Str")]
    #[validate(custom = "amount_unpackable")]
    pub amount: BigUint,
    /// Call data
    pub call_data: Option<Vec<u8>>,
    /// Fee for the transaction, need packaging
    #[serde(with = "BigUintSerdeAsRadix10Str")]
    #[validate(custom = "fee_packable")]
    pub fee: BigUint,
    /// Current account nonce.
    #[validate(custom = "nonce_validator")]
    pub nonce: Nonce,
    /// Transaction zkLink signature.
    #[serde(default)]
    pub signature: ZkLinkSignature,

    /// whether withdraw to layer1.
    #[validate(custom = "boolean_validator")]
    pub withdraw_to_l1: u8,
    /// If ratio is not zero, default fast withdraw.
    #[validate(custom = "rate_validator")]
    pub withdraw_fee_ratio: u16,
    /// Used as request id
    pub ts: TimeStamp,
}

impl Withdraw {
    #[cfg(feature = "ffi")]
    pub fn new(builder: WithdrawBuilder) -> Self {
        builder.build()
    }

    /// Get the first part of the message we expect to be signed by Ethereum account key.
    /// The only difference is the missing `nonce` since it's added at the end of the transactions
    /// batch message.
    pub fn get_eth_sign_message_part(&self, token_symbol: &str) -> String {
        ethereum_sign_message_part(
            "Withdraw",
            token_symbol,
            TOKEN_MAX_PRECISION,
            &self.amount,
            &self.fee,
            &self.to,
        )
    }

    /// Get message that should be signed by Ethereum keys of the account for 2-Factor authentication.
    pub fn get_eth_sign_msg(&self, token_symbol: &str) -> String {
        let mut message = self.get_eth_sign_message_part(token_symbol);
        if !message.is_empty() {
            message.push('\n');
        }
        message.push_str(format!("Nonce: {}", self.nonce).as_str());
        message
    }

    pub fn get_starknet_sign_msg(&self, token_symbol: &str) -> TxMessage {
        starknet_sign_message_part(
            "Withdraw",
            token_symbol,
            TOKEN_MAX_PRECISION,
            &self.amount,
            &self.fee,
            &self.to,
            self.nonce.to_string(),
        )
    }

    #[cfg(feature = "ffi")]
    pub fn eth_signature(
        &self,
        eth_signer: Arc<EthSigner>,
        l2_source_token_symbol: &str,
    ) -> Result<PackedEthSignature, ZkSignerError> {
        let message = self.get_eth_sign_msg(l2_source_token_symbol);
        let eth_signature = eth_signer.sign_message(message.as_bytes())?;
        Ok(eth_signature)
    }

    #[cfg(not(feature = "ffi"))]
    pub fn eth_signature(
        &self,
        eth_signer: &EthSigner,
        l2_source_token_symbol: &str,
    ) -> Result<PackedEthSignature, ZkSignerError> {
        let message = self.get_eth_sign_msg(l2_source_token_symbol);
        let eth_signature = eth_signer.sign_message(message.as_bytes())?;
        Ok(eth_signature)
    }
}

impl GetBytes for Withdraw {
    fn get_bytes(&self) -> Vec<u8> {
        let bytes_len = self.bytes_len();
        let mut tx_bytes = Vec::with_capacity(ORDERS_BYTES);
        tx_bytes.extend_from_slice(&[Self::TX_TYPE]);
        tx_bytes.extend_from_slice(&self.to_chain_id.to_be_bytes());
        tx_bytes.extend_from_slice(&self.account_id.to_be_bytes());
        tx_bytes.extend_from_slice(&self.sub_account_id.to_be_bytes());
        tx_bytes.extend_from_slice(&self.to.to_fixed_bytes());
        tx_bytes.extend_from_slice(&(*self.l2_source_token as u16).to_be_bytes());
        tx_bytes.extend_from_slice(&(*self.l1_target_token as u16).to_be_bytes());
        tx_bytes.extend_from_slice(&self.amount.to_u128().unwrap().to_be_bytes());
        tx_bytes.extend_from_slice(&pack_fee_amount(&self.fee));
        tx_bytes.extend_from_slice(&self.nonce.to_be_bytes());
        tx_bytes.push(self.withdraw_to_l1);
        tx_bytes.extend_from_slice(&self.withdraw_fee_ratio.to_be_bytes());
        tx_bytes.extend_from_slice(&self.ts.to_be_bytes());
        tx_bytes.resize(ORDERS_BYTES, 0);

        let mut out = Vec::with_capacity(bytes_len);
        out.extend(rescue_hash_orders(&tx_bytes));
        out.extend(
            self.call_data
                .as_ref()
                .map(ethers::utils::keccak256)
                .unwrap_or_default(),
        );
        assert_eq!(out.len(), bytes_len);
        out
    }

    fn bytes_len(&self) -> usize {
        63
    }
}

impl TxTrait for Withdraw {}

impl ZkSignatureTrait for Withdraw {
    fn set_signature(&mut self, signature: ZkLinkSignature) {
        self.signature = signature;
    }

    fn signature(&self) -> &ZkLinkSignature {
        &self.signature
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::WithdrawBuilder;
    use std::str::FromStr;

    #[test]
    fn test_get_bytes() {
        let address =
            ZkLinkAddress::from_str("0xAFAFf3aD1a0425D792432D9eCD1c3e26Ef2C42E9").unwrap();
        let ts = 1693472232u32;
        let builder = WithdrawBuilder {
            account_id: AccountId(10),
            sub_account_id: SubAccountId(1),
            to_chain_id: ChainId(1),
            to_address: address,
            l2_source_token: TokenId(18),
            l1_target_token: TokenId(18),
            amount: BigUint::from(10000u32),
            call_data: None,
            fee: BigUint::from(3u32),
            nonce: Nonce(1),
            withdraw_to_l1: false,
            withdraw_fee_ratio: 0,
            timestamp: ts.into(),
        };
        let withdraw = builder.build();
        let bytes = withdraw.get_bytes();
        let excepted_bytes = [
            35, 38, 100, 21, 162, 218, 169, 88, 46, 176, 84, 204, 61, 64, 69, 248, 70, 224, 44,
            240, 208, 221, 29, 8, 236, 225, 227, 255, 131, 200, 226, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        assert_eq!(bytes, excepted_bytes);
    }
}
