//! Common primitives for the layer1 blockchain network interaction.
// Built-in deps
use ethers::types::Address;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
// External uses
use super::error::AddressError as Error;
use rand::Rng;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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

    /// Get bytes by consuming self
    pub fn into_inner(self) -> Vec<u8> {
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
    pub fn rand<T: Rng>(rng: &mut T) -> Self {
        ZkLinkAddress::from(rng.gen::<[u8; 20]>())
    }
}

impl Debug for ZkLinkAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{}", hex::encode(&self.0))
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
        format!("0x{}", hex::encode(&self.0))
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

impl FromStr for ZkLinkAddress {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        anyhow::ensure!(s.starts_with("0x"), "Address should start with 0x");
        let bytes = hex::decode(s.trim_start_matches("0x"))?;
        anyhow::ensure!(bytes.len() == 32 || bytes.len() == 20, "Size mismatch");
        Ok(ZkLinkAddress(bytes))
    }
}

impl From<&ZkLinkAddress> for Address {
    fn from(zk_address: &ZkLinkAddress) -> Self {
        // eth address bytes len is 20
        Address::from_slice(&zk_address.as_bytes().to_vec()[..20])
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

#[test]
fn test_zklink_addresses() {
    let a = "0xffffffffffffffffffffffffffffffffffffffff";
    let b = "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";

    let a1 = ZkLinkAddress::from_slice(&[255u8; 20]).unwrap();
    let b1 = ZkLinkAddress::from_slice(&[255u8; 32]).unwrap();
    let a_str = serde_json::to_string(&a1).unwrap();
    let b_str = serde_json::to_string(&b1).unwrap();

    let a_addr: ZkLinkAddress = serde_json::from_str(&a_str).unwrap();
    let b_addr: ZkLinkAddress = serde_json::from_str(&b_str).unwrap();

    assert_eq!(a_addr, a1);
    assert_eq!(a_addr, ZkLinkAddress::from_str(a).unwrap());
    assert_eq!(b_addr, b1);
    assert_eq!(b_addr, ZkLinkAddress::from_str(b).unwrap());
}
