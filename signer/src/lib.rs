use zklink_types::tx_type::zklink_tx::ZkLinkTx;
use zklink_crypto::eth_signer::packed_eth_signature::PackedEthSignature;

mod credentials;
mod error;
mod signer;

pub struct TxSignature {
    tx: ZkLinkTx,
    eth_signature: Option<PackedEthSignature>,
}
