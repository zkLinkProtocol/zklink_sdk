package main

import (
	"net/http"
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
     Params  []json.RawMessage `json:"params"`
}

type SubmiterSignature struct {
    PubKey string `json:"pubKey"`
    Signature string `json:"signature"`
}

func HighLevelUpdateGlobalVar() {
    privateKey := "0xbe725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"
    parameter := sdk.ParameterFeeAccount {
        AccountId: sdk.AccountId(0),
    }
    builder := sdk.UpdateGlobalVarBuilder {
        sdk.ChainId(1),
        sdk.SubAccountId(1),
        parameter,
        101,
    }

    tx := sdk.NewUpdateGlobalVar(builder)
    signer, err := sdk.NewSigner(privateKey, sdk.L1SignerTypeEth{ Net:"eth"})
    if err != nil {
        return
    }

    // get submitter signature
    zklinkTx := tx.ToZklinkTx()
    submitterSignature, err := signer.SubmitterSignature(zklinkTx)
    submitterSignature2, err := json.Marshal(SubmiterSignature {
        PubKey: submitterSignature.PubKey,
        Signature: submitterSignature.Signature,
    })

	rpc_req := RPCTransaction {
		Id:      1,
		JsonRpc: "2.0",
		Method:  "sendTransaction",
		Params: []json.RawMessage{
		    []byte(zklinkTx),
		    nil,
		    submitterSignature2,
		},
    }
	JsonTx, err := json.Marshal(rpc_req)
	fmt.Println("UpdateGlobalVar rpc request:",  string(JsonTx))
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
    HighLevelUpdateGlobalVar()
}
