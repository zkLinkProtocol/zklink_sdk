#include <iostream>
#include "generated/zklink_sdk.hpp"

using namespace std;
using namespace zklink_sdk;

int main() {
    string private_key = "0xbe725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
    ChainId chain_id = 1;
    AccountId account_id = 2;
    SubAccountId sub_account_id = 4;
    PubKeyHash new_pk_hash = "0xd8d5fb6a6caef06aa3dc2abdcdc240987e5330fe";
    TokenId fee_token = 1;
    BigUint fee = "100";
    Nonce nonce = 100;
    PackedEthSignature eth_signature = "0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001b";
    TimeStamp timestamp = 100;
    ChangePubKeyBuilder builder = {
        chain_id, account_id, sub_account_id, new_pk_hash, fee_token, fee,
        nonce, eth_signature, timestamp
    };
    shared_ptr<ChangePubKey> tx = ChangePubKey::init(builder);
    shared_ptr<Signer> signer = Signer::init(private_key, L1SignerType::ETH());
    TxSignature signature = signer->sign_change_pubkey_with_eth_ecdsa_auth(tx);
    cout << signature.tx << "\n";
    return 0;
}