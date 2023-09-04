mod wallet;

use std::collections::HashMap;
use zklink_signers::eth_signer::EthereumSigner;
use zklink_provider::types::{AccountInfoResp, ChainResp, TokenResp};
use zklink_provider::ZkLinkRpcClient;
use zklink_types::basic_types::{ChainId, TokenId, ZkLinkAddress};

pub enum AccountType {
    ECDSA,
    CREATE2,
}
pub struct Wallet<S: EthereumSigner, P: ZkLinkRpcClient> {
    /// zkLink rpc provider
    pub provider: P,
    /// signer is who can control the account
    pub signer: Signer<S>,
    /// account address
    address: ZkLinkAddress,
    /// account info
    account_info: AccountInfoResp,
    pub account_type: AccountType,
    chains: HashMap<ChainId, ChainResp>,
    tokens: HashMap<TokenId, TokenResp>,
}
