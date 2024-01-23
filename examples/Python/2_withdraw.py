import time
import zklink_sdk as sdk

def main():
    private_key = "0xbe725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"
    account_id = 8300
    sub_account_id = 4
    to_chain_id = 5
    to_address = "0xAFAFf3aD1a0425D792432D9eCD1c3e26Ef2C42E9"
    l2_source_token = 2
    l1_target_token = 17
    amount = "100000"
    fee = "1000"
    nonce = 1
    withdraw_fee_ratio = 50
    timestamp = int(time.time())

    builder = sdk.WithdrawBuilder(
        account_id,
        to_chain_id,
        sub_account_id,
        to_address,
        l2_source_token,
        l1_target_token,
        amount,
        None,
        fee,
        nonce,
        True,
        withdraw_fee_ratio,
        timestamp
    )
    tx = sdk.Withdraw(builder)
    signer = sdk.Signer(private_key, sdk.L1SignerType.ETH())
    tx_signature = signer.sign_withdraw(tx, "USDT", None, None)
    print(tx_signature)

if __name__ == "__main__":
    main()
