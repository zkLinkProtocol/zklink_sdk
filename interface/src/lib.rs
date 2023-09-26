use zklink_sdk_types::tx_type::change_pubkey::Create2Data;

pub mod error;
pub mod sign_change_pubkey;
pub mod sign_forced_exit;
pub mod sign_order;
pub mod sign_order_matching;
pub mod sign_transfer;
pub mod sign_withdraw;
pub mod signer;

pub enum ChangePubKeyAuthRequest {
    Onchain,
    EthECDSA,
    EthCreate2 { data: Create2Data },
}
