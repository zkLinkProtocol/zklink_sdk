//! Common primitives for the layer1 blockchain network interaction.
use crate::error::TypeError as Error;
use ethers::types::Address;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use starknet_ff::FieldElement;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use zklink_sdk_utils::serde::{Prefix, ZeroxPrefix};

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ZkLinkAddress(Vec<u8>);

impl ZkLinkAddress {
    /// Reads a account address from its byte sequence representation.
    ///
    /// Returns err if the slice length does not match with address length.
    pub fn from_slice(slice: &[u8]) -> Result<Self, Error> {
        if slice.len() != 32 && slice.len() != 20 {
            Err(Error::InvalidAddress)
        } else {
            let mut out = ZkLinkAddress(Vec::with_capacity(slice.len()));
            out.0.extend_from_slice(slice);
            Ok(out)
        }
    }

    /// Get address from hex string
    pub fn from_hex(hex_str: &str) -> Result<Self, Error> {
        let s = hex_str.strip_prefix("0x").unwrap_or(hex_str);
        let slice = hex::decode(s).map_err(|_| Error::InvalidAddress)?;
        Self::from_slice(&slice)
    }

    /// Get bytes by consuming self
    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }

    /// Get bytes of indeterminate length.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Get bytes of the certain max length.
    pub fn to_fixed_bytes(&self) -> [u8; 32] {
        let mut bytes = [0; 32];
        bytes[32 - self.0.len()..].copy_from_slice(&self.0);
        bytes
    }

    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|byte| *byte == 0)
    }

    /// GLOBAL_ASSET_ACCOUNT_ADDRESS is [0xff;32]
    pub fn is_global_account_address(&self) -> bool {
        self.0.len() == 32 && self.0.iter().all(|byte| *byte == 0xff)
    }

    /// According to Rng, it will randomly generate a ZklinkAddress.
    pub fn random() -> Self {
        ZkLinkAddress::from(Address::random().to_fixed_bytes())
    }
}

impl Debug for ZkLinkAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Default for ZkLinkAddress {
    fn default() -> ZkLinkAddress {
        ZkLinkAddress(vec![0; 32])
    }
}

impl AsRef<[u8]> for ZkLinkAddress {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl ToString for ZkLinkAddress {
    fn to_string(&self) -> String {
        format!("{}{}", ZeroxPrefix::prefix(), hex::encode(&self.0))
    }
}

impl From<Vec<u8>> for ZkLinkAddress {
    fn from(bytes: Vec<u8>) -> Self {
        assert!(bytes.len() == 32 || bytes.len() == 20);
        ZkLinkAddress(bytes)
    }
}

impl From<[u8; 20]> for ZkLinkAddress {
    fn from(bytes: [u8; 20]) -> Self {
        ZkLinkAddress(bytes.to_vec())
    }
}

impl From<[u8; 32]> for ZkLinkAddress {
    fn from(bytes: [u8; 32]) -> Self {
        ZkLinkAddress(bytes.to_vec())
    }
}

impl FromStr for ZkLinkAddress {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("0x") {
            return Err(Error::NotStartWithZerox);
        }
        let bytes = hex::decode(s.trim_start_matches("0x"))
            .map_err(|e| Error::DecodeFromHexErr(e.to_string()))?;
        if !(bytes.len() == 32 || bytes.len() == 20) {
            return Err(Error::SizeMismatch);
        }
        Ok(ZkLinkAddress(bytes))
    }
}

impl From<&ZkLinkAddress> for Address {
    fn from(zk_address: &ZkLinkAddress) -> Self {
        // eth address bytes len is 20
        Address::from_slice(&zk_address.as_bytes().to_vec()[..20])
    }
}

/// ZkLinkAddress into eth address
impl From<ZkLinkAddress> for Address {
    fn from(zk_address: ZkLinkAddress) -> Self {
        // eth address bytes len is 20
        Address::from_slice(&zk_address.as_bytes().to_vec()[..20])
    }
}

/// Eth address into ZkLinkAddress
impl From<Address> for ZkLinkAddress {
    fn from(address: Address) -> Self {
        ZkLinkAddress::from(address.to_fixed_bytes())
    }
}

/// starknet address into ZkLinkAddress
impl From<FieldElement> for ZkLinkAddress {
    fn from(address: FieldElement) -> Self {
        // starknet address bytes len is 32
        ZkLinkAddress::from(address.to_bytes_be())
    }
}

/// ZkLinkAddress into Starknet address
impl From<ZkLinkAddress> for FieldElement {
    fn from(zk_address: ZkLinkAddress) -> Self {
        // starknet address bytes len is 32
        FieldElement::from_bytes_be(&zk_address.to_fixed_bytes()).unwrap()
    }
}

impl Serialize for ZkLinkAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ZkLinkAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        Self::from_str(&string).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zklink_addresses() {
        let a = "0xffffffffffffffffffffffffffffffffffffffff";
        let b = "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
        let c = "0x0000000000000000000000000000000000000000";

        let a1 = ZkLinkAddress::from_slice(&[255u8; 20]).unwrap();
        let b1 = ZkLinkAddress::from_slice(&[255u8; 32]).unwrap();
        let c1 = ZkLinkAddress::from_slice(&[0u8; 20]).unwrap();
        assert!(b1.is_global_account_address());
        assert!(c1.is_zero());

        // test to_string
        let a_str = a1.to_string();
        let b_str = b1.to_string();
        let c_str = c1.to_string();
        assert_eq!(a, a_str);
        assert_eq!(b, b_str);
        assert_eq!(c, c_str);

        // test serde
        let a_str = serde_json::to_string(&a1).unwrap();
        let a_addr: ZkLinkAddress = serde_json::from_str(&a_str).unwrap();
        assert_eq!(a_addr, a1);
        let b_str = serde_json::to_string(&b1).unwrap();
        let b_addr: ZkLinkAddress = serde_json::from_str(&b_str).unwrap();
        assert_eq!(b_addr, b1);
        let c_str = serde_json::to_string(&c1).unwrap();
        let c_addr: ZkLinkAddress = serde_json::from_str(&c_str).unwrap();
        assert_eq!(c_addr, c1);

        // test deserde
        let a_addr: ZkLinkAddress = serde_json::from_str(&a_str).unwrap();
        assert_eq!(a_addr, a1);
        let b_addr: ZkLinkAddress = serde_json::from_str(&b_str).unwrap();
        assert_eq!(b_addr, b1);
        let c_addr: ZkLinkAddress = serde_json::from_str(&c_str).unwrap();
        assert_eq!(c_addr, c1);
    }

    #[test]
    fn test_address_convert() {
        // zklink address and starknet address convert
        let starknet_address = FieldElement::from_str(
            "0x3c9a304c229732090db86a0f1db015c08aa99e31bd68352855a910e477063f8",
        )
        .unwrap();
        let zklink_address: ZkLinkAddress = starknet_address.into();
        let address: FieldElement = zklink_address.into();
        assert_eq!(address, starknet_address);

        let zklink_address = ZkLinkAddress::from_str(
            "0x03c9a304c229732090db86a0f1db015c08aa99e31bd68352855a910e477063f8",
        )
        .unwrap();
        let address: FieldElement = zklink_address.into();
        assert_eq!(address, starknet_address);

        // zklink address and eth address convert
        let eth_address = Address::from_str("0x3D809E414BA4893709C85F242BA3617481BC4126").unwrap();
        let zklink_address: ZkLinkAddress = eth_address.into();
        let address: Address = zklink_address.into();
        assert_eq!(address, eth_address);

        let zklink_address =
            ZkLinkAddress::from_str("0x3D809E414BA4893709C85F242BA3617481BC4126").unwrap();
        let address: Address = zklink_address.into();
        assert_eq!(address, eth_address);
    }
}
