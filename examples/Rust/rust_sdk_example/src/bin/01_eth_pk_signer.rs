use std::str::FromStr;
use zklink_signers::eth_signer::pk_signer::EthSigner;
use zklink_signers::eth_signer::Address;

fn main() {
    let private_key = "0xb32593e347bf09436b058fbeabc17ebd2c7c1fa42e542f5f78fc3580faef83b7";
    let pk_signer = EthSigner::try_from(private_key).unwrap();

    let address = pk_signer.get_address().unwrap();
    assert_eq!(
        address,
        Address::from_str("0x9e372368c25056D44045e445d72d7B91cE3eE3B1").unwrap()
    );

    let message = b"hello world";
    let signature = pk_signer.sign_message(message).unwrap();
    assert_eq!(signature.as_hex().as_str(), "0xa9aa0710adb18f84d4bed8057382fc433c3dcff1bddf3b2b1c2cb11386ef3be4172b5d0688143759d4e744acc434ae4f96575c7fa9096971fd02fb3d2aaa77121c");

    let recover_addr = signature.signature_recover_signer(message).unwrap();
    assert_eq!(recover_addr, address);
}
