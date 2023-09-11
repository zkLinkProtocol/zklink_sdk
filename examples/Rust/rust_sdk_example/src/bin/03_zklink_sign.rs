use zklink_signers::zklink_signer::pk_signer::ZkLinkSigner;

fn main() {
    let eth_private_key = "0xb32593e347bf09436b058fbeabc17ebd2c7c1fa42e542f5f78fc3580faef83b7";
    let zklink_signer = ZkLinkSigner::new_from_hex_eth_signer(eth_private_key).unwrap();
    assert_eq!(
        "0x8e3eb3abb0cbf96605956a5313ab239ff685a64562332ac52ef51b9eb8d0d72c",
        zklink_signer.public_key().to_string()
    );

    let message = b"hello world";
    let signature = zklink_signer.sign_musig(message).unwrap();
    let passed = signature.verify_musig(message).unwrap();
    assert!(passed);
}
