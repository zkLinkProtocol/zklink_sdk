const {Wallet, EthTxOption,WaitForTxStatus} = require('./node-dist/zklink-sdk-node');
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
        let status = await wallet.waitForTransaction(deposit_hash);
        if (status === WaitForTxStatus.Success) {
            console.log("success");
        }
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

async function getDepositFee() {
    const private_key = "0xb32593e347bf09436b058fbeabc17ebd2c7c1fa42e542f5f78fc3580faef83b7";
    const zksync_rpc_url = "https://goerli.blockpi.network/v1/rpc/e3c85db2286ea898affeb4a718d3203fdec40b4d";
    const zklink_address = "0x041625fdE341e4A317C3E984C27742e09F5b8659";
    try {
        let option = new EthTxOption(true,zklink_address,null,null,null,null);
        let wallet = new Wallet(zksync_rpc_url,private_key);
        console.log(wallet);
        let fee = await wallet.getDepositFee(option);
        console.log(fee);

    } catch (error) {
        console.error(error);
    }
}


async function testSetAuthPubkyHash() {
    const private_key = "0xb32593e347bf09436b058fbeabc17ebd2c7c1fa42e542f5f78fc3580faef83b7";
    const rpc_url = "https://zksync-era-testnet.blockpi.network/v1/rpc/000b867660d26ff0c900789ab881cf09f1d9377f";
    const zklink_address = "0xa97153dd89c6f8F3BeA66190a6e62020aC7213de";
    try {
        let option = new EthTxOption(false,zklink_address,null,null,null,null);
        let wallet = new Wallet(rpc_url,private_key);
        console.log(wallet);
        let new_pubkey_hash = "0x8255f5a6d0d2b34a19f381e448ed151cc3a59b9e";
        let tx_hash = await wallet.setAuthPubkeyHash(1,new_pubkey_hash,option);
        console.log(tx_hash);

    } catch (error) {
        console.error(error);
    }
}

async function main() {
    await testDepositErc20();
    await testDepositErc20ToGateway();
    await getDepositFee();
    await testSetAuthPubkyHash();
    await testDepositEth();
}

main();
