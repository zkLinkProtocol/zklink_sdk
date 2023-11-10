const {UpdateGlobalVarBuilder,UpdateGlobalVar,newUpdateGlobalVar,Parameter,ParameterType,FundingRate,RpcClient } = require('./node-dist/zklink-sdk-node');
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

async function testUpdGlobalVar() {
    const funding_rates = [new FundingRate(1,2).jsonValue(),
        new FundingRate(1,3).jsonValue(),
        new FundingRate(2,5).jsonValue(),
        new FundingRate(1,4).jsonValue()];
    const parameter = new Parameter(ParameterType.FundingRates,funding_rates);
    //const parameter = new Parameter(ParameterType.FeeAccount,1);
    let tx_builder = new UpdateGlobalVarBuilder(1,2,parameter,1000);
    let tx = newUpdateGlobalVar(tx_builder);
    console.log(tx);
}

async function main() {
    console.log(global);
    await testUpdGlobalVar();
}

main();
