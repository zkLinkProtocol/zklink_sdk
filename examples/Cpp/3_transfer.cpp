#include <iostream>
#include "generated/zklink_sdk.hpp"

using namespace std;
using namespace zklink_sdk;

int main() {
    string private_key = "0xbe725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
    AccountId account_id = 20;
    ZkLinkAddress to_address = "0xAFAFf3aD1a0425D792432D9eCD1c3e26Ef2C42E9";
    SubAccountId from_sub_account_id = 1;
    SubAccountId to_sub_account_id = 1;
    TokenId token = 18;
    BigUint amount = "1234567899808787";
    cout << "Original amount: " << amount << "\n";
    amount = closest_packable_token_amount(amount);
    cout << "Converted amount: " << amount << "\n";
    BigUint fee = "10000567777";
    cout << "Original fee: " << fee << "\n";
    fee = closest_packable_fee_amount(fee);
    cout << "Converted fee: " << fee << "\n";
    Nonce nonce = 1;
    TimeStamp timestamp = 1693472232;
    string token_sybmol = "DAI";

    TransferBuilder builder = {
        account_id,
        to_address,
        from_sub_account_id,
        to_sub_account_id,
        token,
        amount,
        fee,
        nonce,
        timestamp
    };
    shared_ptr<Transfer> tx = Transfer::init(builder);
    shared_ptr<Signer> signer = Signer::init(private_key, L1SignerType::kEth{});
    TxSignature signature = signer->sign_transfer(tx, token_sybmol, {}, {});
    cout << signature.tx << "\n";
    return 0;
}
