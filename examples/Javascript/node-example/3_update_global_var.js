const {UpdateGlobalVarBuilder,UpdateGlobalVar,FundingInfo,newUpdateGlobalVar,Parameter,ParameterType,FundingRate,RpcClient } = require('./node-dist/zklink-sdk-node');
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
    const funding_infos = [new FundingInfo(1,2,"100000").jsonValue(),
        new FundingInfo(1,3,"3333").jsonValue(),
        new FundingInfo(2,5,"456").jsonValue(),
        new FundingInfo(1,4,"8980808098").jsonValue()];
    const parameter = new Parameter(ParameterType.FundingInfos,funding_infos);
    let tx_builder = new UpdateGlobalVarBuilder(1,2,parameter,1000);
    let tx = newUpdateGlobalVar(tx_builder);
    console.log(tx);
}

async function main() {
    console.log(global);
    await testUpdGlobalVar();
}

main();
