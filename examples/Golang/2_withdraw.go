package main

import (
	sdk "github.com/zkLinkProtocol/zklink_sdk/go_example/generated/uniffi/zklink_sdk"
	"github.com/stretchr/testify/assert"
	"math/big"
	"net/http"
	"fmt"
)


type RPCTransaction struct {
     Id      int64             `json:"id"`
     JsonRpc string            `json:"jsonrpc"`
     Method  string            `json:"method"`
     Params  []json.RawMessage `json:"params"`
}

func main() {
    privateKey := "0xbe725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"
    // create eth signer
    eth_signer, err = sdk.NewPrivateKeySigner(privateKey)
    if err != nil {
        return
    }

    // create zklink signer
	zkSigner, err := sdk.ZkLinkSignerNewFromHexEthSigner(ethPrivateKey)
	if err != nil {
		return
	}

	accountId := sdk.AccountId(8300)
	subAccountId := sdk.SubAccountId(4)
	toChainId := ChainId(1)
    toAddress := sdk.ZkLinkAddress("0xAFAFf3aD1a0425D792432D9eCD1c3e26Ef2C42E9")
    l2SourceToken := sdk.TokenId(1)
    l1TargetToken := sdk.TokenId(2)
	amount := *big.NewInt(1000000)
	fee := *big.NewInt(1000)
	nonce := sdk.Nonce(1)
	fastWithdraw := false
	withdrawFeeRatio := 50
    // get current timestamp
    now := time.Now()
    timeStamp := sdk.TimeStamp(now.Unix())

    tx := sdk.NewWithdraw(
        accountId,
        subAccountId,
        toChainId,
        to_address,
        l2SourceToken,
        l1TargetToken,
        amount,
        fee,
        nonce,
        fastWithdraw,
        timeStamp,
    )
    // zklink sign
    signed_tx, err := sdk.CreateSignedWithdraw(
        zklink_signer,
        tx,
    )
    if err != nil {
        return
    }
    should_be_valid, err := signed_tx.IsSignatureValid();
    if err != nil {
        return
    }
    if !should_be_valid {
        fmt.Println("invalid signature")
        return
    }
    txJsonStr = signed_tx.JsonStr()
    fmt.Printf("%v\n", txJsonStr)

    // create submitter signature
    bytes := tx.GetBytes()
    submitter_signature, err := sdk.CreateSubmitterSignature(bytes, zklink_signer)
    assert.Nil(t, err)
    // get the json string of submitter signature
	json_str_of_submitter_signature = sdk.JsonStrOfZklinkSignature(submitter_signature)
    fmt.Println("changePubKey submitter signature: %v", json_str_of_submitter_signature)

    // create eth signature
    tokenSymbol = "DAI"
    ethSignature, err = signed_tx.EthSignature(eth_signer, tokenSymbol)

    // rpc request with `sendTransaction`
	zkLinkUrl = sdk.ZkLinkTestNetUrl()
	tx := RPCTransaction{
		Id:      1,
		JsonRpc: "2.0",
		Method:  "sendTransaction",
		Params: []json.RawMessage{
		[]bytes(txJsonStr),
		[]bytes(ethSignature),
		[]bytes(json_str_of_submitter_signature),
	}
	JsonTx, err := json.Marshal(tx)
	fmt.Println("ChangePubKey rpc request:",  string(JsonTx))
	// get the testnet url or main net url
	zklinkUrl = sdk.ZkLinkTestNetUrl()
	response, err := http.Post(zklinkUrl, "application/json",bytes.NewBuffer(JsonTx))
	if err != nil {
        fmt.Println(err)
        return
    }
    defer resp.Body.Close()
    body, _ := ioutil.ReadAll(resp.Body)
    fmt.Println(string(body))
}
