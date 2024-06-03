import uniffi.zklink_sdk.*;

fun main(args : Array<String>) {
    var private_key = "0xbe725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
    var new_pk_hash = "0xd8d5fb6a6caef06aa3dc2abdcdc240987e5330fe";
    var eth_signature = "0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001b";
    
    var builder = ChangePubKeyBuilder(1u,2u,4u,new_pk_hash,1u,"100",100u,eth_signature,100u);
    var tx = ChangePubKey(builder);
    var signer = Signer(private_key, L1SignerType.Eth);
    var sig = signer.signChangePubkeyWithEthEcdsaAuth(tx);
    println(sig.tx);
    println(sig.layer1Signature);
}
