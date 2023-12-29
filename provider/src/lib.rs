pub mod error;
pub mod network;
pub mod response;
#[cfg(not(target_arch = "wasm32"))]
mod rpc;
pub mod web_socket;

#[cfg(not(any(feature = "ffi", target_arch = "wasm32")))]
mod not_ffi {
    pub use crate::rpc::{ZkLinkRpcClient, ZkLinkRpcServer};

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
        pub fn new(network: Network, timeout: Option<Duration>) -> Self {
            let mut builder = HttpClientBuilder::default();
            if let Some(timeout) = timeout {
                builder = builder.request_timeout(timeout);
            }
            let client = builder.build(network.url()).unwrap();
            Self(client)
        }
    }
}

#[cfg(not(any(feature = "ffi", target_arch = "wasm32")))]
pub use not_ffi::*;
