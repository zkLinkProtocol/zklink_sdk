import init, *  as wasm  from "./web-dist/zklink-sdk-web.js";

async function testEcdsaAuth() {
    await init();
    const new_pubkey_hash = "0xd8d5fb6a6caef06aa3dc2abdcdc240987e5330fe";
    const ts  = Math.floor(Date.now() / 1000);
    try {
        let tx_builder = new wasm.ChangePubKeyBuilder(
            1,5,1,new_pubkey_hash,18,"100000000000000",
            1,"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001b",
            ts);
        let tx = wasm.newChangePubkey(tx_builder);
        //use stand window.ethereum as metamask ..
        //await window.ethereum.request({ method: 'eth_requestAccounts' });
        //const provider = window.ethereum;

        //use not stand window.ethereum as bitget ..
        const provider = window.bitkeep && window.bitkeep.ethereum;
        await provider.request({ method: 'eth_requestAccounts' });

        const signer = new wasm.newRpcSignerWithProvider(provider);

        // use cached ethereum signature to init zklink signer
        //const signature = "0x1111111111";
        //await signer.initZklinkSigner(signature);
        // use wallet to init the zklink signer
        await signer.initZklinkSigner(null);
        console.log(signer);

        let tx_signature = await signer.signChangePubkeyWithEthEcdsaAuth(tx);
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
    const new_pubkey_hash = "0xd8d5fb6a6caef06aa3dc2abdcdc240987e5330fe";
    const ts  = Math.floor(Date.now() / 1000);
    try {
        let tx_builder = new wasm.ChangePubKeyBuilder(
            1,5,1,new_pubkey_hash,18,"100000000000000",
            1,"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001b",
            ts);
        let tx = wasm.newChangePubkey(tx_builder);
        const provider = window.bitkeep && window.bitkeep.ethereum;
        await provider.request({ method: 'eth_requestAccounts' });
        const signer = new wasm.newRpcSignerWithProvider(provider);
        await signer.initZklinkSigner(null);
        console.log(signer);

        //auth type 'Create2'
        const creator_address = "0x6E253C951A40fAf4032faFbEc19262Cd1531A5F5";
        const salt = "0x0000000000000000000000000000000000000000000000000000000000000000";
        const code_hash = "0x4f063cd4b2e3a885f61fefb0988cc12487182c4f09ff5de374103f5812f33fe7";
        let create2_data = new wasm.Create2Data(creator_address,salt,code_hash);
        let tx_signature = await signer.signChangePubkeyWithCreate2DataAuth(tx,create2_data);
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
