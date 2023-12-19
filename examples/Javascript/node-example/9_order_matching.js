const {OrderMatchingBuilder,Signer,ContractPrice,newOrderMatching,Order,RpcClient,SpotPriceInfo } = require('./node-dist/zklink-sdk-node');
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

async function testOrderMatching() {
    const private_key = "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
    try {
        const signer = new Signer(private_key);
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
        let maker_order = new Order(5,20,1,1,18,17,"10000000000000","10000000000",true,5,3);
        let maker = signer.createSignedOrder(maker_order);
        console.log(maker);
        let taker_order = new Order(5,20,1,1,18,17,"10000000000000","10000000000",false,5,3);
        let taker = signer.createSignedOrder(taker_order);

        let tx_builder = new OrderMatchingBuilder(5,20,taker,maker,"11",17,contract_prices,margin_prices,"4343433","3957485749");
        let tx = newOrderMatching(tx_builder);
        console.log(tx);
        let tx_signature = signer.signOrderMatching(tx);
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
    await testOrderMatching();
}

main();
