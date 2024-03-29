import init, *  as wasm  from "./web-dist/zklink-sdk-web.js";
async function main() {
    await init();
    try {
        let client = new wasm.RpcClient("testnet");
        // 1.getSupportTokens
        let tokens = await client.getSupportTokens();
        console.log(tokens);
        // 2.getAccountSnapshot
        let account_id = new wasm.AccountQuery(wasm.AccountQueryType.AccountId, "5");
        let sub_account_id = 1;
        // let block_number = 100;
        let account_resp = await client.getAccountSnapshot(account_id,sub_account_id,null);
        console.log(account_resp);
        // 3.sendTransaction(test on the tx example)
        // 4.getSupportChains
        let chains = await client.getSupportChains();
        console.log(chains);
        console.log(chains.length)
        // 5.getLatestBlockNumber
        let block_info = await client.getLatestBlockNumber();
        console.log(block_info);
        // 6.getBlockByNumber
        let block_detail = await client.getBlockByNumber(100,true,true);
        console.log(block_detail);
        // 7.getPendingBlock
        let pending_block_info = await client.getPendingBlock(1696743981000n,true,true,null);
        console.log(pending_block_info);
        // 8.getBlockOnChainByNumber
        let on_chain_block_info = await client.getBlockOnChainByNumber(100);
        console.log(on_chain_block_info);
        // 9.getAccount
        let get_account_id = new wasm.AccountQuery(wasm.AccountQueryType.AccountId, "10");
        let account = await client.getAccount(get_account_id);
        console.log(account);
        // 10.getAccountBalances
        let balances = await client.getAccountBalances(20,1);
        console.log(balances);
        // 11.getAccountOrderSlots
        let slots = await client.getAccountOrderSlots(20,1);
        console.log(slots);
        // 12.getTokenReserve
        let reserve = await client.getTokenReserve(18,false);
        console.log(reserve);
        // 13.getTransactionByHash
        let tx_hash = "0x0cbeabac1a2257fb095c2465e148570e32793345442b39bf64cad4ed87475f9b";
        let tx_info = await client.getTransactionByHash(tx_hash,false);
        console.log(tx_info);
        // 14.getAccountTransactionHistory
        let history = await client.getAccountTransactionHistory(wasm.ZkLinkTxType.Deposit,"0x12aFF993702B5d623977A9044686Fa1A2B0c2147",0n,5);
        console.log(history);
        // 15.getFastWithdrawTxs
        let fast_withdraw_txs = await client.getFastWithdrawTxs(1696743981000n,10);
        console.log(fast_withdraw_txs);
        // 16.pullForwardTxs
    } catch (error) {
        console.error(error);
    }

}

main();
