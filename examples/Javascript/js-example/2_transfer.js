import init, *  as wasm  from "./web-dist/zklink-sdk-web.js";

async function main() {
    await init();
    const private_key = "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
    const eth_signer = wasm.EthPrivateKeySigner.newFromHexPrivateKey(private_key);
    const zklink_signer = wasm.ZklinkSigner.newFromEthSigner(private_key);
    const to_address = "0x5505a8cD4594Dbf79d8C59C0Df1414AB871CA896";
    const ts  = Math.floor(Date.now() / 1000);
    let tx = new wasm.Transfer(10, to_address, 1, 1, 18, "100000000000000", "10000000000000000", 1,ts);
    let message = tx.getEthSignMessage("USDT");
    const msg = new TextEncoder().encode(message);
    try {
        let signature = eth_signer.signMessage(msg);
        console.log(signature);
        let tx_signature = tx.sign(zklink_signer);
        console.log(tx_signature);
        tx.setL2Signature(tx_signature);
        //send to zklink
        let provder = new wasm.Provider("testnet");
        let signed_tx = new wasm.SignedTransaction(tx.getTxType(),tx.getTx());
        console.log(signed_tx);
        let l1_signature = new wasm.TxL1Signature(wasm.L1SignatureType.Eth,signature);
        let submitter_signature = tx.submitterSign(zklink_signer);
        console.log(submitter_signature);
        let tx_hash = await provder.sendTransaction(signed_tx,l1_signature,submitter_signature);
        console.log(tx_hash);

    } catch (error) {
        console.error(error);
    }

}

main();
