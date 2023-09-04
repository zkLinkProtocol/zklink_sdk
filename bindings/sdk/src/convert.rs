use crate::{
    AccountId, BigUint, BlockNumber, ChainId, EthBlockId, Nonce, PackedEthSignature,
    PackedPublicKey, PackedSignature, PairId, PriorityOpId, PubKeyHash, SlotId, SubAccountId,
    TimeStamp, TokenId, TxHash, TxSignature, TypeError, UniffiCustomTypeConverter, ZkLinkAddress,
    H256,
};
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
