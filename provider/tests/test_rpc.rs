#[cfg(test)]
mod test {
    use jsonrpsee::http_client::HttpClientBuilder;
    use std::str::FromStr;
    use zklink_sdk_provider::ZkLinkRpcClient;
    use zklink_sdk_signers::eth_signer::EthSigner;
    use zklink_sdk_signers::zklink_signer::{PubKeyHash, ZkLinkSigner};
    use zklink_sdk_types::basic_types::BigUint;
    use zklink_sdk_types::basic_types::{
        AccountId, ChainId, Nonce, SubAccountId, TimeStamp, TokenId, ZkLinkAddress,
    };
    use zklink_sdk_types::tx_builder::ChangePubKeyBuilder;
    use zklink_sdk_types::tx_type::change_pubkey::ChangePubKey;
    use zklink_sdk_types::tx_type::zklink_tx::ZkLinkTx;
    use zklink_sdk_types::tx_type::ZkSignatureTrait;

    #[tokio::test]
    async fn test_send_change_pubkey() {
        let private_key = "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
        let eth_signer = EthSigner::try_from(private_key).unwrap();
        let zklink_signer = ZkLinkSigner::new_from_hex_eth_signer(private_key).unwrap();
        let main_contract = "0x5505a8cD4594Dbf79d8C59C0Df1414AB871CA896";
        let l1_client_id = 80001;
        let new_pubkey_hash = "0xd8d5fb6a6caef06aa3dc2abdcdc240987e5330fe";
        let ts = 1696595303;
        //auth type 'ECDSA'
        let builder = ChangePubKeyBuilder {
            chain_id: ChainId(1),
            account_id: AccountId(10),
            sub_account_id: SubAccountId(1),
            new_pubkey_hash: PubKeyHash::from_hex(new_pubkey_hash).unwrap(),
            fee_token: TokenId(18),
            fee: BigUint::from(100000000000000u64),
            nonce: Nonce(1),
            eth_signature: None,
            timestamp: TimeStamp(ts),
        };
        let change_pubkey = ChangePubKey::new(builder);
        let message = change_pubkey
            .to_eip712_request_payload(
                l1_client_id,
                &ZkLinkAddress::from_str(main_contract).unwrap(),
            )
            .unwrap();
        let signature = eth_signer
            .sign_message(message.raw_data.as_bytes())
            .unwrap();
        let builder_with_sig = ChangePubKeyBuilder {
            chain_id: ChainId(1),
            account_id: AccountId(10),
            sub_account_id: SubAccountId(1),
            new_pubkey_hash: PubKeyHash::from_hex(new_pubkey_hash).unwrap(),
            fee_token: TokenId(18),
            fee: BigUint::from(100000000000000u64),
            nonce: Nonce(1),
            eth_signature: Some(signature),
            timestamp: TimeStamp(ts),
        };
        let mut tx = ChangePubKey::new(builder_with_sig);
        tx.sign(&zklink_signer).unwrap();
        let submitter_signature = tx.submitter_signature(&zklink_signer).unwrap();

        //use jsonrpsee
        let client = HttpClientBuilder::default()
            .build("https://dev-gw-v1.zk.link")
            .unwrap();
        let ret = client
            .tx_submit(
                ZkLinkTx::ChangePubKey(Box::new(tx.clone())),
                None,
                Some(submitter_signature),
            )
            .await
            .unwrap();
        println!("{:?}", ret)
    }
}
