import { connect,disconnect } from "starknetkit";
import Contract from "starknet";

async function starknetDeposit() {
    await disconnect();
    const starknet = await connect();

    if (!starknet) {
        throw Error("User rejected wallet selection or silent connect found nothing")
    }

    // (optional) connect the wallet
    await starknet.enable();
    if (starknet.isConnected === false) {
        console.log("not connected");
        return;
    }
    console.log(starknet);

    let provider = starknet.provider;
    let token_address = "0x05dea2a666a1542d6b39bb6f6cd914c6c1383e47bd183141b1ded0f341aa733f";
    const zklinkAddress = "0x0346dd9e1bd173fc59cda8f2b9bb5dc10d7ecb8ca838ed08847ff3acbef21484";
    //1.approve
    const erc20_abi = await starknet.provider.getClassAt(token_address);
    const erc20_contract = new Contract(erc20_abi.abi, token_address, provider);
    erc20_contract.connect(starknet.account);
    const call_data_approve = await erc20_contract.populate("approve",[zklinkAddress,100000000]);
    const res_approve = await erc20_contract.approve(call_data_approve.calldata);
    console.log(res_approve.transaction_hash);
    await provider.waitForTransaction(res_approve.transaction_hash);
    //let bal = await erc20_contract.balanceOf(starknet.selectedAddress);
    //console.log(bal);


    //2.deposit
    const { abi } = await starknet.provider.getClassAt(zklinkAddress);
    const contract = new Contract(abi, zklinkAddress, provider);
    // Connect account with the contract
    contract.connect(starknet.account);
    const res = await contract.depositERC20(token_address,1,starknet.selectedAddress,0,false);
    console.log(res.transaction_hash);
    await provider.waitForTransaction(res.transaction_hash);

}

await starknetDeposit();
