use zklink_sdk_signers::starknet_signer::StarkECDSASignature;
use crate::{
    PackedEthSignature, PackedPublicKey, PackedSignature, PubKeyHash, TxHash,
    UniffiCustomTypeConverter,
};

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
ffi_hex_convert!(StarkECDSASignature);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert() {
        // test packed_eth_signature
        let signature = PackedEthSignature::default();
        let s = PackedEthSignature::from_custom(signature);
        println!("packed eth signer: {s}");
        let signature2 = PackedEthSignature::into_custom(s);
        assert!(signature2.is_ok());
    }
}
