const {UpdateGlobalVarBuilder,ContractInfo,Signer,FundingInfo,MarginInfo,newUpdateGlobalVar,Parameter,ParameterType,FundingRate,RpcClient } = require('./node-dist/zklink-sdk-node');
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
    const private_key = "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
    //funding_infos
    const funding_infos = [new FundingInfo(1,2,"100000").jsonValue(),
        new FundingInfo(1,3,"3333").jsonValue(),
        new FundingInfo(2,5,"456").jsonValue(),
        new FundingInfo(1,4,"8980808098").jsonValue()];
    const parameter_funding = new Parameter(ParameterType.FundingInfos,funding_infos);
    // contract_info
    const contract_info = new ContractInfo(1,"USDC/USDT",5,10).jsonValue();
    console.log(contract_info);
    const parameter_contract = new Parameter(ParameterType.ContractInfo,contract_info)
    // margin_info
    const margin_info = new MarginInfo(2,"USDC/USDT",17,10).jsonValue();
    const parameter = new Parameter(ParameterType.MarginInfo,margin_info)
    console.log(parameter);

    let tx_builder = new UpdateGlobalVarBuilder(1,8,parameter,1000);
    console.log(tx_builder);
    let tx = newUpdateGlobalVar(tx_builder);
    console.log(tx.jsonValue());
}

async function main() {
    console.log(global);
    await testUpdGlobalVar();
}

main();
