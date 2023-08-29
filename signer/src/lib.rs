use zklink_crypto::eth_signer::packed_eth_signature::PackedEthSignature;
use zklink_types::tx_type::zklink_tx::ZkLinkTx;

pub mod credentials;
pub mod error;
pub mod signer;

pub struct TxSignature {
    pub tx: ZkLinkTx,
    pub eth_signature: Option<PackedEthSignature>,
}
