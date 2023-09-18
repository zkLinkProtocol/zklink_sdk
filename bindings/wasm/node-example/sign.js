const {ZklinkSignerWasm,ZklinkSignatureWasm} = require('./node-dist/zklink-sdk-node');

function main() {
    try {
        const msg = new Uint8Array([1, 2, 3, 4, 5]);
        const zklink_signer = ZklinkSignerWasm.NewFromEthSigner("be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4");
        console.log(zklink_signer);
        let l2_signature = zklink_signer.sign(msg);
        console.log(l2_signature);

        let signature = ZklinkSignatureWasm.NewFromHexStr(l2_signature);
        console.log(signature);
        let verify = signature.verify(msg);
        console.log(verify);
    } catch (e) {
       console.error(e);
    }
}

main();
