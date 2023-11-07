use crate::prelude::ZkLinkTx;
use serde::{Deserialize, Serialize};
use zklink_sdk_signers::eth_signer::eip1271_signature::EIP1271Signature;
use zklink_sdk_signers::eth_signer::packed_eth_signature::PackedEthSignature;
use zklink_sdk_signers::starknet_signer::ecdsa_signature::StarkECDSASignature;

/// Representation of the signature secured by L1.
/// May be either a signature generated via Ethereum private key
/// corresponding to the account address,
/// or on-chain signature via EIP-1271.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "signature")]
pub enum TxLayer1Signature {
    EthereumSignature(PackedEthSignature),
    EIP1271Signature(EIP1271Signature),
    StarkSignature(StarkECDSASignature),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxSignature {
    pub tx: ZkLinkTx,
    pub layer1_signature: Option<TxLayer1Signature>,
}

impl From<PackedEthSignature> for TxLayer1Signature {
    fn from(value: PackedEthSignature) -> Self {
        Self::EthereumSignature(value)
    }
}

impl From<EIP1271Signature> for TxLayer1Signature {
    fn from(value: EIP1271Signature) -> Self {
        Self::EIP1271Signature(value)
    }
}

impl From<StarkECDSASignature> for TxLayer1Signature {
    fn from(value: StarkECDSASignature) -> Self {
        Self::StarkSignature(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_layer1_signature_deserde() {
        let s = r#"
        {"type":"EthereumSignature","signature":"0x91dc468f37b6ef35cd0972881d37636f0c8f8dc974608ee9bf2e20ec03c546876092999bb802e6d673bb9fc858d750fa3e578b6bd2f3fe5a8e74ca23504a42661c"}
        "#;
        let signature: Result<TxLayer1Signature, _> = serde_json::from_str(s);
        assert!(signature.is_ok());
    }
}
