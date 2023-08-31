use zklink_crypto::eth_signer::packed_eth_signature::PackedEthSignature;
use zklink_crypto::eth_signer::H256;
use zklink_crypto::zklink_signer::pubkey_hash::PubKeyHash;
use zklink_types::basic_types::ZkLinkAddress;
use zklink_types::tx_type::change_pubkey::CREATE2Data;
use zklink_types::tx_type::zklink_tx::ZkLinkTx;

pub mod credentials;
pub mod error;
pub mod signer;

pub struct TxSignature {
    pub tx: ZkLinkTx,
    pub eth_signature: Option<PackedEthSignature>,
}

pub enum ChangePubKeyAuthRequest {
    Onchain,
    EthECDSA,
    EthCREATE2(CREATE2Data),
}
