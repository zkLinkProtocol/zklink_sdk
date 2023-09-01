use zklink_crypto::eth_signer::packed_eth_signature::PackedEthSignature;
use zklink_types::tx_type::change_pubkey::Create2Data;
use zklink_types::tx_type::zklink_tx::ZkLinkTx;

pub mod error;
pub mod sign_change_pubkey;
pub mod sign_forced_exit;
mod sign_order;
mod sign_order_matching;
pub mod sign_transfer;
pub mod sign_withdraw;

pub struct TxSignature {
    pub tx: ZkLinkTx,
    pub eth_signature: Option<PackedEthSignature>,
}

pub enum ChangePubKeyAuthRequest {
    Onchain,
    EthECDSA,
    EthCREATE2 { data: Create2Data },
}
