const {AutoDeleveragingBuilder,Signer,newAutoDeleveraging,ContractPrice,SpotPriceInfo,RpcClient } = require('./node-dist/zklink-sdk-node');
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

async function testAutoDeleveraging() {
    const private_key = "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
    try {
        const contract_price1 = new ContractPrice(1,"10000000000000");
        const contract_price2 = new ContractPrice(1,"2000000000000");
        let contract_prices = [];
        contract_prices.push(contract_price1.jsonValue());
        contract_prices.push(contract_price2.jsonValue());

        let margin_prices = [];
        const margin_price1 = new SpotPriceInfo(17,"3236653653635635");
        const margin_price2 = new SpotPriceInfo(18,"549574875297");
        margin_prices.push(margin_price1.jsonValue());
        margin_prices.push(margin_price2.jsonValue());
        let tx_builder = new AutoDeleveragingBuilder(5,1,10,contract_prices,margin_prices,3,2,"33535545","188888","199",17);
        let tx = newAutoDeleveraging(tx_builder);
        console.log(tx);
        const signer = new Signer(private_key);
        let tx_signature = signer.signAutoDeleveraging(tx);
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
    await testAutoDeleveraging();
}

main();
