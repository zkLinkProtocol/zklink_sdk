package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	sdk "github.com/zkLinkProtocol/zklink_sdk/go_example/generated/uniffi/zklink_sdk"
	"io/ioutil"
	"math/big"
	"net/http"
)

type RPCTransaction struct {
	Id      int64             `json:"id"`
	JsonRpc string            `json:"jsonrpc"`
	Method  string            `json:"method"`
	Params  []json.RawMessage `json:"params"`
}

type SubmiterSignature struct {
	PubKey    string `json:"pubKey"`
	Signature string `json:"signature"`
}

func HighLevelTransfer() {
	privateKey := "0xbe725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"
	address := sdk.ZkLinkAddress("0xAFAFf3aD1a0425D792432D9eCD1c3e26Ef2C42E9")
	builder := sdk.TransferBuilder{
		sdk.AccountId(20),
		address,
		sdk.SubAccountId(1),
		sdk.SubAccountId(1),
		sdk.TokenId(18),
		*big.NewInt(100000),
		*big.NewInt(100),
		sdk.Nonce(1),
		sdk.TimeStamp(1693472232),
	}
	tokenSymbol := "DAI"
	tx := sdk.NewTransfer(builder)
	signer, err := sdk.NewSigner(privateKey, sdk.L1SignerTypeEth{})
	if err != nil {
		return
	}
	txSignature, err := signer.SignTransfer(tx, tokenSymbol, nil, nil)
	if err != nil {
		return
	}
	fmt.Println("tx signature: %s", txSignature)
	// get the eth signature
	var layer1Signature []byte = nil
	if txSignature.Layer1Signature != nil {
		layer1Signature = []byte(*txSignature.Layer1Signature)
	}

	rpc_req := RPCTransaction{
		Id:      1,
		JsonRpc: "2.0",
		Method:  "sendTransaction",
		Params: []json.RawMessage{
			[]byte(txSignature.Tx),
			layer1Signature,
		},
	}
	JsonTx, err := json.Marshal(rpc_req)
	if err != nil {
		fmt.Println("error rpc req: %s", err)
		return
	}
	fmt.Println("ChangePubKey rpc request:", string(JsonTx))
	// get the testnet url or main net url
	zklinkUrl := sdk.ZklinkTestNetUrl()
	response, err := http.Post(zklinkUrl, "application/json", bytes.NewBuffer(JsonTx))
	if err != nil {
		fmt.Println(err)
		return
	}
	defer response.Body.Close()
	body, _ := ioutil.ReadAll(response.Body)
	fmt.Println(string(body))
}

func main() {
	HighLevelTransfer()
}
