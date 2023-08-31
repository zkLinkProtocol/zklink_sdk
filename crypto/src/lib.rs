/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

pub mod eth_signer;
pub mod zklink_signer;
pub use zklink_signer::signature::ZkLinkSignature;

#[cfg(feature = "ffi")]
mod ffi {
    use crate::zklink_signer::error::ZkSignerError;
    use crate::zklink_signer::pk_signer::ZkLinkSigner;
    use crate::zklink_signer::private_key::PackedPrivateKey;
    use crate::zklink_signer::pubkey_hash::PubKeyHash;
    use crate::zklink_signer::public_key::PackedPublicKey;
    use super::ZkLinkSignature;

    include!(concat!(env!("OUT_DIR"), "/ffi.uniffi.rs"));
}

#[cfg(feature = "ffi")]
pub use ffi::*;
