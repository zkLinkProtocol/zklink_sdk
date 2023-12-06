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
     Params  []json.RawMessage `json:"params"`
}

type SubmiterSignature struct {
    PubKey string `json:"pubKey"`
    Signature string `json:"signature"`
}

func LowLevelChangePubkey() {
    privateKey := "0xbe725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"
    // create eth signer
    ethSigner, err := sdk.NewEthSigner(privateKey)
    if err != nil {
        return
    }

    // create zklink signer
	zklinkSigner, err := sdk.ZkLinkSignerNewFromHexEthSigner(privateKey)
	if err != nil {
		return
	}

	chainId := sdk.ChainId(1)
	accountId := sdk.AccountId(2)
	subAccountId := sdk.SubAccountId(4)
    newPkHash:= sdk.PubKeyHash("0xd8d5fb6a6caef06aa3dc2abdcdc240987e5330fe")
    feeToken := sdk.TokenId(1)
    fee := big.NewInt(100)
    nonce := sdk.Nonce(100)
    // TODO: create ethSignature
    ethSignature := sdk.PackedEthSignature("0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001b")
    // get current timestamp
    now := time.Now()
    timeStamp := sdk.TimeStamp(now.Unix())

    // create ChangePubKey transaction type without signed
	builder := sdk.ChangePubKeyBuilder{
		chainId,
		accountId,
		subAccountId,
		newPkHash,
		feeToken,
		*fee,
		nonce,
		&ethSignature,
		timeStamp,
	}
	tx := sdk.NewChangePubKey(builder)

	// create ethAuthData
	// AuthData has 3 types of enum
	// 1. sdk.ChangePubKeyAuthDataOnChain{}
	// 2. sdk.ChangePubKeyAuthDataEthCreate2 { Data: sdk.Create2Data }
	// 3. sdk.ChangePubKeyAuthDataEthEcdsa

    ethSignature, err = sdk.EthSignatureOfChangePubkey(tx, ethSigner);
    if err != nil {
        return
    }
    ethAuthData := sdk.ChangePubKeyAuthDataEthEcdsa {
        EthSignature: ethSignature,
    }

    // sign the changePubKey, add the ethAuthData
    tx, err = sdk.CreateSignedChangePubkey(zklinkSigner, tx, ethAuthData)
    if err != nil {
        return
    }
    // check if the signature is valid
    valid := tx.IsSignatureValid();
    if !valid {
        return
    }
    zklinkTx := tx.ToZklinkTx();
    fmt.Println("changePubKey tx: %v", zklinkTx)

    // create submitter signature
    txHash := tx.TxHash()
    submitterSignature, err := zklinkSigner.SignMusig(txHash)
    if err != nil {
        return
    }
    fmt.Println("changePubKey submitter signature: %v", submitterSignature)
    submitterSignature2, err := json.Marshal(SubmiterSignature {
        PubKey: submitterSignature.PubKey,
        Signature: submitterSignature.Signature,
    })

    // rpc request with `sendTransaction`
	txReq := RPCTransaction {
		Id:      1,
		JsonRpc: "2.0",
		Method:  "sendTransaction",
		Params: []json.RawMessage{
		[]byte(zklinkTx),
		nil,
		[]byte(submitterSignature2)},
    }
	JsonTx, err := json.Marshal(txReq)
	fmt.Println("ChangePubKey rpc request:",  string(JsonTx))
	zklinkUrl := sdk.ZklinkTestNetUrl()
	response, err := http.Post(zklinkUrl, "application/json",bytes.NewBuffer(JsonTx))
	if err != nil {
        fmt.Println(err)
    }
    defer response.Body.Close()
    body, _ := ioutil.ReadAll(response.Body)
    fmt.Println(string(body))
}

func HighLevelChangePubkeyEcdsa() {
    privateKey := "0xbe725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"
	chainId := sdk.ChainId(1)
	accountId := sdk.AccountId(2)
	subAccountId := sdk.SubAccountId(4)
    newPkHash:= sdk.PubKeyHash("0xd8d5fb6a6caef06aa3dc2abdcdc240987e5330fe")
    feeToken := sdk.TokenId(1)
    fee := big.NewInt(100)
    nonce := sdk.Nonce(100)
    // TODO: create ethSignature
    ethSignature := sdk.PackedEthSignature("0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001b")
    // get current timestamp
    now := time.Now()
    timeStamp := sdk.TimeStamp(now.Unix())

    // create ChangePubKey transaction type without signed
	builder := sdk.ChangePubKeyBuilder{
		chainId,
		accountId,
		subAccountId,
		newPkHash,
		feeToken,
		*fee,
		nonce,
		&ethSignature,
		timeStamp,
	}
	tx := sdk.NewChangePubKey(builder)
    signer, err := sdk.NewSigner(privateKey, sdk.L1TypeEth)
    if err != nil {
        return
    }
    txSignature, err := signer.SignChangePubkeyWithEthEcdsaAuth(tx)
    fmt.Println("tx signature: %s", txSignature)

    // get eth signature
    var layer1Signature []byte = nil;
    if txSignature.Layer1Signature != nil {
        layer1Signature = []byte(*txSignature.Layer1Signature)
    }
    // get submitter signature
    submitterSignature, err := signer.SubmitterSignature(txSignature.Tx)
    fmt.Println("submitter signature: %s", submitterSignature)
    submitterSignature2, err := json.Marshal(SubmiterSignature {
        PubKey: submitterSignature.PubKey,
        Signature: submitterSignature.Signature,
    })

    // rpc request with `sendTransaction`
	request := RPCTransaction {
		Id:      1,
		JsonRpc: "2.0",
		Method:  "sendTransaction",
		Params: []json.RawMessage{
		    []byte(txSignature.Tx),
		    layer1Signature,
		    submitterSignature2,
		},
    }
	JsonTx, err := json.Marshal(request)
	fmt.Println("ChangePubKey rpc request:",  string(JsonTx))
	zklinkUrl := sdk.ZklinkTestNetUrl()
	response, err := http.Post(zklinkUrl, "application/json", bytes.NewBuffer(JsonTx))
	if err != nil {
        fmt.Println(err)
    }
    defer response.Body.Close()
    body, _ := ioutil.ReadAll(response.Body)
    fmt.Println(string(body))
}

func main() {
    LowLevelChangePubkey()
    HighLevelChangePubkeyEcdsa()
}
