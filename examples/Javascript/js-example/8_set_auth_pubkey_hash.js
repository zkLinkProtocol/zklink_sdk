import init,*  as wasm  from "./web-dist/zklink-sdk-web.js";
import { connect,disconnect } from "starknetkit";
import Contract from "starknet";

async function setStarknetPubkeyHash() {
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
    console.log(starknet);


    let provider = starknet.provider;
    const zklinkAddress = "0x0346dd9e1bd173fc59cda8f2b9bb5dc10d7ecb8ca838ed08847ff3acbef21484";
    const { abi } = await starknet.provider.getClassAt(zklinkAddress);
    const contract = new Contract(abi, zklinkAddress, provider);
    // Connect account with the contract
    contract.connect(starknet.account);
    // get pubkey on chain
    /*const { abi } = await starknet.provider.getClassAt(accountAddress);
    if (!abi) {
    throw new Error("Error while getting ABI");
    }
    const contractAccount = new Contract(abi, accountAddress, starknet.provider);
    console.log(contractAccount);
    const pubKey = await contractAccount.get_owner();*/
    const pubKey = "1082125475812817975721104073212648033952831721853656627074253194227094744819";
    try {
        const signer = wasm.newRpcSignerWithSigner(starknet.account,pubKey,"SN_GOERLI",starknet.selectedAddress);

        // use cached ethereum signature to init zklink signer
        //const signature = "0x1111111111";
        //await signer.initZklinkSigner(signature);
        // use wallet to init the zklink signer
        await signer.initZklinkSigner(null);
        console.log(signer);

        const newPubkeyHash = signer.pubkeyHash();
        console.log(newPubkeyHash);
        contract.connect(starknet.account);
        const res_set_pubkey = await contract.setAuthPubkeyHash(newPubkeyHash,0);
        console.log(res_set_pubkey.transaction_hash);
        await provider.waitForTransaction(res_set_pubkey.transaction_hash);

    } catch (error) {
        console.error(error);
    }

}

await setStarknetPubkeyHash();
