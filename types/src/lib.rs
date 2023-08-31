pub mod basic_types;
pub mod tx_type;

// #[cfg(feature = "ffi")]
// mod ffi {
//     use crate::basic_types::{SlotId, TokenId, PairId, TimeStamp, AccountId, BlockNumber, Nonce, PriorityOpId, EthBlockId, ChainId, SubAccountId};
//     use crate::basic_types::error::TypeError;
//     use crate::basic_types::tx_hash::TxHash;
//     use crate::basic_types::zklink_address::ZkLinkAddress;
//     use std::str::FromStr;
//
//     macro_rules! ffi_convert {
//         ($(#[$attr:meta])* $name:ident, $type:ty) => {
//             impl UniffiCustomTypeConverter for $name {
//                 type Builtin = $type;
//                 fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
//                     Ok($name(val))
//                 }
//                 fn from_custom(obj: Self) -> Self::Builtin {
//                     obj.0
//                 }
//             }
//         };
//     }
//
//     ffi_convert!(SlotId, u32);
//     ffi_convert!(TokenId, u32);
//     ffi_convert!(PairId, u16);
//     ffi_convert!(TimeStamp, u32);
//     ffi_convert!(AccountId, u32);
//     ffi_convert!(BlockNumber, u32);
//     ffi_convert!(Nonce, u32);
//     ffi_convert!(PriorityOpId, u64);
//     ffi_convert!(EthBlockId, u64);
//     ffi_convert!(ChainId, u8);
//     ffi_convert!(SubAccountId, u8);
//     // include!(concat!(env!("OUT_DIR"), "/ffi.uniffi.rs"));
// }
//
// #[cfg(feature = "ffi")]
// pub use ffi::*;
