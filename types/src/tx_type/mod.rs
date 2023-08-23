use crate::basic_types::ZkLinkAddress;
use crate::tx_type::change_pubkey::ChangePubKey;
use crate::tx_type::forced_exit::ForcedExit;
use crate::tx_type::order_matching::OrderMatching;
use crate::tx_type::transfer::Transfer;
use crate::tx_type::withdraw::Withdraw;
use num::{BigUint, Zero};
use std::collections::VecDeque;

pub mod change_pubkey;
pub mod deposit;
pub mod error;
pub mod float_convert;
pub mod forced_exit;
pub mod full_exit;
pub mod order_matching;
pub mod pack;
pub mod transfer;
pub mod validator;
pub mod withdraw;

impl Withdraw {
    pub const TX_TYPE: u8 = 0x03;
}
impl Transfer {
    pub const TX_TYPE: u8 = 0x04;
}
impl ChangePubKey {
    pub const TX_TYPE: u8 = 0x06;
}
impl ForcedExit {
    pub const TX_TYPE: u8 = 0x07;
}
impl OrderMatching {
    pub const TX_TYPE: u8 = 0x08;
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
