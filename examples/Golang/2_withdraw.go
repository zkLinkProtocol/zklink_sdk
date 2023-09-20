package main

import (
	"net/http"
	"math/big"
	"encoding/json"
	"fmt"
	"time"
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

func HighLevelWithdraw() {
    privateKey := "0xbe725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"
	accountId := sdk.AccountId(8300)
	subAccountId := sdk.SubAccountId(4)
	toChainId := sdk.ChainId(1)
    toAddress := sdk.ZkLinkAddress("0xAFAFf3aD1a0425D792432D9eCD1c3e26Ef2C42E9")
    l2SourceToken := sdk.TokenId(6)
    l1TargetToken := sdk.TokenId(5)
	amount := *big.NewInt(1000000)
	fee := *big.NewInt(1000)
	nonce := sdk.Nonce(1)
	fastWithdraw := false
	withdrawFeeRatio := uint16(50)
    // get current timestamp
    now := time.Now()
    timestamp := sdk.TimeStamp(now.Unix())

    builder := sdk.WithdrawBuilder{
        accountId,
        subAccountId,
        toChainId,
        toAddress,
        l2SourceToken,
        l1TargetToken,
        amount,
        fee,
        nonce,
        fastWithdraw,
        withdrawFeeRatio,
        timestamp,
    }
    params, err := sdk.BuildWithdrawRequest(privateKey, builder)
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
    HighLevelWithdraw()
}
