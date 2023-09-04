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
	s = "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"
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
    main_contract := sdk.ZkLinkAddress("0x0000000000000000000000000000000000000000")
    account_address := sdk.ZkLinkAddress("0x0000000000000000000000000000000000000000")
    l1_client_id := uint32(1)
    auth_request := sdk.ChangePubKeyAuthRequestOnchain{}
    tx_signature, err := sdk.SignChangePubkey(
        eth_signer,
        zklink_signer,
        tx,
        main_contract,
        l1_client_id,
        account_address,
        auth_request,
    )
    assert.Nil(t, err)
    assert.NotNil(t, tx_signature)
    fmt.Printf("%v\n", tx_signature)
}

func TestSignChangePubkeyEcdsa(t *testing.T) {
    packed_eth_signature := sdk.PackedEthSignature("0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001b")
    assert.NotNil(t, packed_eth_signature)

	s := "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"
    eth_signer, err := sdk.NewPrivateKeySigner(s)
    assert.Nil(t, err)
	s = "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"
	zklink_signer, err := sdk.ZkLinkSignerNewFromHexEthSigner(s)
    assert.Nil(t, err)
    pubkey_hash := sdk.PubKeyHash("0xd8d5fb6a6caef06aa3dc2abdcdc240987e5330fe")
    tx := sdk.NewChangePubKey(
        sdk.ChainId(1),
        sdk.AccountId(1),
        sdk.SubAccountId(1),
        pubkey_hash,
        sdk.TokenId(18),
        *big.NewInt(1),
        sdk.Nonce(1),
        &packed_eth_signature,
        sdk.TimeStamp(1),
    )
    main_contract := sdk.ZkLinkAddress("0x959Ae135F0387C55DC8bbFe8a4Db753485067D59")
    account_address := sdk.ZkLinkAddress("0x526212fBD41080B455aE81014B5B6bF859c30094")
    l1_client_id := uint32(80001)
    auth_request := sdk.ChangePubKeyAuthRequestEthEcdsa{}
    tx_signature, err := sdk.SignChangePubkey(
        eth_signer,
        zklink_signer,
        tx,
        main_contract,
        l1_client_id,
        account_address,
        auth_request,
    )
    assert.Nil(t, err)
    assert.NotNil(t, tx_signature)
    fmt.Printf("%v\n", tx_signature)
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
    tx_signature, err := sdk.SignForcedExit(
        zklink_signer,
        tx,
    )
    assert.Nil(t, err)
    assert.NotNil(t, tx_signature)
    fmt.Printf("%v\n", tx_signature)
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
    tx_signature, err := sdk.SignTransfer(
        eth_signer,
        zklink_signer,
        tx,
        "USDC",
    )
    assert.Nil(t, err)
    assert.NotNil(t, tx_signature)
    fmt.Printf("%v\n", tx_signature)
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
    taker, err = sdk.SignedOrder(
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
    maker, err = sdk.SignedOrder(
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
    tx_signature, err := sdk.SignOrderMatching(
        zklink_signer,
        tx,
    )
    assert.Nil(t, err)
    assert.NotNil(t, tx_signature)
    fmt.Printf("%v\n", tx_signature)
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
    tx_signature, err := sdk.SignWithdraw(
        eth_signer,
        zklink_signer,
        tx,
        "USDC",
    )
    assert.Nil(t, err)
    assert.NotNil(t, tx_signature)
    fmt.Printf("%v\n", tx_signature)
}
