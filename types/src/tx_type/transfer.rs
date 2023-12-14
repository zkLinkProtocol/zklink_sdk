use crate::basic_types::pack::{pack_fee_amount, pack_token_amount};
use crate::basic_types::{
    AccountId, GetBytes, Nonce, SubAccountId, TimeStamp, TokenId, ZkLinkAddress,
};
use crate::tx_type::validator::*;
use crate::tx_type::{ethereum_sign_message_part, TxTrait, ZkSignatureTrait, starknet_sign_message_part};

use crate::params::{SIGNED_TRANSFER_BIT_WIDTH, TOKEN_MAX_PRECISION, TX_TYPE_BIT_WIDTH};
#[cfg(feature = "ffi")]
use crate::prelude::TransferBuilder;
use crate::signatures::TxLayer1Signature;
use num::BigUint;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ffi")]
use std::sync::Arc;
use validator::Validate;
use zklink_sdk_signers::eth_signer::pk_signer::EthSigner;
use zklink_sdk_signers::starknet_signer::error::StarkSignerError;
use zklink_sdk_signers::starknet_signer::StarkSigner;
use zklink_sdk_signers::zklink_signer::error::ZkSignerError;
use zklink_sdk_signers::zklink_signer::signature::ZkLinkSignature;
use zklink_sdk_utils::serde::BigUintSerdeAsRadix10Str;
use zklink_sdk_signers::starknet_signer::typed_data::message::TxMessage;

/// `Transfer` transaction performs a move of funds from one zklink account to another.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Transfer {
    /// zklink network account ID of the transaction initiator.
    #[validate(custom = "account_validator")]
    pub account_id: AccountId,
    /// zklink network sub-account ID of the transaction initiator.
    #[validate(custom = "sub_account_validator")]
    pub from_sub_account_id: SubAccountId,
    /// zklink network sub-account ID of the transaction initiator.
    #[validate(custom = "sub_account_validator")]
    pub to_sub_account_id: SubAccountId,
    /// Layer1 address of account to transfer funds to.
    #[validate(custom = "zklink_address_validator")]
    pub to: ZkLinkAddress,
    /// Type of token for transfer. Also represents the token in which fee will be paid.
    #[validate(custom = "token_validator")]
    pub token: TokenId,
    /// Amount of funds to transfer, layer1 need unpack it, do packaging
    #[serde(with = "BigUintSerdeAsRadix10Str")]
    #[validate(custom = "amount_packable")]
    pub amount: BigUint,
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

    /// Used as request id
    pub ts: TimeStamp,
}

impl Transfer {
    #[cfg(feature = "ffi")]
    pub fn new(builder: TransferBuilder) -> Self {
        builder.build()
    }

    /// Get the first part of the message we expect to be signed by Ethereum account key.
    /// The only difference is the missing `nonce` since it's added at the end of the transactions
    /// batch message.
    pub fn get_eth_sign_msg_part(&self, token_symbol: &str) -> String {
        ethereum_sign_message_part(
            "Transfer",
            token_symbol,
            TOKEN_MAX_PRECISION,
            &self.amount,
            &self.fee,
            &self.to,
        )
    }

    /// Gets message that should be signed by Ethereum keys of the account for 2-Factor authentication.
    pub fn get_eth_sign_msg(&self, token_symbol: &str) -> String {
        let mut message = self.get_eth_sign_msg_part(token_symbol);
        if !message.is_empty() {
            message.push('\n');
        }
        message.push_str(format!("Nonce: {}", self.nonce).as_str());
        message
    }

    pub fn get_starknet_sign_msg(&self,token_symbol: &str) -> TxMessage {
        starknet_sign_message_part(
            "Transfer",
            token_symbol,
            TOKEN_MAX_PRECISION,
            &self.amount,
            &self.fee,
            &self.to,
            self.nonce.into(),
        )
    }

    #[cfg(not(feature = "ffi"))]
    pub fn eth_signature(
        &self,
        eth_signer: &EthSigner,
        token_symbol: &str,
    ) -> Result<TxLayer1Signature, ZkSignerError> {
        let message = self.get_eth_sign_msg(token_symbol);
        let eth_signature = eth_signer.sign_message(message.as_bytes())?;
        let tx_eth_signature = TxLayer1Signature::EthereumSignature(eth_signature);
        Ok(tx_eth_signature)
    }

    #[cfg(feature = "ffi")]
    pub fn eth_signature(
        &self,
        eth_signer: Arc<EthSigner>,
        token_symbol: &str,
    ) -> Result<TxLayer1Signature, ZkSignerError> {
        let message = self.get_eth_sign_msg(token_symbol);
        let eth_signature = eth_signer.sign_message(message.as_bytes())?;
        let tx_eth_signature = TxLayer1Signature::EthereumSignature(eth_signature);
        Ok(tx_eth_signature)
    }

    #[cfg(not(feature = "ffi"))]
    pub fn starknet_signature(
        &self,
        starknet_signer: &StarkSigner,
        token_symbol: &str,
    ) -> Result<TxLayer1Signature, StarkSignerError> {
        //todo: use eip712
        //let message = self.get_starknet_sign_msg(token_symbol);
        let message = self.get_eth_sign_msg(token_symbol);
        let signature = starknet_signer.sign_message(message.as_bytes())?;
        let tx_eth_signature = TxLayer1Signature::StarkSignature(signature);
        Ok(tx_eth_signature)
    }

    #[cfg(feature = "ffi")]
    pub fn starknet_signature(
        &self,
        starknet_signer: Arc<StarkSigner>,
        token_symbol: &str,
    ) -> Result<TxLayer1Signature, StarkSignerError> {
        let message = self.get_eth_sign_msg(token_symbol);
        let signature = starknet_signer.sign_message(message.as_bytes())?;
        let tx_eth_signature = TxLayer1Signature::StarkSignature(signature);
        Ok(tx_eth_signature)
    }
}

impl GetBytes for Transfer {
    fn get_bytes(&self) -> Vec<u8> {
        let bytes_len = self.bytes_len();
        let mut out = Vec::with_capacity(bytes_len);
        out.extend_from_slice(&[Self::TX_TYPE]);
        out.extend_from_slice(&self.account_id.to_be_bytes());
        out.extend_from_slice(&self.from_sub_account_id.to_be_bytes());
        out.extend_from_slice(&self.to.to_fixed_bytes());
        out.extend_from_slice(&self.to_sub_account_id.to_be_bytes());
        out.extend_from_slice(&(*self.token as u16).to_be_bytes());
        out.extend_from_slice(&pack_token_amount(&self.amount));
        out.extend_from_slice(&pack_fee_amount(&self.fee));
        out.extend_from_slice(&self.nonce.to_be_bytes());
        out.extend_from_slice(&self.ts.to_be_bytes());
        assert_eq!(out.len(), bytes_len);
        out
    }
    fn bytes_len(&self) -> usize {
        SIGNED_TRANSFER_BIT_WIDTH / TX_TYPE_BIT_WIDTH
    }
}

impl TxTrait for Transfer {}

impl ZkSignatureTrait for Transfer {
    fn set_signature(&mut self, signature: ZkLinkSignature) {
        self.signature = signature
    }

    fn signature(&self) -> &ZkLinkSignature {
        &self.signature
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::TransferBuilder;
    use std::str::FromStr;
    use zklink_sdk_signers::eth_signer::packed_eth_signature::PackedEthSignature;
    use zklink_sdk_signers::eth_signer::pk_signer::EthSigner;
    use zklink_sdk_signers::zklink_signer::public_key::PackedPublicKey;

    #[test]
    fn test_get_bytes() {
        let address =
            ZkLinkAddress::from_str("0xAFAFf3aD1a0425D792432D9eCD1c3e26Ef2C42E9").unwrap();
        let ts = 1693472232u32;
        let builder = TransferBuilder {
            account_id: AccountId(10),
            to_address: address,
            from_sub_account_id: SubAccountId(1),
            to_sub_account_id: SubAccountId(1),
            token: TokenId(18),
            amount: BigUint::from(10000u32),
            fee: BigUint::from(3u32),
            nonce: Nonce(1),
            timestamp: ts.into(),
        };
        let transfer = builder.build();
        let bytes = transfer.get_bytes();
        let excepted_bytes = [
            4, 0, 0, 0, 10, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 175, 175, 243, 173, 26, 4, 37,
            215, 146, 67, 45, 158, 205, 28, 62, 38, 239, 44, 66, 233, 1, 0, 18, 0, 0, 4, 226, 0, 0,
            96, 0, 0, 0, 1, 100, 240, 85, 232,
        ];
        assert_eq!(bytes, excepted_bytes);
    }

    #[test]
    fn test_verify_signature() {
        let private_key_str = "0xbe725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
        let ts = 1693472232;
        let eth_signature = "0x1f11707e54773e059bc38aa73526fe2b51af9b89a77df731af7bcc429750d0317727a857efda5d79232eb5f9a66ed60a79aad2195d4de1375f5021c0db041b221b";
        let signature = "0x7b173e25e484eed3461091430f81b2a5bd7ae792f69701dcb073cb903f8125107ecbe23c307d18007ee43090940a4a43bd02bdcda206ad695f745c2f0a64f4ac4c4c8beb9ed9cbdd0e523e75ffc7dedd0281da4946bb37fa26a04283bd480a04";
        let public_key_str = "0x7b173e25e484eed3461091430f81b2a5bd7ae792f69701dcb073cb903f812510";
        let address =
            ZkLinkAddress::from_str("0xAFAFf3aD1a0425D792432D9eCD1c3e26Ef2C42E9").unwrap();
        let builder = TransferBuilder {
            account_id: AccountId(1),
            to_address: address,
            from_sub_account_id: SubAccountId(1),
            to_sub_account_id: SubAccountId(1),
            token: TokenId(18),
            amount: BigUint::from_str("100000").unwrap(),
            fee: BigUint::from_str("100").unwrap(),
            nonce: Nonce(1),
            timestamp: ts.into(),
        };

        //check l2 signature
        let mut tx = builder.build();
        tx.signature = ZkLinkSignature::from_hex(signature).unwrap();
        let recover_pubkey_hash = tx.verify_signature().unwrap();
        let pubkey = PackedPublicKey::from_hex(public_key_str).unwrap();
        let pubkey_hash = pubkey.public_key_hash();
        assert_eq!(pubkey_hash, recover_pubkey_hash);

        //check eth signature
        let l1_signature = PackedEthSignature::from_hex(eth_signature).unwrap();
        let token_symbol = "USDC";
        let message = tx.get_eth_sign_msg(token_symbol).as_bytes().to_vec();
        let recover_address = l1_signature.signature_recover_signer(&message).unwrap();
        let private_key = EthSigner::try_from(private_key_str).unwrap();
        let address = private_key.get_address();
        assert_eq!(address, recover_address);

        // check starknet signature
        let private_key_str = "0x02c5dbad71c92a45cc4b40573ae661f8147869a91d57b8d9b8f48c8af7f83159";
        let private_key = StarkSigner::new_from_hex_str(private_key_str).unwrap();
        let message = tx.get_starknet_sign_msg();
        let signature = private_key.sign_message(&message).unwrap();
        let is_ok = signature.verify(&message).unwrap();
        assert!(is_ok);
        let starknet_signature = "0x0226424352505249f119fd6913430aad28321afcff18036139b6fa77c4fad6cc064a51bbd082ecb0956589f6089ee24dd9ec32b083f5d56352ca94e5c51d3aa504b98450aa2cc9f9f1f5c0ab47dc28ccff41f7661b6e20181e8204ada6213e1e";
        assert_eq!(signature.as_hex(), starknet_signature);
    }
}
