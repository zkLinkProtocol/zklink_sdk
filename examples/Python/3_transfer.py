import zklink_sdk as sdk

def main():
    private_key = "0xbe725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"
    account_id = 20
    to_address = "0xAFAFf3aD1a0425D792432D9eCD1c3e26Ef2C42E9"
    from_sub_account_id = 1
    to_sub_account_id = 1
    token = 18
    amount = "1234567899808787"
    print("Original amount: " + amount)
    assert not sdk.is_token_amount_packable(amount)
    amount = sdk.closest_packable_token_amount(amount)
    assert sdk.is_token_amount_packable(amount)
    print("Converted amount: " + amount)
    fee = "10000567777"
    print("Original fee: " + fee)
    assert not sdk.is_fee_amount_packable(fee)
    fee = sdk.closest_packable_fee_amount(fee)
    assert sdk.is_fee_amount_packable(fee)
    print("Converted fee: " + fee)
    nonce = 1
    timestamp = 1693472232
    token_sybmol = "DAI"

    builder = sdk.TransferBuilder(
        account_id,
        to_address,
        from_sub_account_id,
        to_sub_account_id,
        token,
        amount,
        fee,
        nonce,
        timestamp
    )
    tx = sdk.Transfer(builder)
    signer = sdk.Signer(private_key, sdk.L1SignerType.ETH())

    tx_signature = signer.sign_transfer(tx, "USDT", None, None)
    print(tx_signature)

if __name__ == "__main__":
    main()
