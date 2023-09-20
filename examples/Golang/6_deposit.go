package main

import (
	"net/http"
	"math/big"
	"encoding/json"
	"fmt"
	"bytes"
	"io/ioutil"
	sdk "github.com/zkLinkProtocol/zklink_sdk/go_example/generated/uniffi/zklink_sdk"
)

type RPCTransaction struct {
     Id      int64             `json:"id"`
     JsonRpc string            `json:"jsonrpc"`
     Method  string            `json:"method"`
     Params  json.RawMessage `json:"params"`
}

func HighLevelDeposit() {
    fromAddress := sdk.ZkLinkAddress("0xAFAFf3aD1a0425D792432D9eCD1c3e26Ef2C42E9")
    toAddress := sdk.ZkLinkAddress("0xAFAFf3aD1a0425D792432D9eCD1c3e26Ef2C42E9")
    ethHash := "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
    builder := sdk.DepositBuilder {
      fromAddress,
      toAddress,
      sdk.ChainId(1),
      sdk.SubAccountId(2),
      sdk.TokenId(3),
      sdk.TokenId(4),
      *big.NewInt(100),
      100,
      ethHash,
    }
    params, err := sdk.BuildDepositRequest(builder)
    if err != nil {
        return
    }
	rpc_req := RPCTransaction {
		Id:      1,
		JsonRpc: "2.0",
		Method:  "sendTransaction",
		Params: json.RawMessage(params),
    }
	JsonTx, err := json.Marshal(rpc_req)
	fmt.Println("ChangePubKey rpc request:",  string(JsonTx))
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
    HighLevelDeposit()
}
