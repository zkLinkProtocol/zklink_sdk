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

func HighLevelContractMatching() {
    privateKey := "0xbe725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"
    // create zklink signer
	zklinkSigner, err := sdk.ZkLinkSignerNewFromHexEthSigner(privateKey)
	if err != nil {
		return
	}
	taker_contract_builder := sdk.ContractBuilder {
        sdk.AccountId(18),
        sdk.SubAccountId(1),
        sdk.SlotId(2),
        sdk.Nonce(10),
        sdk.PairId(1),
        *big.NewInt(45454),
        *big.NewInt(113),
        true,
        5,
        3,
        false,
    }

    unsigned_taker_contract := sdk.NewContract(taker_contract_builder)
    taker_contract, err := unsigned_taker_contract.CreateSignedContract(
        zklinkSigner,
    )

    maker_contract1_builder := sdk.ContractBuilder {
        sdk.AccountId(3),
        sdk.SubAccountId(1),
        sdk.SlotId(2),
        sdk.Nonce(6),
        sdk.PairId(1),
        *big.NewInt(43434),
        *big.NewInt(6767),
        true,
        1,
        2,
        false,
    }

    maker_contract2_builder := sdk.ContractBuilder {
        sdk.AccountId(5),
        sdk.SubAccountId(1),
        sdk.SlotId(2),
        sdk.Nonce(100),
        sdk.PairId(1),
        *big.NewInt(45656),
        *big.NewInt(343),
        true,
        8,
        20,
        true,
    }

    unsigned_maker_contract1 := sdk.NewContract(maker_contract1_builder)
    unsigned_maker_contract2 := sdk.NewContract(maker_contract2_builder)
    maker_contract1, err := unsigned_maker_contract1.CreateSignedContract(
        zklinkSigner,
    )
    maker_contract2, err := unsigned_maker_contract2.CreateSignedContract(
        zklinkSigner,
    )

    var makers []*sdk.Contract
    makers = make([]*sdk.Contract,2)
    makers[0] = maker_contract1
    makers[1] = maker_contract2

    contract_price1 := sdk.ContractPrice{
        sdk.PairId(1),
        *big.NewInt(656566),
    }

    contract_price2 := sdk.ContractPrice{
        sdk.PairId(3),
        *big.NewInt(52552131),
    }
    var contract_prices = make([]sdk.ContractPrice,2)
    contract_prices[0] = contract_price1
    contract_prices[1] = contract_price2

    margin_price1 := sdk.SpotPriceInfo {
       sdk.TokenId(17),
       *big.NewInt(3236653653635635),
    }
    margin_price2 := sdk.SpotPriceInfo {
      sdk.TokenId(18),
      *big.NewInt(549574875297),
    }
    var margin_prices = make([]sdk.SpotPriceInfo,2)
    margin_prices[0] = margin_price1
    margin_prices[1] = margin_price2

    builder := sdk.ContractMatchingBuilder {
        sdk.AccountId(10),
        sdk.SubAccountId(1),
        taker_contract,
        makers,
       *big.NewInt(1),
        sdk.TokenId(17),
        contract_prices,
        margin_prices,
    }

    tx := sdk.NewContractMatching(builder)
    signer, err := sdk.NewSigner(privateKey, sdk.L1TypeEth)
    if err != nil {
        return
    }
    txSignature, err := signer.SignContractMatching(tx)
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
	fmt.Println("ContractMatching rpc request:",  string(JsonTx))
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
    HighLevelContractMatching()
}
