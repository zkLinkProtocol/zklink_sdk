pub mod basic_types;
pub mod tx_type;

#[cfg(feature = "ffi")]
mod ffi {
    use crate::basic_types::error::TypeError;
    use crate::basic_types::tx_hash::TxHash;
    use crate::basic_types::zklink_address::ZkLinkAddress;
    use std::str::FromStr;
    include!(concat!(env!("OUT_DIR"), "/ffi.uniffi.rs"));
}

#[cfg(feature = "ffi")]
pub use ffi::*;
