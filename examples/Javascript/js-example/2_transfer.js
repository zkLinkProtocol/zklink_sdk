import init, *  as wasm  from "./web-dist/zklink-sdk-web.js";
import { connect,disconnect } from "starknetkit";
import { Contract  } from "starknet";

async function testEvmChains() {
    await init();
    const to_address = "0x5505a8cD4594Dbf79d8C59C0Df1414AB871CA896";
    const ts  = Math.floor(Date.now() / 1000);
    try {
        let amount = wasm.closestPackableTransactionAmount("1234567899808787");
        let fee = wasm.closestPackableTransactionFee("10000567777")
        let tx_builder = new wasm.TransferBuilder(10, to_address, 1,
            1, 18, fee, amount, 1,ts);
        let transfer = wasm.newTransfer(tx_builder);
        const provider = new providers.Web3Provider(window.ethereum);
        const etherenumSigner = provider.getSigner();
        const signer = new wasm.newEthereumRpcSigner(etherenumSigner);
        await signer.initZklinkSigner(null);
        console.log(signer);

        let signature = await signer.signTransfer(transfer,"USDC")
        console.log(signature);

        let rpc_client = new wasm.RpcClient("testnet");
        let l1_signature = new wasm.TxLayer1Signature(wasm.L1SignatureType.Eth,signature.layer1_signature.signature);
        let tx_hash = await rpc_client.sendTransaction(signature.tx,l1_signature);
        console.log(tx_hash);

    } catch (error) {
        console.error(error);
    }

}

async function testStarknet() {
    await init();
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

    //get pubkey from account contract
    const { abi } = await starknet.provider.getClassAt(accountAddress);
    if (!abi) {
    throw new Error("Error while getting ABI");
    }
    const contractAccount = new Contract(abi, accountAddress, starknet.provider);
    console.log(contractAccount);
    const pubKey = await contractAccount.get_owner();

    const to_address = "0x0322546b712D87B8565C33530A6396D85f024F2C99ff564019a5Fc4c38e0F740";
    const ts  = Math.floor(Date.now() / 1000);
    try {
        let amount = wasm.closestPackableTransactionAmount("1234567899808787");
        let fee = wasm.closestPackableTransactionFee("10000567777")
        let tx_builder = new wasm.TransferBuilder(10, to_address, 1,
            1, 18, fee, amount, 1,ts);
        let transfer = wasm.newTransfer(tx_builder);
        const signer = wasm.newRpcSignerWithSigner(starknet.account,pubKey,"SN_GOERLI",starknet.selectedAddress);
        await signer.initZklinkSigner(null);
        console.log(signer);

        let signature = await signer.signTransfer(transfer,"USDC")
        console.log(signature);

        let rpc_client = new wasm.RpcClient("testnet");
        let l1_signature = new wasm.TxLayer1Signature(wasm.L1SignatureType.Eth,signature.eth_signature);
        let tx_hash = await rpc_client.sendTransaction(signature.tx,l1_signature);
        console.log(tx_hash);

    } catch (error) {
        console.error(error);
    }
}

await testEvmChains();
await testStarknet();
