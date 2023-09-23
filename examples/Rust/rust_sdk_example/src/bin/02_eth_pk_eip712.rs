use serde::{Deserialize, Serialize};
use serde_json::json;
use zklink_signers::eth_signer::eip712::eip712::{EIP712Domain, TypedData};
use zklink_signers::eth_signer::pk_signer::EthSigner;
use zklink_signers::eth_signer::EIP712Address;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Person {
    pub name: String,
    pub wallet: EIP712Address,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Mail {
    pub from: Person,
    pub to: Person,
    pub contents: String,
}
fn main() {
    let private_key = "0xb32593e347bf09436b058fbeabc17ebd2c7c1fa42e542f5f78fc3580faef83b7";
    let pk_signer = EthSigner::try_from(private_key).unwrap();

    let message = json!(
      {
        "from": {
          "name": "Cow",
          "wallet": "0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826"
        },
        "to": {
          "name": "Bob",
          "wallet": "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB"
        },
        "contents": "Hello, Bob!"
      }
    );

    let message: Mail = serde_json::from_value(message).expect("parse domain");
    let domain = EIP712Domain::new(
        "Ether Mail".into(),
        "1".into(),
        1,
        "0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC".into(),
    )
    .unwrap();

    let typed_data = TypedData::new(domain, message).unwrap();

    let signature = pk_signer
        .sign_byted_data(&typed_data.sign_hash().unwrap())
        .unwrap();

    assert_eq!(signature.as_hex(), "0xbf24877c59766e95717686e71a0402ba12f5db4a8aa93ac6c30b5742925ebfc26c91d6b6bb949a2b0578c397e296830dde9cc3531adbb259c4b4b06441b1a9c51b");
}
