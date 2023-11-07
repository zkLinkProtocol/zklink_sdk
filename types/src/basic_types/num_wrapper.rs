use num::{BigInt, BigUint};
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use num::bigint::ToBigInt;
use zklink_sdk_utils::serde::{BigUintSerdeAsRadix10Str, BigIntSerdeAsRadix10Str};

#[derive(Clone, Debug, Serialize, Deserialize, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct BigUintSerdeWrapper(#[serde(with = "BigUintSerdeAsRadix10Str")] pub BigUint);

impl From<BigUint> for BigUintSerdeWrapper {
    fn from(uint: BigUint) -> BigUintSerdeWrapper {
        BigUintSerdeWrapper(uint)
    }
}

impl From<BigInt> for BigUintSerdeWrapper {
    fn from(big_int: BigInt) -> BigUintSerdeWrapper {
        BigUintSerdeWrapper(big_int.to_biguint().unwrap())
    }
}
impl Deref for BigUintSerdeWrapper {
    type Target = BigUint;

    fn deref(&self) -> &BigUint {
        &self.0
    }
}

impl DerefMut for BigUintSerdeWrapper {
    fn deref_mut(&mut self) -> &mut BigUint {
        &mut self.0
    }
}

impl ToString for BigUintSerdeWrapper {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct BigIntSerdeWrapper(#[serde(with = "BigIntSerdeAsRadix10Str")] pub BigInt);

impl From<BigUint> for BigIntSerdeWrapper {
    fn from(uint: BigUint) -> BigIntSerdeWrapper {
        BigIntSerdeWrapper(uint.to_bigint().unwrap())
    }
}

impl From<BigInt> for BigIntSerdeWrapper {
    fn from(big_int: BigInt) -> BigIntSerdeWrapper {
        BigIntSerdeWrapper(big_int)
    }
}

impl Deref for BigIntSerdeWrapper {
    type Target = BigInt;

    fn deref(&self) -> &BigInt {
        &self.0
    }
}

impl DerefMut for BigIntSerdeWrapper {
    fn deref_mut(&mut self) -> &mut BigInt {
        &mut self.0
    }
}

impl ToString for BigIntSerdeWrapper {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

