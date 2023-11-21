const {ChangePubKeyBuilder,Signer,newChangePubkey,RpcClient } = require('./node-dist/zklink-sdk-node');
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
    const main_contract = "0x49ea5715b7aae82e0dece40a9263797e5a12cfb9";
    const l1_client_id = 80001;
    const new_pubkey_hash = "0x0043a38170c9fe8ff718bb86435814468a616044";
    const ts  = Math.floor(Date.now() / 1000);
    try {
        let tx_builder = new ChangePubKeyBuilder(
            2,31,0,new_pubkey_hash,17,"0",
            1,"0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001b",
            ts);
        let tx = newChangePubkey(tx_builder);
        const signer = new Signer(private_key);
        let tx_signature = signer.signChangePubkeyWithEthEcdsaAuth(tx,l1_client_id,main_contract);
        console.log(tx_signature);

        let submitter_signature = signer.submitterSignature(tx_signature.tx);
        console.log(submitter_signature);
        //send to zklink
        let rpc_client = new RpcClient("testnet");
        let tx_hash = await rpc_client.sendTransaction(tx_signature.tx,null,submitter_signature);
        console.log(tx_hash);

    } catch (error) {
        console.error(error);
    }

}

async function main() {
    console.log(global);
    await testEcdsaAuth();
}

main();
