package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	sdk "github.com/zkLinkProtocol/zklink_sdk/go_example/generated/uniffi/zklink_sdk"
	"io/ioutil"
	"math/big"
	"net/http"
	"time"
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

func HighLevelWithdraw() {
	privateKey := "0xbe725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"
	accountId := sdk.AccountId(8300)
	subAccountId := sdk.SubAccountId(4)
	toChainId := sdk.ChainId(5)
	toAddress := sdk.ZkLinkAddress("0xAFAFf3aD1a0425D792432D9eCD1c3e26Ef2C42E9")
	l2SourceToken := sdk.TokenId(17)
	l1TargetToken := sdk.TokenId(17)
	amount := *big.NewInt(1000000)
	fee := *big.NewInt(1000)
	nonce := sdk.Nonce(1)
	withdrawFeeRatio := uint16(50)
	// get current timestamp
	now := time.Now()
	timestamp := sdk.TimeStamp(now.Unix())

<<<<<<< HEAD
	builder := sdk.WithdrawBuilder{
		AccountId:     accountId,
		ToChainId:     toChainId,
		SubAccountId:  subAccountId,
		ToAddress:     toAddress,
		L2SourceToken: l2SourceToken,
		L1TargetToken: l1TargetToken,
		Amount:        amount,
		nil,
		Fee:              fee,
		Nonce:            nonce,
		WithdrawToL1:     true,
		WithdrawFeeRatio: withdrawFeeRatio,
		Timestamp:        timestamp,
	}
	tx := sdk.NewWithdraw(builder)
	signer, err := sdk.NewSigner(privateKey, sdk.L1SignerTypeEth{})
	if err != nil {
		return
	}
	txSignature, err := signer.SignWithdraw(tx, "USDT", nil, nil)
	fmt.Println("tx signature: %s", txSignature)
	if err != nil {
		return
	}
	// get the eth signature
	var layer1Signature []byte = nil
	if txSignature.Layer1Signature != nil {
		layer1Signature = []byte(*txSignature.Layer1Signature)
	}
=======
    builder := sdk.WithdrawBuilder{
        AccountId: accountId,
        ToChainId: toChainId,
        SubAccountId: subAccountId,
        ToAddress: toAddress,
        L2SourceToken: l2SourceToken,
        L1TargetToken: l1TargetToken,
        Amount: amount,
        nil,
        Fee: fee,
        Nonce: nonce,
        WithdrawToL1: true,
        WithdrawFeeRatio: withdrawFeeRatio,
        Timestamp: timestamp,
    }
    tx := sdk.NewWithdraw(builder)
    signer, err := sdk.NewSigner(privateKey, sdk.L1SignerTypeEth{})
    if err != nil {
        return
    }
    txSignature, err := signer.SignWithdraw(tx, "USDT",nil,nil)
    fmt.Println("tx signature: %s", txSignature)
    if err != nil {
        return
    }
    // get the eth signature
    var layer1Signature []byte = nil;
    if txSignature.Layer1Signature != nil {
        layer1Signature = []byte(*txSignature.Layer1Signature)
    }
>>>>>>> fix review suggests

	// create the submitter signature
	zklinkTx := tx.ToZklinkTx()
	submitterSignature, err := signer.SubmitterSignature(zklinkTx)
	submitterSignature2, err := json.Marshal(SubmiterSignature{
		PubKey:    submitterSignature.PubKey,
		Signature: submitterSignature.Signature,
	})
	rpc_req := RPCTransaction{
		Id:      1,
		JsonRpc: "2.0",
		Method:  "sendTransaction",
		Params: []json.RawMessage{
			[]byte(txSignature.Tx),
			layer1Signature,
			submitterSignature2,
		},
	}

	JsonTx, err := json.Marshal(rpc_req)
	if err != nil {
		fmt.Println(err)
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
	HighLevelWithdraw()
}
