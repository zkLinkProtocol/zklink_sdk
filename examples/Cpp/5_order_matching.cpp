#include <iostream>
#include "zklink_sdk.hpp"

using namespace std;
using namespace zklink_sdk;

int main() {
    string private_key = "0xbe725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
    shared_ptr<ZkLinkSigner> zklink_signer = ZkLinkSigner::new_from_hex_eth_signer(private_key);
    shared_ptr<Order> taker = Order::init(10, 1, 3, 1, 18, 145, "323289", "135", true, true, 2, 5, {});
    taker = taker->create_signed_order(zklink_signer);
    shared_ptr<Order> maker = Order::init(10, 1, 3, 1, 18, 145, "323288", "135", true, true, 2, 5, {});
    maker = maker->create_signed_order(zklink_signer);
    vector<ContractPrice> contract_prices;
    contract_prices.push_back({0, "1"});
    contract_prices.push_back({1, "1"});
    contract_prices.push_back({2, "1"});
    contract_prices.push_back({3, "1"});
    vector<SpotPriceInfo> margin_prices;
    margin_prices.push_back({17, "1"});
    margin_prices.push_back({141, "1"});
    margin_prices.push_back({142, "1"});
    // build the transaction
    AccountId account_id = 3;
    SubAccountId sub_account_id = 1;
    OrderMatchingBuilder builder = {
        account_id, sub_account_id, taker, maker, "1000", 18,
        contract_prices, margin_prices, "808077878", "5479779"
    };
    shared_ptr<OrderMatching> tx = OrderMatching::init(builder);
    shared_ptr<Signer> signer = Signer::init(private_key, L1SignerType::ETH());
    TxSignature signature = signer->sign_order_matching(tx);
    cout << signature.tx << "\n";
    return 0;
}
