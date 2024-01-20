import init, *  as wasm  from "./web-dist/zklink-sdk-web.js";

async function main() {
    await init();
    const to_address = "0x5505a8cD4594Dbf79d8C59C0Df1414AB871CA896";
    const ts  = Math.floor(Date.now() / 1000);
    try {

        let tx_builder = new wasm.WithdrawBuilder(10, 1, 1, to_address,18, "100000000000000", false,10,18,"10000000000000000", 1,ts);
        let withdraw = wasm.newWithdraw(tx_builder);
        const provider = window.bitkeep && window.bitkeep.ethereum;
        await provider.request({ method: 'eth_requestAccounts' });
        const signer = new wasm.newRpcSignerWithProvider(provider);
        await signer.initZklinkSigner(null);
        console.log(signer);

        let signature = await signer.signWithdraw(withdraw,"USDC")
        console.log(signature);

        let rpc_client = new wasm.RpcClient("testnet");
        let l1_signature = new wasm.TxLayer1Signature(wasm.L1SignatureType.Eth,signature.layer1_signature.signature);
        let tx_hash = await rpc_client.sendTransaction(signature.tx,l1_signature);
        console.log(tx_hash);

    } catch (error) {
        console.error(error);
    }

}

main();
