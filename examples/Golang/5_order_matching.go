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

func HighLevelOrderMatching() {
    privateKey := "0xbe725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"
    // create zklink signer
	zklinkSigner, err := sdk.ZkLinkSignerNewFromHexEthSigner(privateKey)
	if err != nil {
		return
	}
    taker := sdk.NewOrder(
        sdk.AccountId(10),
        sdk.SubAccountId(1),
        sdk.SlotId(3),
        sdk.Nonce(1),
        sdk.TokenId(18),
        sdk.TokenId(145),
        *big.NewInt(323289),
        *big.NewInt(135),
        true,
        true,
        2,
        5,
        nil,
    )
    taker, err = taker.CreateSignedOrder(
        zklinkSigner,
    )

    maker := sdk.NewOrder(
         sdk.AccountId(20),
         sdk.SubAccountId(1),
         sdk.SlotId(3),
         sdk.Nonce(1),
         sdk.TokenId(18),
         sdk.TokenId(145),
         *big.NewInt(323355),
         *big.NewInt(135),
         false,
         true,
         2,
         5,
         nil,
    )
    maker, err = maker.CreateSignedOrder(
        zklinkSigner,
    )

    contract_price1 := sdk.ContractPrice{
        sdk.PairId(0),
        *big.NewInt(1),
    }

    contract_price2 := sdk.ContractPrice{
        sdk.PairId(1),
        *big.NewInt(1),
    }

    contract_price3 := sdk.ContractPrice{
        sdk.PairId(2),
        *big.NewInt(1),
    }

    contract_price4 := sdk.ContractPrice{
        sdk.PairId(3),
        *big.NewInt(1),
    }

    var contract_prices = make([]sdk.ContractPrice,4)
    contract_prices[0] = contract_price1
    contract_prices[1] = contract_price2
    contract_prices[2] = contract_price3
    contract_prices[3] = contract_price4

    margin_price1 := sdk.SpotPriceInfo {
       sdk.TokenId(17),
       *big.NewInt(1),
    }
    margin_price2 := sdk.SpotPriceInfo {
      sdk.TokenId(141),
      *big.NewInt(1),
    }

    margin_price3 := sdk.SpotPriceInfo {
      sdk.TokenId(142),
      *big.NewInt(1),
    }
    var margin_prices = make([]sdk.SpotPriceInfo,3)
    margin_prices[0] = margin_price1
    margin_prices[1] = margin_price2
    margin_prices[2] = margin_price3
    builder := sdk.OrderMatchingBuilder{
        sdk.AccountId(3),
        sdk.SubAccountId(1),
        taker,
        maker,
        *big.NewInt(1000),
        sdk.TokenId(18),
        contract_prices,
        margin_prices,
        *big.NewInt(808077878),
        *big.NewInt(5479779),
    }
    tx := sdk.NewOrderMatching(builder)
    signer, err := sdk.NewSigner(privateKey, sdk.L1TypeEth)
    if err != nil {
        return
    }
    txSignature, err := signer.SignOrderMatching(tx)
    if err != nil {
        return
    }
    fmt.Println("tx signature: %s", txSignature)

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
    HighLevelOrderMatching()
}
