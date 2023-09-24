use crate::prelude::ZkLinkTx;
use serde::{Deserialize, Serialize};
use zklink_signers::eth_signer::eip1271_signature::EIP1271Signature;
use zklink_signers::eth_signer::packed_eth_signature::PackedEthSignature;
use zklink_signers::starknet_signer::ecdsa_signature::StarkECDSASignature;

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
    pub eth_signature: Option<PackedEthSignature>,
}
