use std::collections::HashMap;
use std::time::Duration;
use zklink_provider::network::Network;
use zklink_provider::rpc::ZkLinkRpcClient;
use zklink_provider::types::TokenResp;
use zklink_provider::ZkLinkRpcProvider;
use zklink_types::basic_types::TokenId;

#[tokio::test]
async fn test_get_tokens() {
    let client = ZkLinkRpcProvider::new(Network::Mainnet, Duration::from_secs(3));
    let result: HashMap<TokenId, TokenResp> = client.as_ref().tokens().await.unwrap();
    println!("{:?}", result);
    assert!(!result.is_empty());
}
