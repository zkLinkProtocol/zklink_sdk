pub mod error;
pub mod network;
pub mod response;
#[cfg(not(any(feature = "ffi", target_arch = "wasm32")))]
pub mod rpc;
pub mod rpc_client;

#[cfg(not(any(feature = "ffi", target_arch = "wasm32")))]
mod not_ffi {
    use crate::network::Network;
    use jsonrpsee::http_client::{HttpClient, HttpClientBuilder};
    use std::ops::Deref;
    use std::time::Duration;

    /// `ZkLinkRpcProvider` is capable of interacting with the ZKLink node via its
    /// JSON RPC interface.
    #[derive(Debug, Clone)]
    pub struct ZkLinkRpcProvider(HttpClient);

    impl Deref for ZkLinkRpcProvider {
        type Target = HttpClient;

        fn deref(&self) -> &Self::Target {
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

#[cfg(not(any(feature = "ffi", target_arch = "wasm32")))]
pub use crate::rpc::ZkLinkRpcClient;
#[cfg(not(any(feature = "ffi", target_arch = "wasm32")))]
pub use not_ffi::*;
