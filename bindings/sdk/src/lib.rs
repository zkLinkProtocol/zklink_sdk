/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
mod crypto;
mod types;

use crate::crypto::{get_public_key_hash, verify_musig};

use std::str::FromStr;

use zklink_crypto::eth_signer::error::EthSignerError;
use zklink_crypto::eth_signer::packed_eth_signature::PackedEthSignature;
use zklink_crypto::eth_signer::pk_signer::PrivateKeySigner;

use zklink_crypto::zklink_signer::error::ZkSignerError;
use zklink_crypto::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_crypto::zklink_signer::pubkey_hash::PubKeyHash;
use zklink_crypto::zklink_signer::public_key::PackedPublicKey;
use zklink_crypto::zklink_signer::signature::{PackedSignature, ZkLinkSignature};

use zklink_types::basic_types::error::TypeError;
use zklink_types::basic_types::tx_hash::TxHash;
use zklink_types::basic_types::zklink_address::ZkLinkAddress;
use zklink_types::basic_types::{
    AccountId, BigUint, BlockNumber, ChainId, EthBlockId, Nonce, PairId, PriorityOpId, SlotId,
    SubAccountId, TimeStamp, TokenId, H160, H256,
};
use zklink_types::tx_type::change_pubkey::ChangePubKey;
use zklink_types::tx_type::change_pubkey::Create2Data;
use zklink_types::tx_type::deposit::Deposit;
use zklink_types::tx_type::forced_exit::ForcedExit;
use zklink_types::tx_type::order_matching::Order;
use zklink_types::tx_type::order_matching::OrderMatching;
use zklink_types::tx_type::transfer::Transfer;
use zklink_types::tx_type::withdraw::Withdraw;

use zklink_interface::error::SignError;
use zklink_interface::sign_change_pubkey::sign_change_pubkey;
use zklink_interface::sign_forced_exit::sign_forced_exit;
use zklink_interface::sign_order::sign_order;
use zklink_interface::sign_order_matching::sign_order_matching;
use zklink_interface::sign_transfer::sign_transfer;
use zklink_interface::sign_withdraw::sign_withdraw;
use zklink_interface::ChangePubKeyAuthRequest;
use zklink_interface::TxSignature;

macro_rules! ffi_convert {
    ($(#[$attr:meta])* $name:ident, $type:ty) => {
        impl UniffiCustomTypeConverter for $name {
            type Builtin = $type;
            fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
                Ok($name(val))
            }
            fn from_custom(obj: Self) -> Self::Builtin {
                obj.0
            }
        }
    };
}

ffi_convert!(SlotId, u32);
ffi_convert!(TokenId, u32);
ffi_convert!(PairId, u16);
ffi_convert!(TimeStamp, u32);
ffi_convert!(AccountId, u32);
ffi_convert!(BlockNumber, u32);
ffi_convert!(Nonce, u32);
ffi_convert!(PriorityOpId, u64);
ffi_convert!(EthBlockId, u64);
ffi_convert!(ChainId, u8);
ffi_convert!(SubAccountId, u8);

macro_rules! ffi_str_convert {
    ($(#[$attr:meta])* $name:ident) => {
        impl UniffiCustomTypeConverter for $name {
            type Builtin = String;
            fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
                let s = $name::from_str(&val)?;
                Ok(s)
            }
            fn from_custom(obj: Self) -> Self::Builtin {
                obj.to_string()
            }
        }
    };
}

ffi_str_convert!(BigUint);
ffi_str_convert!(ZkLinkAddress);

macro_rules! ffi_hex_convert {
    ($(#[$attr:meta])* $name:ident) => {
        impl UniffiCustomTypeConverter for $name {
            type Builtin = String;
            fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
                let s = $name::from_hex(&val)?;
                Ok(s)
            }
            fn from_custom(obj: Self) -> Self::Builtin {
                obj.as_hex()
            }
        }
    };
}

ffi_hex_convert!(TxHash);
ffi_hex_convert!(PackedPublicKey);
ffi_hex_convert!(PackedSignature);
ffi_hex_convert!(PubKeyHash);
ffi_hex_convert!(PackedEthSignature);

macro_rules! ffi_num_convert {
    ($(#[$attr:meta])* $name:ident, $num:expr) => {
        impl UniffiCustomTypeConverter for $name {
            type Builtin = String;
            fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
                let s = val.as_str().strip_prefix("0x").unwrap_or(&val);
                let raw = hex::decode(s)?;
                if raw.len() != $num {
                    return Err(TypeError::SizeMismatch.into());
                }
                let h = $name::from_slice(&raw);
                Ok(h)
            }
            fn from_custom(obj: Self) -> Self::Builtin {
                let s = hex::encode(obj.as_bytes());
                format!("0x{s}")
            }
        }
    };
}

ffi_num_convert!(H256, 32);
ffi_num_convert!(H160, 20);

macro_rules! ffi_json_convert {
    ($(#[$attr:meta])* $name:ident) => {
        impl UniffiCustomTypeConverter for $name {
            type Builtin = String;
            fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
                let s: $name = serde_json::from_str(&val)?;
                Ok(s)
            }
            fn from_custom(obj: Self) -> Self::Builtin {
                serde_json::to_string(&obj).expect("invalid string")
            }
        }
    };
}

ffi_json_convert!(TxSignature);

include!(concat!(env!("OUT_DIR"), "/ffi.uniffi.rs"));

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert() {
        let h = H256::zero();
        let s = H256::from_custom(h);
        println!("{s}");
        let h2 = H256::into_custom(s).unwrap();
        println!("{h2:?}");

        // test BigUnit
        let b = BigUint::default();
        let s = b.to_string();
        let b2 = BigUint::from_str("12345678909876543219999999999").unwrap();
        println!("big uint: {:?}", s);
        println!("big uint: {:?}", b2);
        println!("big uint: {:?}", b2.to_string());

        // test packed_eth_signature
        let signature = PackedEthSignature::default();
        let s = PackedEthSignature::from_custom(signature);
        println!("packed eth signer: {s}");
        let signature2 = PackedEthSignature::into_custom(s);
        assert!(signature2.is_ok());
    }
}
