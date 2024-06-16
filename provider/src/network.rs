use crate::error::RpcError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Network to be used for a zklink client.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Network {
    /// Mainnet.
    MainNet,
    /// Test network for testkit purposes
    TestNet,
    /// Develop network
    DevNet,
    /// Custom url
    Custom(String),
}

impl Network {
    pub fn url(&self) -> &str {
        match self {
            Network::MainNet => "https://api-v1.zk.link",
            Network::TestNet => "https://aws-gw-v2.zk.link",
            Network::DevNet => "https://dev-gw-v1.zk.link",
            Network::Custom(s) => &s,
        }
    }
}

impl FromStr for Network {
    type Err = RpcError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mainet" => Ok(Network::MainNet),
            "testnet" => Ok(Network::TestNet),
            "devnet" => Ok(Network::DevNet),
            _ => Ok(Network::Custom(s.to_string())),
        }
    }
}
