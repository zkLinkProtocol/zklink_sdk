pub mod basic_types;
pub mod contract;
pub mod error;
pub mod signatures;
pub mod tx_builder;
pub mod tx_type;
pub mod utils;

pub mod prelude {
    pub use super::basic_types::{
        bigunit_wrapper::BigUintSerdeWrapper, bit_convert::BitConvert,
        float_convert::FloatConversions, pad::pad_front, tx_hash::TxHash,
        zklink_address::ZkLinkAddress, AccountId, BlockNumber, ChainId, EthBlockId, GetBytes,
        MarginId, Nonce, PairId, PriorityOpId, SlotId, SubAccountId, TimeStamp, TokenId,
    };
    pub use super::contract::*;
    pub use super::error::TypeError;
    pub use super::signatures::{TxLayer1Signature, TxSignature};
    pub use super::tx_builder::*;
    pub use super::tx_type::{
        change_pubkey::{ChangePubKey, ChangePubKeyAuthData, Create2Data},
        deposit::Deposit,
        exit_info::ExitInfo,
        forced_exit::ForcedExit,
        full_exit::FullExit,
        order_matching::{Order, OrderMatching},
        transfer::Transfer,
        validator,
        withdraw::Withdraw,
        zklink_tx::{ZkLinkTx, ZkLinkTxType},
    };
    pub use zklink_sdk_signers::eth_signer::{PackedEthSignature, H160, H256, U256};
    pub use zklink_sdk_signers::zklink_signer::pubkey_hash::PubKeyHash;
    pub use zklink_sdk_signers::zklink_signer::public_key::PackedPublicKey;
    pub use zklink_sdk_signers::zklink_signer::signature::ZkLinkSignature;
}

pub mod params {
    pub use crate::basic_types::params::*;
    pub use zklink_sdk_signers::zklink_signer::{NEW_PUBKEY_HASH_BYTES_LEN, NEW_PUBKEY_HASH_WIDTH};
}
