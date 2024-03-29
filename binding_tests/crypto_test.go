/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package binding_tests

import (
	"github.com/stretchr/testify/assert"
	sdk "github.com/zkLinkProtocol/zklink_sdk/binding_tests/generated/uniffi/zklink_sdk"
	"testing"
)

func TestEthSigner(t *testing.T) {
	s := "0xb32593e347bf09436b058fbeabc17ebd2c7c1fa42e542f5f78fc3580faef83b7"
	signer, err := sdk.NewEthSigner(s)
	assert.Nil(t, err)
	msg := []byte("hello world")
	signature, err := signer.SignMessage(msg)
	assert.Nil(t, err)

	assert.Equal(t, signature, "0xa9aa0710adb18f84d4bed8057382fc433c3dcff1bddf3b2b1c2cb11386ef3be4172b5d0688143759d4e744acc434ae4f96575c7fa9096971fd02fb3d2aaa77121c")
	address := signer.GetAddress()
	assert.Equal(t, address, "0x9e372368c25056d44045e445d72d7b91ce3ee3b1")
}

func TestZkLinkSigner(t *testing.T) {
	signer, err := sdk.NewZkLinkSigner()
	assert.Nil(t, err)
	assert.NotNil(t, signer)
	s := "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"
	signer, err = sdk.ZkLinkSignerNewFromHexEthSigner(s)
	pub_key := signer.PublicKey()
	assert.Equal(t, pub_key, "0x7b173e25e484eed3461091430f81b2a5bd7ae792f69701dcb073cb903f812510")
	pubkey_hash := sdk.GetPublicKeyHash(pub_key)
	assert.Equal(t, pubkey_hash, "0xd8d5fb6a6caef06aa3dc2abdcdc240987e5330fe")
	msg := []uint8{0, 1, 2, 3, 4, 5, 6}
	signature, err := signer.SignMusig(msg)
	assert.Nil(t, err)
	assert.NotNil(t, signature)
	is_ok := sdk.VerifyMusig(signature, msg)
	assert.Equal(t, is_ok, true)
}
