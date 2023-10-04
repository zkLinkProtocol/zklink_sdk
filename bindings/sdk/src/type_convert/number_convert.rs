use crate::{
    AccountId, BlockNumber, ChainId, EthBlockId, Nonce, PairId, PriorityOpId, SlotId, SubAccountId,
    TimeStamp, TokenId, TypeError, UniffiCustomTypeConverter,
};
use zklink_sdk_signers::eth_signer::{Address, H256};
macro_rules! ffi_num_convert {
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

ffi_num_convert!(SlotId, u32);
ffi_num_convert!(TokenId, u32);
ffi_num_convert!(PairId, u16);
ffi_num_convert!(TimeStamp, u32);
ffi_num_convert!(AccountId, u32);
ffi_num_convert!(BlockNumber, u32);
ffi_num_convert!(Nonce, u32);
ffi_num_convert!(PriorityOpId, u64);
ffi_num_convert!(EthBlockId, u64);
ffi_num_convert!(ChainId, u8);
ffi_num_convert!(SubAccountId, u8);

macro_rules! ffi_num_hex_convert {
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

ffi_num_hex_convert!(H256, 32);
ffi_num_hex_convert!(Address, 20);

#[cfg(test)]
mod test {
    use super::*;
    use zklink_sdk_signers::eth_signer::H256;

    #[test]
    fn test_convert() {
        let h = H256::zero();
        let s = H256::from_custom(h);
        println!("{s}");
        let h2 = H256::into_custom(s).unwrap();
        println!("{h2:?}");
    }
}
