const {ContractMatchingBuilder,Signer,newContractMatching,newContract,ContractBuilder,
    RpcClient,ContractPrice,SpotPriceInfo } = require('./node-dist/zklink-sdk-node');
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
        const contract_price1 = new ContractPrice(0,"1");
        const contract_price2 = new ContractPrice(1,"1");
        const contract_price3 = new ContractPrice(2,"1");
        const contract_price4 = new ContractPrice(3,"1")
        let contract_prices = [];
        contract_prices.push(contract_price1.jsonValue());
        contract_prices.push(contract_price2.jsonValue());
        contract_prices.push(contract_price3.jsonValue());
        contract_prices.push(contract_price4.jsonValue());

        let margin_prices = [];
        const margin_price1 = new SpotPriceInfo(17,"1");
        const margin_price2 = new SpotPriceInfo(141,"1");
        const margin_price3 = new SpotPriceInfo(142,"1");
        margin_prices.push(margin_price1.jsonValue());
        margin_prices.push(margin_price2.jsonValue());
        margin_prices.push(margin_price3.jsonValue());

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

        let tx_builder = new ContractMatchingBuilder(5,1,taker_contract,
            [maker_contract1,maker_contract2],"1",17,contract_prices,margin_prices);
        let tx = newContractMatching(tx_builder);
        console.log(tx);
        let tx_signature = signer.signContractMatching(tx);
        console.log(tx_signature);

        //send to zklink
        let rpc_client = new RpcClient("testnet");
        let tx_hash = await rpc_client.sendTransaction(tx_signature.tx,null);
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
