//! The declaration of the most primitive types used in zklink network.
//! Most of them are just re-exported from the `web3` crate.

use std::fmt;
use std::num::ParseIntError;
use std::ops::{Add, Deref, DerefMut, Sub};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

pub use zklink_address::ZkLinkAddress;

#[macro_use]
mod macros;
pub mod bigunit_wrapper;
pub mod bit_convert;
pub mod float_convert;
pub(crate) mod pack;
pub(crate) mod pad;
pub(crate) mod params;
pub mod tx_hash;
pub mod zklink_address;

use crate::params::{ACCOUNT_ID_BIT_WIDTH, TOKEN_BIT_WIDTH};
pub use num::BigUint;
use zklink_sdk_signers::zklink_signer::utils::rescue_hash_orders;

basic_type!(
    /// Unique identifier of the slot in the zklink network.
    SlotId,
    u32
);

basic_type!(
    /// Unique identifier of the token in the zklink network.
    TokenId,
    u32
);

basic_type!(
    /// Unique identifier of the contract token pair in the zklink network.
    PairId,
    u16
);

basic_type!(
    /// unix timestamp
    TimeStamp,
    u32
);

basic_type!(
    /// Unique identifier of the account in the zklink network.
    AccountId,
    u32
);

basic_type!(
    /// zklink network block sequential index.
    BlockNumber,
    u32
);

basic_type!(
    /// zklink account nonce.
    Nonce,
    u32
);

basic_type!(
    /// Unique identifier of the priority operation in the zklink network.
    PriorityOpId,
    u64
);

basic_type!(
    /// Block number in the Ethereum network.
    EthBlockId,
    u64
);

basic_type!(
    /// Unique identifier of the chain in the network
    ChainId,
    u8
);
basic_type!(
    /// Unique identifier of the SubAccount in the network
    SubAccountId,
    u8
);

basic_type!(
    /// Unique identifier of the margin in the network
    MarginId,
    u8
);

pub trait GetBytes {
    /// the length of encoded bytes
    fn bytes_len(&self) -> usize;

    /// Encodes the data as the byte sequence.
    fn get_bytes(&self) -> Vec<u8>;

    /// calculate the hash of encoded bytes
    fn rescue_hash(&self) -> Vec<u8> {
        let mut hash_input = self.get_bytes();
        hash_input.resize(self.bytes_len(), 0);
        rescue_hash_orders(&hash_input)
    }
}

impl GetBytes for AccountId {
    fn get_bytes(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }

    fn bytes_len(&self) -> usize {
        ACCOUNT_ID_BIT_WIDTH / 8
    }
}

impl GetBytes for TokenId {
    fn get_bytes(&self) -> Vec<u8> {
        (self.0 as u16).to_be_bytes().to_vec()
    }
    fn bytes_len(&self) -> usize {
        TOKEN_BIT_WIDTH / 8
    }
}

impl<T: GetBytes> GetBytes for Vec<T> {
    fn get_bytes(&self) -> Vec<u8> {
        let bytes_len = self.bytes_len();
        let mut bytes = Vec::with_capacity(bytes_len);
        self.iter()
            .for_each(|info| bytes.extend(info.get_bytes()));
        bytes
    }
    fn bytes_len(&self) -> usize {
        self.iter().map(|v| v.bytes_len()).sum()
    }
}
