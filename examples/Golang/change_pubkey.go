package main

import (
	sdk "github.com/zkLinkProtocol/zklink_sdk/go_example/generated/uniffi/zklink_sdk"
	"github.com/stretchr/testify/assert"
	"testing"
	"math/big"
	"fmt"
)


type RPCTransaction struct {
     Id      int64             `json:"id"`
     JSONRpc string            `json:"jsonrpc"`
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

	chainId := sdk.ChainId(1)
	accountId := sdk.AccountId(2)
	subAccountId := sdk.SubAccountId(4)
    newPkHash:= sdk.PubKeyHash("0xd8d5fb6a6caef06aa3dc2abdcdc240987e5330fe")
    feeToken := big.NewInt(1000)
    nonce := sdk.Nonce(100)
    // TODO: create ethSignature
    ethSignature := sdk.PackedEthSignature("0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001b")
    // get current timestamp
    now := time.Now()
    timeStamp := sdk.TimeStamp(now.Unix())

    // create ChangePubKey transaction type without signed
	changePubKey := sdk.NewChangePubKey(
		chainId,
		accountId,
		subAccountId,
		newPkHash,
		feeToken,
		fee,
		nonce,
		ethSignature,
		req.Ts,
	)

	// create AuthData
	// AuthData has 3 types of enum
	// 1. sdk.ChangePubKeyAuthDataOnChain{}
	// 2. sdk.ChangePubKeyAuthDataEthCreate2 { Data: Create2Data }
	// 3. sdk.ChangePubKeyAuthDataEthEcdsa

	// TODO: use real main contract address
    main_contract := sdk.ZkLinkAddress("0x0000000000000000000000000000000000000000")
    l1_client_id := uint32(1)
    ethSignature, err := sdk.EthSignatureOfChangePubkey(l1_client_id, tx, eth_signer, main_contract);
    assert.Nil(t, err)
    ethAuthData := sdk.ChangePubKeyAuthDataEthEcdsa {
        EthSignature: ethSignature,
    }

    // sign the changePubKey, add the ethAuthData
    tx, err = sdk.CreateSignedChangePubkey(zklink_signer, tx, ethAuthData)
    assert.Nil(t, err)
    // check if the signature is valid
    valid, err := tx.IsSignatureValid();
    assert.Equal(t, valid, true)
    txJsonStr = tx.JsonStr()
    fmt.Println("changePubKey tx: %v", txJsonStr)

    // create submitter signature
    bytes := tx.GetBytes()
    submitter_signature, err := sdk.CreateSubmitterSignature( bytes, zklink_signer)
    assert.Nil(t, err)
	json_str_of_submitter_signature = sdk.JsonStrOfZklinkSignature(submitter_signature)
    fmt.Println("changePubKey submitter signature: %v", json_str_of_submitter_signature)

    // rpc request with `sendTransaction`
	zkLinkUrl = sdk.ZkLinkTestNetUrl()
	tx := RPCTransaction{
		Id:      1,
		JSONRpc: "2.0",
		Method:  "sendTransaction",
		Params: []json.RawMessage{
		[]bytes(txJsonStr),
		nil,
		[]bytes(json_str_of_submitter_signature),
	}
	JsonTx, err := json.Marshal(tx)
	fmt.Println("ChangePubKey rpc request:",  string(JsonTx))
	zklinkUrl = sdk.ZkLinkTestNetUrl()
	response, err := post.PostJson(zklinkUrl, JsonTx)
	fmt.Println("ChangePubKey rpc response:",  response, "err", err)
}
