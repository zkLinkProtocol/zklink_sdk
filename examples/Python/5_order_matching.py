import zklink_sdk as sdk

def main():
    private_key = "0xbe725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"
    zklink_signer = sdk.ZkLinkSigner.new_from_hex_eth_signer(private_key)
    taker = sdk.Order( 10, 1, 3, 1, 18, 145, "323289", "135", True, True, 2, 5, None)
    taker = taker.create_signed_order(zklink_signer)
    maker = sdk.Order( 10, 1, 3, 1, 18, 145, "323289", "135", True, True, 2, 5, None)
    maker = maker.create_signed_order(zklink_signer)
    contract_price1 = sdk.ContractPrice(0, "1")
    contract_price2 = sdk.ContractPrice(1, "1")
    contract_price3 = sdk.ContractPrice(2, "1")
    contract_price4 = sdk.ContractPrice(3, "1")
    contract_prices = [contract_price1, contract_price2, contract_price3, contract_price4]
    margin_prices = [sdk.SpotPriceInfo(17, "1"), sdk.SpotPriceInfo(141, "1"), sdk.SpotPriceInfo(142, "1")]
    # build the transaction
    account_id = 3
    sub_account_id = 1
    builder = sdk.OrderMatchingBuilder(account_id, sub_account_id, taker, maker, "1000", 18, contract_prices, margin_prices, "808077878", "5479779")
    tx = sdk.OrderMatching(builder)
    signer = sdk.Signer(private_key, sdk.L1SignerType.ETH())
    tx_signature = signer.sign_order_matching(tx)
    print(tx_signature)

    # build submitter signature
    zklink_tx = tx.to_zklink_tx()

if __name__ == "__main__":
    main()
