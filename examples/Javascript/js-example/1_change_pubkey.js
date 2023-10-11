import init, *  as wasm  from "./web-dist/zklink-sdk-web.js";

async function main() {
    await init();
    const private_key = "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
    const main_contract = "0x5505a8cD4594Dbf79d8C59C0Df1414AB871CA896";
    const l1_client_id = 80001;
    const new_pubkey_hash = "0xd8d5fb6a6caef06aa3dc2abdcdc240987e5330fe";
    const ts  = Math.floor(Date.now() / 1000);
    try {
        let tx_builder = new wasm.ChangePubKeyBuilder(
            1,5,1,new_pubkey_hash,18,"100000000000000",
            1,"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001b",
            ts);
        let tx = wasm.newChangePubkey(tx_builder);
        const signer = new wasm.Signer(private_key);
        //auth type 'ECDSA'
        let tx_signature = signer.signChangePubkeyWithEthEcdsaAuth(tx,l1_client_id,main_contract);
        console.log(tx_signature);
        //auth type 'Create2'
        //0x0000000000000000000000000000000000000000000000000000000000000000
        let submitter_signature = signer.SubmitterSignature(tx_signature.tx);
        console.log(submitter_signature);
        //send to zklink
        let rpc_client = new wasm.RpcClient("testnet");
        let tx_hash = await rpc_client.sendTransaction(tx_signature.tx,null,submitter_signature);
        console.log(tx_hash);

    } catch (error) {
        console.error(error);
    }

}

main();
