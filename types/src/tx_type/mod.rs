use crate::basic_types::{GetBytes, ZkLinkAddress};
#[cfg(feature = "ffi")]
use crate::prelude::ZkLinkTx;
use crate::tx_type::change_pubkey::ChangePubKey;
use crate::tx_type::deposit::Deposit;
use crate::tx_type::forced_exit::ForcedExit;
use crate::tx_type::full_exit::FullExit;
use crate::tx_type::order_matching::OrderMatching;
use crate::tx_type::transfer::Transfer;
use crate::tx_type::withdraw::Withdraw;
use ::validator::Validate;
use num::{BigUint, Zero};
use serde::Serialize;
use std::collections::VecDeque;
#[cfg(feature = "ffi")]
use std::sync::Arc;
use zklink_sdk_signers::starknet_signer::typed_data::message::TxMessage;
use zklink_sdk_signers::zklink_signer::error::ZkSignerError;
use zklink_sdk_signers::zklink_signer::pk_signer::{sha256_bytes, ZkLinkSigner};
use zklink_sdk_signers::zklink_signer::signature::ZkLinkSignature;
use zklink_sdk_signers::zklink_signer::PubKeyHash;
use zklink_tx::ZkLinkTxType;

pub mod validator;

pub mod change_pubkey;
pub mod contract;
pub mod deposit;
pub mod exit_info;
pub mod forced_exit;
pub mod full_exit;
pub mod order_matching;
pub mod transfer;
pub mod withdraw;
pub mod zklink_tx;

impl Deposit {
    pub const TX_TYPE: u8 = ZkLinkTxType::Deposit as u8;
}
impl Withdraw {
    pub const TX_TYPE: u8 = ZkLinkTxType::Withdraw as u8;
}
impl Transfer {
    pub const TX_TYPE: u8 = ZkLinkTxType::Transfer as u8;
}
impl FullExit {
    pub const TX_TYPE: u8 = ZkLinkTxType::FullExit as u8;
}
impl ChangePubKey {
    pub const TX_TYPE: u8 = ZkLinkTxType::ChangePubKey as u8;
}
impl ForcedExit {
    pub const TX_TYPE: u8 = ZkLinkTxType::ForcedExit as u8;
}
impl OrderMatching {
    pub const TX_TYPE: u8 = ZkLinkTxType::OrderMatching as u8;
}

/// Construct the first part of the message that should be signed by Ethereum key.
/// The pattern is as follows:
///
/// [{Transfer/Withdraw} {amount} {token} to: {to_address}]
/// [Fee: {fee} {token}]
///
/// Note that both lines are optional.
pub fn ethereum_sign_message_part(
    transaction: &str,
    token_symbol: &str,
    decimals: u8,
    amount: &BigUint,
    fee: &BigUint,
    to: &ZkLinkAddress,
) -> String {
    let mut message = if !amount.is_zero() {
        format!(
            "{transaction} {amount} {token} to: {to}",
            transaction = transaction,
            amount = format_units(amount, decimals),
            token = token_symbol,
            to = to.to_string()
        )
    } else {
        String::new()
    };
    if !fee.is_zero() {
        if !message.is_empty() {
            message.push('\n');
        }
        message.push_str(
            format!(
                "Fee: {fee} {token}",
                fee = format_units(fee, decimals),
                token = token_symbol
            )
            .as_str(),
        );
    }
    message
}

pub fn starknet_sign_message_part(
    transaction: &str,
    token_symbol: &str,
    decimals: u8,
    amount: &BigUint,
    fee: &BigUint,
    to: &ZkLinkAddress,
    nonce: String,
) -> TxMessage {
    TxMessage {
        transaction: transaction.to_string(),
        amount: format_units(amount, decimals),
        token: token_symbol.to_string(),
        fee: format_units(fee, decimals),
        to: to.to_string(),
        nonce,
    }
}

/// Formats amount in wei to tokens with precision.
/// Behaves just like ethers.utils.formatUnits
pub fn format_units(wei: impl ToString, units: u8) -> String {
    let mut chars: VecDeque<char> = wei.to_string().chars().collect();

    while chars.len() < units as usize {
        chars.push_front('0');
    }
    chars.insert(chars.len() - units as usize, '.');
    if *chars.front().unwrap() == '.' {
        chars.push_front('0');
    }
    while *chars.back().unwrap() == '0' {
        chars.pop_back();
    }
    if *chars.back().unwrap() == '.' {
        chars.push_back('0');
    }
    chars.iter().collect()
}

pub trait TxTrait: Validate + Serialize + GetBytes {
    fn tx_hash(&self) -> Vec<u8> {
        let bytes = self.get_bytes();
        sha256_bytes(&bytes)
    }

    #[cfg(feature = "ffi")]
    fn json_str(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }
}

#[cfg(feature = "ffi")]
pub trait ToZklinkTx {
    fn to_zklink_tx(&self) -> ZkLinkTx;
}

#[cfg(feature = "ffi")]
impl<T> ToZklinkTx for T
where
    ZkLinkTx: From<T>,
    T: Clone,
{
    fn to_zklink_tx(&self) -> ZkLinkTx
    where
        Self: Sized,
    {
        self.clone().into()
    }
}

pub trait ZkSignatureTrait: TxTrait {
    fn set_signature(&mut self, signature: ZkLinkSignature);

    #[cfg(feature = "ffi")]
    fn get_signature(&self) -> ZkLinkSignature {
        self.signature().clone()
    }

    fn signature(&self) -> &ZkLinkSignature;

    fn verify_signature(&self) -> Option<PubKeyHash> {
        let signature = self.signature();
        signature
            .verify_musig(&self.get_bytes())
            .then(|| signature.pub_key.public_key_hash())
    }

    fn is_signature_valid(&self) -> bool {
        let bytes = self.get_bytes();
        self.signature().verify_musig(&bytes)
    }

    fn sign(&mut self, signer: &ZkLinkSigner) -> Result<(), ZkSignerError> {
        let bytes = self.get_bytes();
        let signature = signer.sign_musig(&bytes)?;
        self.set_signature(signature);
        Ok(())
    }

    #[cfg(feature = "ffi")]
    fn create_signed_tx(&self, signer: Arc<ZkLinkSigner>) -> Result<Arc<Self>, ZkSignerError>
    where
        Self: Sized + Clone,
    {
        let mut tx = self.clone();
        let bytes = self.get_bytes();
        let signature = signer.sign_musig(&bytes)?;
        tx.set_signature(signature);
        Ok(Arc::new(tx))
    }
}
