import java.lang.reflect.*;
import uniffi.zklink_sdk.*;

public class ChangePubkey_1 {
    public static void main(String[] args) {
        String private_key = "0xbe725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
        byte chain_id = 1;
        int account_id = 2;
        byte sub_account_id = 4;
        String new_pk_hash = "0xd8d5fb6a6caef06aa3dc2abdcdc240987e5330fe";
        int fee_token = 1;
        String fee = "100";
        int nonce = 100;
        String eth_signature = "0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001b";
        int timestamp = 100;

        try {
            Class c;
            c = Class.forName("uniffi.zklink_sdk.ChangePubKeyBuilder");
            ChangePubKeyBuilder builder = (ChangePubKeyBuilder) c.getConstructors()[0].newInstance(
                chain_id,account_id,sub_account_id,new_pk_hash,fee_token,fee,nonce,eth_signature,timestamp,null
            );
            ChangePubKey tx = new ChangePubKey(builder);
            c = Class.forName("uniffi.zklink_sdk.L1SignerType$Eth");
            Constructor t = c.getDeclaredConstructors()[0];
            t.setAccessible(true);
            L1SignerType eth = (L1SignerType) t.newInstance();
            Signer signer = new Signer(private_key, eth);
            TxSignature sig = signer.signChangePubkeyWithEthEcdsaAuth(tx);
            System.out.println(sig.getTx());
            System.out.println(sig.getLayer1Signature());
         } catch (Throwable e) {
            System.out.println(e);
        }
    }
}
