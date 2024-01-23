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
     Params  []json.RawMessage `json:"params"`
}

type SubmiterSignature struct {
    PubKey string `json:"pubKey"`
    Signature string `json:"signature"`
}

func HighLevelFunding() {
    privateKey := "0xbe725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"

    var funding_account_ids = make([]sdk.AccountId,3)
    funding_account_ids[0] = sdk.AccountId(55)
    funding_account_ids[1] = sdk.AccountId(56)
    funding_account_ids[2] = sdk.AccountId(57)

    builder := sdk.FundingBuilder{
        sdk.AccountId(14),
        sdk.SubAccountId(3),
        sdk.Nonce(23),
        funding_account_ids,
        *big.NewInt(100000000000),
        sdk.TokenId(17),
    }
    tx := sdk.NewFunding(builder)
    signer, err := sdk.NewSigner(privateKey, sdk.L1SignerTypeEth{})
    if err != nil {
        return
    }
    txSignature, err := signer.SignFunding(tx)
    if err != nil {
        return
    }
    fmt.Println("tx signature: %s", txSignature)
    // create the submitter signature
    zklinkTx := tx.ToZklinkTx()

	rpc_req := RPCTransaction {
		Id:      1,
		JsonRpc: "2.0",
		Method:  "sendTransaction",
		Params: []json.RawMessage{
		    []byte(txSignature.Tx),
            nil
		},
    }
	JsonTx, err := json.Marshal(rpc_req)
	fmt.Println("Funding rpc request:",  string(JsonTx))
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
    HighLevelFunding()
}
