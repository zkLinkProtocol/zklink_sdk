pub mod network;
pub mod response;
#[cfg(not(feature = "ffi"))]
#[cfg(not(target_arch = "wasm32"))]
pub mod rpc;
#[cfg(target_arch = "wasm32")]
pub mod rpc_wasm;
pub mod error;

#[cfg(not(feature = "ffi"))]
#[cfg(not(target_arch = "wasm32"))]
mod not_ffi {
    use crate::network::Network;
    use jsonrpsee::http_client::{HttpClient as Client, HttpClientBuilder as ClientBuilder};
    use std::ops::Deref;
    use std::time::Duration;

    /// `ZkLinkRpcProvider` is capable of interacting with the ZKLink node via its
    /// JSON RPC interface.
    #[derive(Debug)]
    pub struct ZkLinkRpcProvider(Client);

    impl Deref for ZkLinkRpcProvider {
        type Target = Client;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl ZkLinkRpcProvider {
        #[cfg(not(target_arch = "wasm32"))]
        pub fn new(network: Network, timeout: Duration) -> Self {
            let zklink_client = ClientBuilder::default()
                .request_timeout(timeout)
                .build(network.url())
                .unwrap();

            Self(zklink_client)
        }
    }
}

#[cfg(not(feature = "ffi"))]
#[cfg(not(target_arch = "wasm32"))]
pub use crate::rpc::ZkLinkRpcClient;
#[cfg(not(feature = "ffi"))]
#[cfg(not(target_arch = "wasm32"))]
pub use jsonrpsee::core::Error as RpcError;
#[cfg(not(feature = "ffi"))]
#[cfg(not(target_arch = "wasm32"))]
pub use not_ffi::*;
