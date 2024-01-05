import zklink_sdk as sdk

def main():
    private_key = "0xbe725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"
    chain_id = 1
    account_id = 2
    sub_account_id = 4
    new_pk_hash = "0xd8d5fb6a6caef06aa3dc2abdcdc240987e5330fe"
    fee_token = 1
    fee = "100"
    nonce = 100
    eth_signature = "0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001b"
    timestamp = 100
    builder = sdk.ChangePubKeyBuilder(chain_id, account_id, sub_account_id, new_pk_hash, fee_token, fee, nonce, eth_signature, timestamp)
    tx = sdk.ChangePubKey(builder)
    signer = sdk.Signer(private_key, sdk.L1SignerType.ETH())
    signature = signer.sign_change_pubkey_with_eth_ecdsa_auth(tx)
    print(signature.tx)
    print(signature.layer1_signature)
    submitter_signature = signer.submitter_signature(signature.tx)
    print(submitter_signature.pub_key)
    print(submitter_signature.signature)

if __name__ == "__main__":
    main()

