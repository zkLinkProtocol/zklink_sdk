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
        const signer = new Signer(private_key);
        let taker_contract_builder = new ContractBuilder(5,1,1,3,2,
            "10","5454545445",true,50,22,false);
        let unsigned_taker_contract = newContract(taker_contract_builder);
        let taker_contract = signer.createSignedContract(unsigned_taker_contract);
        console.log(taker_contract);

        let maker_contract_builder1 = new ContractBuilder(5,1,1,4,2,
            "556556","898989",false,50,22,true);
        let unsigned_maker_contract1 = newContract(maker_contract_builder1);
        let maker_contract1 = signer.createSignedContract(unsigned_maker_contract1);
        console.log(maker_contract1);

        let maker_contract_builder2 = new ContractBuilder(5,1,1,5,2,
            "54554","78787878",false,50,22,false);
        let unsigned_maker_contract2 = newContract(maker_contract_builder2);
        let maker_contract2 = signer.createSignedContract(unsigned_maker_contract2);
        console.log(maker_contract2);

        let tx_builder = new ContractMatchingBuilder(5,1,taker_contract,[maker_contract1,maker_contract2],"1",17);
        let tx = newContractMatching(tx_builder);
        console.log(tx);
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
