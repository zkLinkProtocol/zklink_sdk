use crate::UniffiCustomTypeConverter;
use crate::{TxLayer1Signature, ZkLinkTx};
use zklink_sdk_signers::zklink_signer::signature::ZkLinkSignature;

macro_rules! ffi_json_convert {
    ($(#[$attr:meta])* $name:ident) => {
        impl UniffiCustomTypeConverter for $name {
            type Builtin = String;
            fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
                let s = serde_json::from_str(&val)?;
                Ok(s)
            }
            fn from_custom(obj: Self) -> Self::Builtin {
                serde_json::to_string(&obj).unwrap()
            }
        }
    };
}

ffi_json_convert!(TxLayer1Signature);
ffi_json_convert!(ZkLinkTx);
ffi_json_convert!(ZkLinkSignature);
