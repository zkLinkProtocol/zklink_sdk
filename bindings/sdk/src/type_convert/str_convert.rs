use crate::{BigUint, UniffiCustomTypeConverter, ZkLinkAddress};
use std::str::FromStr;

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert() {
        // test BigUnit
        let b = BigUint::default();
        let s = b.to_string();
        let b2 = BigUint::from_str("12345678909876543219999999999").unwrap();
        println!("big uint: {:?}", s);
        println!("big uint: {:?}", b2);
        println!("big uint: {:?}", b2.to_string());
    }
}
