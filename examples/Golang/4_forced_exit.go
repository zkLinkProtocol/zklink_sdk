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

func HighLevelForcedExit() {
    privateKey := "0xbe725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"
    address := sdk.ZkLinkAddress("0xAFAFf3aD1a0425D792432D9eCD1c3e26Ef2C42E9")
    builder := sdk.ForcedExitBuilder{
        ToChainId: sdk.ChainId(5),
        InitiatorAccountId: sdk.AccountId(11),
        TargetSubAccountId: sdk.SubAccountId(1),
        Target: address,
        InitiatorSubAccountId: sdk.SubAccountId(1),
        L2SourceToken: sdk.TokenId(18),
        L1TargetToken: sdk.TokenId(18),
        InitiatorNonce: sdk.Nonce(1),
        ExitAmount: *big.NewInt(100000),
        WithdrawToL1: false,
        Timestamp: sdk.TimeStamp(1693472232),
    }
    tx := sdk.NewForcedExit(builder)
    signer, err := sdk.NewSigner(privateKey, sdk.L1SignerTypeEth{ Net:"eth"})
    if err != nil {
        return
    }
    txSignature, err := signer.SignForcedExit(tx)
    if err != nil {
        return
    }
    fmt.Println("tx signature: %s", txSignature)
    // create the submitter signature
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
		    []byte(txSignature.Tx),
		    nil,
            submitterSignature2,
		},
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
    HighLevelForcedExit()
}
