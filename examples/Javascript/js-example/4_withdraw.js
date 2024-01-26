import init, *  as wasm  from "./web-dist/zklink-sdk-web.js";

async function main() {
    await init();
    const to_address = "0x5505a8cD4594Dbf79d8C59C0Df1414AB871CA896";
    const ts  = Math.floor(Date.now() / 1000);
    try {
        let tx_builder = new wasm.WithdrawBuilder(10, 1, 1, to_address,18,
            10,"100000000000000", null,"10000000000000000",1,false,50,ts);
        let withdraw = wasm.newWithdraw(tx_builder);
        const provider = new providers.Web3Provider(window.ethereum);
        const etherenumSigner = provider.getSigner();
        const signer = new wasm.newEthereumRpcSigner(etherenumSigner);
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
