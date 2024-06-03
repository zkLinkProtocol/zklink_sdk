#include <iostream>
#include "zklink_sdk.hpp"

using namespace std;
using namespace zklink_sdk;

int main() {
    string private_key = "0xbe725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
    AccountId account_id = 8300;
    SubAccountId sub_account_id = 4;
    ChainId to_chain_id = 5;
    ZkLinkAddress to_address = "0xAFAFf3aD1a0425D792432D9eCD1c3e26Ef2C42E9";
    TokenId l2_source_token = 2;
    TokenId l1_target_token = 17;
    BigUint amount = "100000";
    BigUint fee = "1000";
    Nonce nonce = 1;
    uint16_t withdraw_fee_ratio = 50;
    TimeStamp timestamp = 100;

    WithdrawBuilder builder = {
        account_id, sub_account_id, to_chain_id, to_address,
        l2_source_token, l1_target_token, amount, {},
        fee, nonce, withdraw_fee_ratio, true, timestamp
    };
    shared_ptr<Withdraw> tx = Withdraw::init(builder);
    shared_ptr<Signer> signer = Signer::init(private_key, L1SignerType::ETH());
    TxSignature signature = signer->sign_withdraw(tx, "USDT", {}, {});
    cout << signature.tx << "\n";
    return 0;
}
