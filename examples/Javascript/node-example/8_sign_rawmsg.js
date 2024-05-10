const {Signer, L1Type } = require('./node-dist/zklink-sdk-node');
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

async function testSignRawmsg() {
    const private_key = "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
    try {
        const signer = new Signer(private_key, L1Type.Eth);
        const zklinkSig = signer.signMusig(new TextEncoder().encode("SIGNED MESSAGE"));
        console.log("pubkey: " + zklinkSig.pubKey());
        console.log("signature: " + zklinkSig.signature());
    } catch (error) {
        console.error(error);
    }

}

async function main() {
    await testSignRawmsg();
}

main();
