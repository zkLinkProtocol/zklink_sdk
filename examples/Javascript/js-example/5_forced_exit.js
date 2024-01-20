import init, *  as wasm  from "./web-dist/zklink-sdk-web.js";

async function main() {
    await init();
    const to_address = "0x5505a8cD4594Dbf79d8C59C0Df1414AB871CA896";
    const ts  = Math.floor(Date.now() / 1000);
    try {
        let tx_builder = new wasm.ForcedExitBuilder(1,10, 1, 1, to_address,18, 18,"100000000000000",  1,ts);
        let forced_exit = wasm.newForcedExit(tx_builder);
        const provider = window.bitkeep && window.bitkeep.ethereum;
        await provider.request({ method: 'eth_requestAccounts' });
        const signer = new wasm.newRpcSignerWithProvider(provider);
        await signer.initZklinkSigner(null);
        console.log(signer);
        let signature = signer.signForcedExit(forced_exit)
        console.log(signature);

        let rpc_client = new wasm.RpcClient("testnet");
        let tx_hash = await rpc_client.sendTransaction(signature.tx,null);
        console.log(tx_hash);

    } catch (error) {
        console.error(error);
    }

}

main();
