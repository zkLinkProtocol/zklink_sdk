pub mod network;
pub mod response;
#[cfg(not(feature = "ffi"))]
pub mod rpc;

#[cfg(not(feature = "ffi"))]
mod not_ffi {
    use crate::network::Network;
    use jsonrpsee::http_client::{HttpClient, HttpClientBuilder};
    use std::time::Duration;

    /// `ZkLinkRpcProvider` is capable of interacting with the ZKLink node via its
    /// JSON RPC interface.
    #[derive(Debug, Clone)]
    pub struct ZkLinkRpcProvider(HttpClient);

    impl AsRef<HttpClient> for ZkLinkRpcProvider {
        fn as_ref(&self) -> &HttpClient {
            &self.0
        }
    }

    impl ZkLinkRpcProvider {
        pub fn new(network: Network, timeout: Duration) -> Self {
            let zklink_client = HttpClientBuilder::default()
                .request_timeout(timeout)
                .build(network.url())
                .unwrap();

            Self(zklink_client)
        }
    }
}

#[cfg(not(feature = "ffi"))]
pub use crate::rpc::ZkLinkRpcClient;
#[cfg(not(feature = "ffi"))]
pub use jsonrpsee::core::Error as RpcError;
#[cfg(not(feature = "ffi"))]
pub use not_ffi::*;
