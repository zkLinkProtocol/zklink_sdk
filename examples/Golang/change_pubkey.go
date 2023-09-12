package go_example

import (
	sdk "github.com/zkLinkProtocol/zklink_sdk/go_example/generated/uniffi/zklink_sdk"
	"github.com/stretchr/testify/assert"
	"testing"
	"math/big"
	"fmt"
)

type ChangePubKeyReq struct {
	ChainId sdk.ChainId,
	AccountId sdk.AccountId,
	SubAccountId sdk.SubAccountId,
	NewPkHash sdk.PubKeyHash,
	FeeToken sdk.Fee,
	FeeToken sdk.TokenId,
	Fee big.Int,
	Nonce sdk.Nonce,
	EthSignature sdk.PackedEthSignature,
	Ts sdk.TimeStamp
}

func ChangePubKey(
	ethPrivateKey *String,
	 req *ChangePubKeyReq,
	 main_contract sdk.ZkLinkAddress,
	 
	 ) (nil , error){
	zkSigner, err := sdk.ZkLinkSignerNewFromHexEthSigner(ethPrivateKey)
	if err != nil {
		return (nil, err)
	}
	ethSigner, err := sdk.NewPrivateKeySigner(ethPrivateKey)
	if err != nil {
		return (nil, err)
	}
	changePubKey := sdk.NewChangePubKey(
		req.ChainId,
		req.AccountId,
		req.SubAccountId,
		req.NewPkHash,
		req.FeeToken,
		req.Fee,
		req.Nonce,
		req.EthSignature,
		req.Ts,
	)
	changePubKey = sign
	signature_should_valid, err := changePubKey.IsSignatureValid();
	if err != nil {
		return (nil, err)
	}
	
}
