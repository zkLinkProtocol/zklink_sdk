package binding_tests

import (
	sdk "github.com/zkLinkProtocol/zklink_sdk/binding_tests/generated/uniffi/zklink_sdk"
	"github.com/stretchr/testify/assert"
	"testing"
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
        sdk.BigUint("1"),
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
