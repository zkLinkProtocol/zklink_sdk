const {Wallet, EthTxOption} = require('./node-dist/zklink-sdk-node');
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

async function testDepositEth() {
    const private_key = "0xd511266f7d37f0957564e4ce1a1dcc8bb3408383634774a2f4a94a35f4bc53e0";
    const eth_rpc_url = "https://linea-goerli.blockpi.network/v1/rpc/0348a3ff6425076c6c8e9b26cc7dbbdb8a6ff3f0";
    const zklink_address = "0x63A7Dd7B68f569eDA9bFefd1041b8e56b885e5E9";
    const sub_account_id = 10;
    const deposit_to = "0x4770a669b28092251940f35165d2c94f7d47359a";
    // 0.1 eth
    const amount = "100000000000000000"
    try {
        let approve_option = new EthTxOption(true, zklink_address, null, amount, null, null);
        let wallet = new Wallet(eth_rpc_url, private_key);
        console.log(wallet);
        let hash = await wallet.depositETH(sub_account_id, deposit_to, approve_option, false)
        console.log(hash);
    } catch(error) {
        console.log(error)
    }
}

async function testDepositErc20() {
    const private_key = "0xb32593e347bf09436b058fbeabc17ebd2c7c1fa42e542f5f78fc3580faef83b7";
    const avax_rpc_url = "https://avalanche-fuji.blockpi.network/v1/rpc/383a4619d008fe876a25060ff8e66a0f6465c3de";
    const zklink_address = "0xa1552D819a3F83f459bDF5dB3390CA16d056f74A";
    const usdc_address = "0xb9337d4da91117566050c37c75a2cf96f4bcb875";
    const user_address = "0x9e372368c25056D44045e445d72d7B91cE3eE3B1";
    try {
        let approve_option = new EthTxOption(true,usdc_address,null,null,null,null);
        let wallet = new Wallet(avax_rpc_url,private_key);
        console.log(wallet);
        let approve_hash = await wallet.approveERC20(zklink_address,"1000000",approve_option);
        console.log(approve_hash);

        let deposit_option = new EthTxOption(true,zklink_address,null,null,null,null);
        let deposit_hash = await wallet.depositERC20(1,user_address,usdc_address,
            "1000000",false,deposit_option,false);
        console.log(deposit_hash);

    } catch (error) {
        console.error(error);
    }
}

async function testDepositErc20ToGateway() {
    const private_key = "0xb32593e347bf09436b058fbeabc17ebd2c7c1fa42e542f5f78fc3580faef83b7";
    const eth_rpc_url = "https://goerli.blockpi.network/v1/rpc/e3c85db2286ea898affeb4a718d3203fdec40b4d";
    const gateway_address = "0xa1552D819a3F83f459bDF5dB3390CA16d056f74A";
    const usdc_address = "0xb9337d4da91117566050c37c75a2cf96f4bcb875";
    const user_address = "0x9e372368c25056D44045e445d72d7B91cE3eE3B1";
    try {
        let approve_option = new EthTxOption(true,usdc_address,null,null,null,null);
        let wallet = new Wallet(eth_rpc_url,private_key);
        console.log(wallet);
        let approve_hash = await wallet.approveERC20(gateway_address,"1000000",approve_option);
        console.log(approve_hash);

        let deposit_option = new EthTxOption(true,zklink_address,null,null,null,null);
        let deposit_hash = await wallet.depositERC20(1,user_address,usdc_address,
            "1000000",false,deposit_option,true);
        console.log(deposit_hash);

    } catch (error) {
        console.error(error);
    }
}

async function main() {
    console.log(global);
    await testDepositErc20();
    await testDepositErc20ToGateway();
    await testDepositEth();
}

main();
