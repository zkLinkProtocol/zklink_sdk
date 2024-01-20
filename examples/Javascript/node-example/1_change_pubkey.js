const {ChangePubKeyBuilder,Signer,newChangePubkey,RpcClient,L1Type } = require('./node-dist/zklink-sdk-node');
// CommonJS
const fetch = require('node-fetch');
const AbortController = require('abort-controller')

// @ts-ignore
global.fetch = fetch;
// @ts-ignore
global.Headers = fetch.Headers;
// @ts-ignore
global.Request = fetch.Request;
// @ts-ignore
global.Response = fetch.Response;
// @ts-ignore
global.AbortController = AbortController;

async function testEcdsaAuth() {
    const private_key = "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
    const new_pubkey_hash = "0x8255f5a6d0d2b34a19f381e448ed151cc3a59b9e";
    const ts  = Math.floor(Date.now() / 1000);
    try {
        let tx_builder = new ChangePubKeyBuilder(
            16,21,0,new_pubkey_hash,140,"10",
            0,"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001b",
            ts);
        let tx = newChangePubkey(tx_builder);
        const signer = new Signer(private_key);
        let tx_signature = signer.signChangePubkeyWithEthEcdsaAuth(tx);
        console.log(tx_signature);

        //send to zklink
        let rpc_client = new RpcClient("testnet");
        let tx_hash = await rpc_client.sendTransaction(tx_signature.tx,null);
        console.log(tx_hash);

    } catch (error) {
        console.error(error);
    }

}

async function testOnchainAuth() {
    const private_key = "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
    const new_pubkey_hash = "0x8255f5a6d0d2b34a19f381e448ed151cc3a59b9e";
    const ts  = Math.floor(Date.now() / 1000);
    try {
        let tx_builder = new ChangePubKeyBuilder(
            16,21,0,new_pubkey_hash,140,"1",
            0,"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001b",
            ts);
        let tx = newChangePubkey(tx_builder);
        let addr = "0x04A69b67bcaBfA7D3CCb96e1d25C2e6fC93589fE24A6fD04566B8700ff97a71a";
        const signer = new Signer(private_key,L1Type.Starknet,"SN_GOERLI",addr);
        let tx_signature = signer.signChangePubkeyWithOnchain(tx);
        console.log(tx_signature);

        //let submitter_signature = signer.submitterSignature(tx_signature.tx);
        //console.log(submitter_signature);
        //send to zklink
        let rpc_client = new RpcClient("custum","http://127.0.0.1:3030");
        let tx_hash = await rpc_client.sendTransaction(tx_signature.tx,null);
        console.log(tx_hash);

    } catch (error) {
        console.error(error);
    }

}

async function main() {
    await testEcdsaAuth();
    await testOnchainAuth();
}

main();
