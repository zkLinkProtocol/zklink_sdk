const {ContractMatchingBuilder,Signer,newContractMatching,newContract,ContractBuilder,RpcClient } = require('./node-dist/zklink-sdk-node');
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

async function testContractMatching() {
    const private_key = "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
    try {
        let taker_contract_builder = new ContractBuilder(5,1,1,3,2,
            "343434343434","5454545445",true,50,22,false);
        let taker_contract = newContract(taker_contract_builder).jsonValue();

        let maker_contract_builder1 = new ContractBuilder(5,1,1,4,2,
            "556556","898989",false,50,22,true);
        let maker_contract1 = newContract(maker_contract_builder1).jsonValue();

        let maker_contract_builder2 = new ContractBuilder(5,1,1,5,2,
            "54554","78787878",false,50,22,false);
        let maker_contract2 = newContract(maker_contract_builder2).jsonValue();

        let tx_builder = new ContractMatchingBuilder(5,1,taker_contract,[maker_contract1,maker_contract2],"34343",17);
        let tx = newContractMatching(tx_builder);
        console.log(tx);
        const signer = new Signer(private_key);
        let tx_signature = signer.signContractMatching(tx);
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
    await testContractMatching();
}

main();
