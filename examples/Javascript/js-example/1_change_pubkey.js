import init, *  as wasm  from "./web-dist/zklink-sdk-web.js";

async function testEcdsaAuth() {
    await init();
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
        const signer = new wasm.JsonRpcSigner();
        await signer.initZklinkSigner();
        let tx_signature = await signer.signChangePubkeyWithEthEcdsaAuth(tx,l1_client_id,main_contract);
        console.log(tx_signature);

        let submitter_signature = signer.submitterSignature(tx_signature.tx);
        console.log(submitter_signature);
        //send to zklink
        let rpc_client = new wasm.RpcClient("testnet");
        let tx_hash = await rpc_client.sendTransaction(tx_signature.tx,null,submitter_signature);
        console.log(tx_hash);

    } catch (error) {
        console.error(error);
    }

}

async function testCreate2() {
    await init();
    const private_key = "43be0b8bdeccb5a13741c8fd076bf2619bfc9f6dcc43ad6cf965ab489e156ced";
    const new_pubkey_hash = "0xd8d5fb6a6caef06aa3dc2abdcdc240987e5330fe";
    const ts  = Math.floor(Date.now() / 1000);
    try {
        let tx_builder = new wasm.ChangePubKeyBuilder(
            1,5,1,new_pubkey_hash,18,"100000000000000",
            1,"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001b",
            ts);
        let tx = wasm.newChangePubkey(tx_builder);
        const signer = new wasm.Signer(private_key);
        //auth type 'Create2'
        const creator_address = "0x6E253C951A40fAf4032faFbEc19262Cd1531A5F5";
        const salt = "0x0000000000000000000000000000000000000000000000000000000000000000";
        const code_hash = "0x4f063cd4b2e3a885f61fefb0988cc12487182c4f09ff5de374103f5812f33fe7";
        let create2_data = new wasm.Create2Data(creator_address,salt,code_hash);
        let from_account = "0x4504d5BE8634e3896d42784A5aB89fc41C3d4511";
        let tx_signature = signer.signChangePubkeyWithCreate2DataAuth(tx,create2_data);
        console.log(tx_signature);

        let submitter_signature = signer.submitterSignature(tx_signature.tx);
        console.log(submitter_signature);
        //send to zklink
        let rpc_client = new wasm.RpcClient("devnet");
        let tx_hash = await rpc_client.sendTransaction(tx_signature.tx,null,submitter_signature);
        console.log(tx_hash);

    } catch (error) {
        console.error(error);
    }

}

await testEcdsaAuth();
// await testCreate2();
