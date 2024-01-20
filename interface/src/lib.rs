use zklink_sdk_types::tx_type::change_pubkey::Create2Data;

pub mod error;
#[cfg(feature = "web")]
pub mod json_rpc_signer;
pub mod sign_auto_deleveraging;
pub mod sign_change_pubkey;
pub mod sign_contract_matching;
pub mod sign_forced_exit;
pub mod sign_funding;
pub mod sign_liquidation;
pub mod sign_order_matching;
pub mod sign_transfer;
pub mod sign_withdraw;
#[cfg(not(feature = "web"))]
pub mod signer;

pub enum ChangePubKeyAuthRequest {
    Onchain,
    EthECDSA,
    EthCreate2 { data: Create2Data },
}
