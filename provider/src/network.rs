use serde::{Deserialize, Serialize};

/// Network to be used for a zklink client.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Network {
    /// Mainnet.
    Mainnet,
    /// Test network for testkit purposes
    Test,
    Localhost,
}

impl Network {
    pub fn url(&self) -> &str {
        match self {
            Network::Mainnet => "https://api-v1.zk.link:443",
            Network::Test => "https://aws-gw-v2.zk.link:443",
            Network::Localhost => "http://127.0.0.1:3030",
        }
    }
}
