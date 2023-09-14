pub mod error;
pub mod signer;
pub mod wallet;
pub use zklink_interface::ChangePubKeyAuthRequest;
pub use zklink_types::*;

use crate::signer::Signer;
use std::collections::HashMap;
use zklink_provider::response::{AccountInfoResp, ChainResp, TokenResp};
use zklink_provider::ZkLinkRpcClient;

use zklink_types::basic_types::{ChainId, TokenId, ZkLinkAddress};

pub enum AccountType {
    ECDSA,
    CREATE2,
}
pub struct Wallet<P: ZkLinkRpcClient> {
    /// zkLink rpc provider
    pub provider: P,
    /// signer is who can control the account
    pub signer: Signer,
    /// account address
    address: ZkLinkAddress,
    /// account info
    account_info: AccountInfoResp,
    pub account_type: AccountType,
    chains: HashMap<ChainId, ChainResp>,
    tokens: HashMap<TokenId, TokenResp>,
}
