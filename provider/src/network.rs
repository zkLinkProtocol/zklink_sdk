use crate::error::RpcError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Network to be used for a zklink client.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Network {
    /// Mainnet.
    MainNet,
    /// Test network for testkit purposes
    TestNet,
}

impl Network {
    pub fn url(&self) -> &str {
        match self {
            Network::MainNet => "https://api-v1.zk.link",
            Network::TestNet => "https://aws-gw-v2.zk.link",
        }
    }

    pub fn from_str(str: &str) -> Self {
        match str {
            "mainet" => Network::MainNet,
            "testnet" => Network::TestNet,
            _ => Network::TestNet,
        }
    }
}

impl FromStr for Network {
    type Err = RpcError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mainet" => Ok(Network::MainNet),
            "testnet" => Ok(Network::TestNet),
            _ => Err(RpcError::InvalidNetwork),
        }
    }
}
