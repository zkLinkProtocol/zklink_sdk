use std::collections::HashMap;
use std::time::Duration;
use zklink_provider::network::Network;
use zklink_provider::response::TokenResp;
use zklink_provider::rpc::ZkLinkRpcClient;
use zklink_provider::ZkLinkRpcProvider;
use zklink_types::basic_types::TokenId;

#[tokio::test]
async fn test_get_tokens() {
    let client = ZkLinkRpcProvider::new(Network::MainNet, Duration::from_secs(3));
    let result: HashMap<TokenId, TokenResp> = client.tokens().await.unwrap();
    println!("{:?}", result);
    assert!(!result.is_empty());
}
