use crate::eth_signer::error::EthSignerError;
use crate::eth_signer::Address;
use ethers::types::Signature;
use ethers::utils::keccak256;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use zklink_sdk_utils::serde::ZeroPrefixHexSerde;
/// Struct used for working with ethereum signatures created using eth_sign (using geth, ethers.js, etc)
/// message is serialized as 65 bytes long `0x` prefixed string.
///
/// Some notes on implementation of methods of this structure:
///
/// Ethereum signed message produced by most clients contains v where v = 27 + recovery_id(0,1,2,3),
/// but for some clients v = recovery_id(0,1,2,3).
/// Library that we use for signature verification (written for bitcoin) expects v = recovery_id
///
/// That is why:
/// 1) when we create this structure by deserialization of message produced by user
/// we subtract 27 from v in `ETHSignature` if necessary and store it in the `ETHSignature` structure this way.
/// 2) When we serialize/create this structure we add 27 to v in `ETHSignature`.
///
/// This way when we have methods that consumes &self we can be sure that ETHSignature::recover_signer works
/// And we can be sure that we are compatible with Ethereum clients.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackedEthSignature(pub Signature);

impl Default for PackedEthSignature {
    fn default() -> Self {
        let bytes: [u8; 65] = [0; 65];
        Self::deserialize_packed(&bytes).unwrap()
    }
}

impl PackedEthSignature {
    pub fn serialize_packed(&self) -> [u8; 65] {
        self.0.into()
    }

    pub fn deserialize_packed(bytes: &[u8]) -> Result<Self, EthSignerError> {
        if bytes.len() != 65 {
            return Err(EthSignerError::LengthMismatched);
        }

        let mut bytes_array = [0u8; 65];
        bytes_array.copy_from_slice(bytes);

        Ok(PackedEthSignature(
            Signature::try_from(bytes_array.as_slice())
                .map_err(|_err| EthSignerError::InvalidEthSigner)?,
        ))
    }

    pub fn from_hex(s: &str) -> Result<Self, EthSignerError> {
        let s = s.strip_prefix("0x").unwrap_or(s);
        let raw = hex::decode(s).map_err(|_e| EthSignerError::InvalidSignatureStr)?;
        Self::deserialize_packed(&raw)
    }

    pub fn as_hex(&self) -> String {
        let raw = self.serialize_packed();
        format!("0x{}", hex::encode(raw))
    }

    /// Checks signature and returns ethereum address of the signer.
    /// message should be the same message that was passed to `eth.sign`(or similar) method
    /// as argument. No hashing and prefixes required.
    pub fn signature_recover_signer(&self, msg: &[u8]) -> Result<Address, EthSignerError> {
        let address = self
            .0
            .recover(msg)
            .map_err(|err| EthSignerError::RecoverAddress(err.to_string()))?;

        Ok(Address::from_slice(address.as_bytes()))
    }

    pub fn eip712_signature_recover_signer(&self, msg: &[u8]) -> Result<Address, EthSignerError> {
        let msg_hash = keccak256(msg);
        let address = self
            .0
            .recover(msg_hash)
            .map_err(|err| EthSignerError::RecoverAddress(err.to_string()))?;
        Ok(address)
    }
}

impl Serialize for PackedEthSignature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let packed_signature = self.serialize_packed();
        ZeroPrefixHexSerde::serialize(packed_signature, serializer)
    }
}

impl<'de> Deserialize<'de> for PackedEthSignature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes = ZeroPrefixHexSerde::deserialize(deserializer)?;
        Self::deserialize_packed(&bytes).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eth_signer::pk_signer::EthSigner;
    use crate::eth_signer::Address;
    use std::str::FromStr;

    #[test]
    fn test_packed_eth_signature() {
        let private_key = "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
        let msg = vec![1, 2, 3, 4, 5];
        let pk = EthSigner::try_from(private_key).unwrap();
        let signature = pk.sign_message(&msg).unwrap();
        let signature_str = "0xd226c38ff38e07f50d8455fa004168bdd3eb6d860d72ecb1549c0891db64a56e52d450091f0c1dbff67d2bb8394e01df9a4a7c13d47c9fa10897e0bbcab122de1b";
        assert_eq!(signature.as_hex(), signature_str);

        let sig = PackedEthSignature::from_hex(signature_str).unwrap();
        let address = sig.signature_recover_signer(&msg).unwrap();
        let pk_address = pk.get_address();
        assert_eq!(address, pk_address);

        let sign_address = Address::from_str("0xdec58607c3f5a0f8bc51ca50cc2578ab282865fc").unwrap();
        assert_eq!(address, sign_address);
    }
}
