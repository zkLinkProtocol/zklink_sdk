#[cfg(not(feature = "ffi"))]
use crate::error::SignError;
#[cfg(not(feature = "ffi"))]
use serde::{Deserialize, Serialize};
use zklink_signers::eth_signer::eth_signature::TxEthSignature;
#[cfg(not(feature = "ffi"))]
use zklink_signers::eth_signer::packed_eth_signature::PackedEthSignature;
use zklink_types::tx_type::change_pubkey::Create2Data;
#[cfg(not(feature = "ffi"))]
use zklink_types::tx_type::zklink_tx::ZkLinkTx;

pub mod error;
pub mod sign_change_pubkey;
pub mod sign_forced_exit;
pub mod sign_order;
pub mod sign_order_matching;
pub mod sign_transfer;
pub mod sign_withdraw;

#[cfg(not(feature = "ffi"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxSignature {
    pub tx: ZkLinkTx,
    pub eth_signature: Option<PackedEthSignature>,
}

pub enum ChangePubKeyAuthRequest {
    OnChain,
    EthECDSA,
    EthCreate2 { data: Create2Data },
}
