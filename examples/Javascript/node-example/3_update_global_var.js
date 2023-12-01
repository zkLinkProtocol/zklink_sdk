const {UpdateGlobalVarBuilder,Signer,FundingInfo,newUpdateGlobalVar,Parameter,ParameterType,FundingRate,RpcClient } = require('./node-dist/zklink-sdk-node');
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
    const parameter_infos = new Parameter(ParameterType.FundingInfos,funding_infos);
    // fee_account
    const parameter = new Parameter(ParameterType.FeeAccount,10)
    let tx_builder = new UpdateGlobalVarBuilder(1,2,parameter,1000);
    let tx = newUpdateGlobalVar(tx_builder);
    const signer = new Signer(private_key);
    const submitterSignature = await signer.submitterSignature(tx.zklinkTx());
    console.log(submitterSignature);
}

async function main() {
    console.log(global);
    await testUpdGlobalVar();
}

main();
