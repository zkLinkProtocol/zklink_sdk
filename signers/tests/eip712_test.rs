use ethers_primitives::Address;
use serde::{Deserialize, Serialize};
use serde_eip712::{eip712_encode_type, eip712_hash_struct, eip712_type_definitions};
use serde_json::json;
use zklink_sdk_signers::eth_signer::eip712::eip712::{EIP712Domain, TypedData};

#[test]
fn test_mail() {
    let domain_val = json!({
        "name": "Ether Mail",
        "version": "1",
        "chainId": 1,
        "verifyingContract": "CcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC",
    });

    let domain_1: EIP712Domain = serde_json::from_value(domain_val.clone()).unwrap();

    let type_definitions = eip712_type_definitions(&domain_1).unwrap();

    assert_eq!(
        hex::encode(eip712_hash_struct("EIP712Domain", &type_definitions, &domain_1).unwrap()),
        "f2cee375fa42b42143804025fc449deafd50cc031ca257e0b194a650a912090f"
    );

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Person {
        pub name: String,
        pub wallet: Address,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Mail {
        pub from: Person,
        pub to: Person,
        pub contents: String,
    }

    let json = json!({
      "from": {
        "name": "Cow",
        "wallet": "0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826"
      },
      "to": {
        "name": "Bob",
        "wallet": "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB"
      },
      "contents": "Hello, Bob!"
    });

    let mail: Mail = serde_json::from_value(json).expect("parse domain");

    assert_eq!(
        "Mail(Person from,Person to,string contents)Person(string name,address wallet)",
        eip712_encode_type(&mail).expect("generate e712 types")
    );

    let expect_request: TypedData<Mail> =
        serde_json::from_str(include_str!("./eip712.json")).unwrap();

    assert_eq!(
        TypedData::<Mail>::new(domain_1, mail).unwrap(),
        expect_request
    );

    assert_eq!(
        hex::encode(expect_request.sign_hash().unwrap()),
        "be609aee343fb3c4b28e1df9e632fca64fcfaede20f02e86244efddf30957bd2"
    );

    let expect_request: TypedData<serde_json::Value> =
        serde_json::from_str(include_str!("./eip712.json")).unwrap();

    assert_eq!(
        hex::encode(expect_request.sign_hash().unwrap()),
        "be609aee343fb3c4b28e1df9e632fca64fcfaede20f02e86244efddf30957bd2"
    );

    expect_request.sign_hash().unwrap();
}
