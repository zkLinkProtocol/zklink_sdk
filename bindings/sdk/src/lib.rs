/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

mod uniffi_bindings {
    zklink_crypto::uniffi_reexport_scaffolding!();
    // zklink_types::uniffi_reexport_scaffolding!();
}

use zklink_types::basic_types::{SlotId, TokenId, PairId, TimeStamp, AccountId, BlockNumber, Nonce, PriorityOpId, EthBlockId, ChainId, SubAccountId};
use zklink_types::basic_types::error::TypeError;
use zklink_types::basic_types::tx_hash::TxHash;
use zklink_types::basic_types::zklink_address::ZkLinkAddress;
use std::str::FromStr;

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

include!(concat!(env!("OUT_DIR"), "/ffi.uniffi.rs"));
