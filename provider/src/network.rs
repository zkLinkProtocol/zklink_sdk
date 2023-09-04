use serde::{Deserialize, Serialize};

/// Network to be used for a zklink client.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Network {
    /// Mainnet.
    MainNet,
    /// Test network for testkit purposes
    TestNet,
    DevNet,
}

impl Network {
    pub fn url(&self) -> &str {
        match self {
            Network::MainNet => "https://api-v1.zk.link:443",
            Network::TestNet => "https://aws-gw-v2.zk.link:443",
            Network::DevNet => "http://127.0.0.1:3030",
        }
    }
}
