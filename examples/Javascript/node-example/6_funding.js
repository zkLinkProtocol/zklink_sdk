const {FundingBuilder,Signer,newFunding,RpcClient } = require('./node-dist/zklink-sdk-node');
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

async function testFunding() {
    const private_key = "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
    try {
        const signer = new Signer(private_key);
        let tx_builder = new FundingBuilder(5,1,2,[3,4,5],"34343",17);
        let tx = newFunding(tx_builder);
        console.log(tx);
        let tx_signature = signer.signFunding(tx);
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
    await testFunding();
}

main();
