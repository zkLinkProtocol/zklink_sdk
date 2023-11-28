import init, *  as wasm  from "./web-dist/zklink-sdk-web.js";

async function main() {
    await init();
    try {
        const provider = window.bitkeep && window.bitkeep.ethereum;
        await provider.request({ method: 'eth_requestAccounts' });
        const signer = new wasm.JsonRpcSigner(provider);
        await signer.initZklinkSigner(null);
        console.log(signer);

        //maker = taker = submitter
        let maker_order = new wasm.Order(5,1,1,1,18,17,"10000000000000","10000000000",true,5,3);
        let maker = signer.createSignedOrder(maker_order);
        console.log(maker);
        let taker_order = new wasm.Order(5,1,1,1,18,17,"10000000000000","10000000000",false,5,3);
        let taker = signer.createSignedOrder(taker_order);
        console.log(taker);
        let tx_builder = new wasm.OrderMatchingBuilder(10, 1, taker, maker, "1000000000", 18,"10000000000000000", "10000000000000000");
        let order_matching = wasm.newOrderMatching(tx_builder);
        let signature = signer.signOrderMatching(order_matching);
        console.log(signature);

        let submitter_signature = signer.submitterSignature(signature.tx);
        console.log(submitter_signature);
        let rpc_client = new wasm.RpcClient("testnet");
        let tx_hash = await rpc_client.sendTransaction(signature.tx,null,submitter_signature);
        console.log(tx_hash);

    } catch (error) {
        console.error(error);
    }

}

main();
