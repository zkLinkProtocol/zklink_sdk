import init, { ZklinkSignerWasm }  from  "./web-dist/zklink-sdk-web.js";

async function main() {
    await init();
    const signer = ZklinkSignerWasm.NewFromEthSigner("be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4");
    const msg_str = "hello world!";
    const msg = new TextEncoder().encode(msg_str);
    try {
        let signature = signer.sign(msg);
        console.log(signature);
    } catch (error) {
        console.error(error);
    }

}

main();
