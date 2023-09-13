package binding_tests

import (
	sdk "github.com/zkLinkProtocol/zklink_sdk/binding_tests/generated/uniffi/zklink_sdk"
	"github.com/stretchr/testify/assert"
	"testing"
	"math/big"
	"fmt"
)

func TestSignChangePubkey(t *testing.T) {
    packed_eth_signature := sdk.PackedEthSignature("0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001b")
    assert.NotNil(t, packed_eth_signature)

	s := "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"
    eth_signer, err := sdk.NewPrivateKeySigner(s)
    assert.Nil(t, err)
	zklink_signer, err := sdk.ZkLinkSignerNewFromHexEthSigner(s)
    assert.Nil(t, err)
    pubkey_hash := sdk.PubKeyHash("0xd8d5fb6a6caef06aa3dc2abdcdc240987e5330fe")
    tx := sdk.NewChangePubKey(
        sdk.ChainId(1),
        sdk.AccountId(1),
        sdk.SubAccountId(1),
        pubkey_hash,
        sdk.TokenId(1),
        *big.NewInt(1),
        sdk.Nonce(1),
        &packed_eth_signature,
        sdk.TimeStamp(1),
    )

    // create auth data
    main_contract := sdk.ZkLinkAddress("0x0000000000000000000000000000000000000000")
    l1_client_id := uint32(1)
    eth_signature, err := sdk.EthSignatureOfChangePubkey(l1_client_id, tx, eth_signer, main_contract);
    assert.Nil(t, err)
    eth_auth_data := sdk.ChangePubKeyAuthDataEthEcdsa {
        EthSignature: eth_signature,
    }

    // sign tx
    tx, err = sdk.CreateSignedChangePubkey(zklink_signer, tx, eth_auth_data)
    assert.Nil(t, err)
    valid, err := tx.IsSignatureValid();
    assert.Equal(t, valid, true)
    fmt.Printf("%v\n", tx.JsonStr())

    // submitter signature
    txHash := tx.TxHash()
    submitter_signature, err := zklink_signer.SignMusig(txHash)
    assert.Nil(t, err)
    fmt.Printf("submitter signature: %v\n", sdk.JsonStrOfZklinkSignature(submitter_signature))
}


func TestSignForcedExit(t *testing.T) {
	s := "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"
	zklink_signer, err := sdk.ZkLinkSignerNewFromHexEthSigner(s)
    assert.Nil(t, err)
    address := sdk.ZkLinkAddress("0xAFAFf3aD1a0425D792432D9eCD1c3e26Ef2C42E9")
    tx := sdk.NewForcedExit(
        sdk.ChainId(1),
        sdk.AccountId(1),
        sdk.SubAccountId(1),
        address,
        sdk.SubAccountId(1),
        sdk.TokenId(18),
        sdk.TokenId(18),
        sdk.Nonce(1),
        *big.NewInt(100000),
        sdk.TimeStamp(1693472232),
    )
    signed_tx, err := sdk.CreateSignedForcedExit(
        zklink_signer,
        tx,
    )
    assert.Nil(t, err)
    should_be_valid, err := signed_tx.IsSignatureValid();
    assert.Nil(t, err)
    assert.Equal(t, should_be_valid, true)
    fmt.Printf("signed forced exit:%v\n", signed_tx.JsonStr())
}

func TestSignTransfer(t *testing.T) {
    packed_eth_signature := sdk.PackedEthSignature("0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001b")
    assert.NotNil(t, packed_eth_signature)

	s := "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"
    eth_signer, err := sdk.NewPrivateKeySigner(s)
    assert.Nil(t, err)
	zklink_signer, err := sdk.ZkLinkSignerNewFromHexEthSigner(s)
    assert.Nil(t, err)
    address := sdk.ZkLinkAddress("0xAFAFf3aD1a0425D792432D9eCD1c3e26Ef2C42E9")
    tx := sdk.NewTransfer(
        sdk.AccountId(1),
        address,
        sdk.SubAccountId(1),
        sdk.SubAccountId(1),
        sdk.TokenId(18),
        *big.NewInt(100000),
        *big.NewInt(100),
        sdk.Nonce(1),
        sdk.TimeStamp(1693472232),
    )
    signed_tx, err := sdk.CreateSignedTransfer(
        zklink_signer,
        tx,
    )
    assert.Nil(t, err)
    should_be_valid, err := signed_tx.IsSignatureValid();
    assert.Nil(t, err)
    assert.Equal(t, should_be_valid, true)
    fmt.Printf("%v\n", signed_tx.JsonStr())
    // get eth signature
    eth_signature, err := signed_tx.EthSignature(eth_signer, "USDT")
    assert.Nil(t, err)
    fmt.Printf("eth signature: %v\n", eth_signature)
}


func TestSignOrderMatching(t *testing.T) {
	s := "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"
	zklink_signer, err := sdk.ZkLinkSignerNewFromHexEthSigner(s)

    taker := sdk.NewOrder(
        sdk.AccountId(1),
        sdk.SubAccountId(1),
        sdk.SlotId(3),
        sdk.Nonce(1),
        sdk.TokenId(18),
        sdk.TokenId(145),
        *big.NewInt(323289),
        *big.NewInt(135),
        true,
        2,
        5,
    )
    taker, err = sdk.CreateSignedOrder(
        zklink_signer,
        taker,
    )
    assert.Nil(t, err)
    assert.NotNil(t, taker.Signature())
    fmt.Printf("taker signature:%v\n", taker.Signature())

    maker := sdk.NewOrder(
         sdk.AccountId(2),
         sdk.SubAccountId(1),
         sdk.SlotId(3),
         sdk.Nonce(1),
         sdk.TokenId(18),
         sdk.TokenId(145),
         *big.NewInt(323355),
         *big.NewInt(135),
         false,
         2,
         5,
    )
    maker, err = sdk.CreateSignedOrder(
        zklink_signer,
        maker,
    )
    assert.Nil(t, err)
    assert.NotNil(t, maker.Signature())
    fmt.Printf("maker signature:%v\n", maker.Signature())

    tx := sdk.NewOrderMatching(
        sdk.AccountId(3),
        sdk.SubAccountId(1),
        taker,
        maker,
        *big.NewInt(1000),
        sdk.TokenId(18),
        *big.NewInt(808077878),
        *big.NewInt(5479779),
    )
    signed_tx, err := sdk.CreateSignedOrderMatching(
        zklink_signer,
        tx,
    )
    assert.Nil(t, err)
    should_be_valid, err := signed_tx.IsSignatureValid();
    assert.Nil(t, err)
    assert.Equal(t, should_be_valid, true)
    fmt.Printf("%v\n", signed_tx.JsonStr())
}

func TestSignWithdraw(t *testing.T) {
    packed_eth_signature := sdk.PackedEthSignature("0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001b")
    assert.NotNil(t, packed_eth_signature)

	s := "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"
    eth_signer, err := sdk.NewPrivateKeySigner(s)
    assert.Nil(t, err)
	zklink_signer, err := sdk.ZkLinkSignerNewFromHexEthSigner(s)
    assert.Nil(t, err)
    address := sdk.ZkLinkAddress("0xAFAFf3aD1a0425D792432D9eCD1c3e26Ef2C42E9")
    tx := sdk.NewWithdraw(
        sdk.AccountId(1),
        sdk.SubAccountId(1),
        sdk.ChainId(1),
        address,
        sdk.TokenId(18),
        sdk.TokenId(18),
        *big.NewInt(100000),
        *big.NewInt(100),
        sdk.Nonce(1),
        false,
        50,
        sdk.TimeStamp(1693472232),
    )
    signed_tx, err := sdk.CreateSignedWithdraw(
        zklink_signer,
        tx,
    )
    assert.Nil(t, err)
    should_be_valid, err := signed_tx.IsSignatureValid();
    assert.Nil(t, err)
    assert.Equal(t, should_be_valid, true)
    fmt.Printf("%v\n", signed_tx.JsonStr())

    // eth signature
    eth_signature, err := signed_tx.EthSignature(eth_signer, "USDC")
    assert.Nil(t, err)
    fmt.Printf("%v\n", eth_signature)
}
