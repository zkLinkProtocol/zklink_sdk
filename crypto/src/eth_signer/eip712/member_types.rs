use super::eip712_standard::Structuralization;
use ethers::types::{Address, H256, U256};
use ethers::utils::keccak256;

enum MemberEncodeType {
    Atomic(String),
    Dynamic(String),
    Reference(String),
}

impl Structuralization for String {
    const MEMBER_TYPE: &'static str = "string";
    const IS_REFERENCE_TYPE: bool = false;

    fn encode_member_data(&self) -> H256 {
        keccak256(self).into()
    }
}

impl Structuralization for Address {
    const MEMBER_TYPE: &'static str = "address";
    const IS_REFERENCE_TYPE: bool = false;

    fn encode_member_data(&self) -> H256 {
        H256::from(*self)
    }
}

impl Structuralization for U256 {
    const MEMBER_TYPE: &'static str = "uint256";
    const IS_REFERENCE_TYPE: bool = false;

    fn encode_member_data(&self) -> H256 {
        let mut bytes = [0u8; 32];
        self.to_big_endian(&mut bytes);

        bytes.into()
    }
}

impl Structuralization for H256 {
    const MEMBER_TYPE: &'static str = "hash256";
    const IS_REFERENCE_TYPE: bool = false;

    fn encode_member_data(&self) -> H256 {
        *self
    }
}

macro_rules! impl_primitive {
    ($T: ident, $name:expr, $bit_size:expr) => {
        impl Structuralization for $T {
            const MEMBER_TYPE: &'static str = $name;
            const IS_REFERENCE_TYPE: bool = false;

            fn encode_member_data(&self) -> H256 {
                let mut bytes = [0u8; 32];
                let bytes_value = self.to_be_bytes();
                bytes[32 - $bit_size / 8..].copy_from_slice(&bytes_value);

                bytes.into()
            }
        }
    };
}

impl_primitive!(u8, "uint8", 8);
impl_primitive!(u16, "uint16", 16);
impl_primitive!(u32, "uint32", 32);
impl_primitive!(u64, "uint64", 64);
impl_primitive!(u128, "uint128", 128);
