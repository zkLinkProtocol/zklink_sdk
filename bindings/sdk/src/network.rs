use zklink_sdk_provider::network::Network;

pub fn zklink_main_net_url() -> String {
    let network = Network::MainNet;
    network.url().into()
}

pub fn zklink_test_net_url() -> String {
    let network = Network::TestNet;
    network.url().into()
}
