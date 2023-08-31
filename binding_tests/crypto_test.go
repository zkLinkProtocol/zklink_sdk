/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package binding_tests

import (
	"github.com/zkLinkProtocol/zklink_sdk/binding_tests/generated/uniffi/zklink_sdk"
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestPrivateKey(t *testing.T) {
	priv_key, err := zklink_sdk.NewPackedPrivateKey()
	assert.Nil(t, err)
	assert.NotNil(t, priv_key)
	eth_key := "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4";
	priv_key, err = zklink_sdk.PackedPrivateKeyNewFromHexEthSigner(eth_key)
	assert.Nil(t, err)
	assert.NotNil(t, priv_key)
	pub_key := priv_key.PublicKey()
	assert.NotNil(t, pub_key)
	assert.Equal(t, pub_key, "0x7b173e25e484eed3461091430f81b2a5bd7ae792f69701dcb073cb903f812510")
	pubkey_hash := zklink_sdk.GetPublicKeyHash(pub_key)
	assert.Equal(t, pubkey_hash, "0xd8d5fb6a6caef06aa3dc2abdcdc240987e5330fe")
}

func TestZkLinkSigner(t *testing.T) {
    signer, err := zklink_sdk.NewZkLinkSigner()
	assert.Nil(t, err)
	assert.NotNil(t, signer)
	s := "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"
	signer, err = zklink_sdk.ZkLinkSignerNewFromHexEthSigner(s)
	pub_key := signer.PublicKey()
	assert.Equal(t, pub_key, "0x7b173e25e484eed3461091430f81b2a5bd7ae792f69701dcb073cb903f812510")
	pubkey_hash := zklink_sdk.GetPublicKeyHash(pub_key)
	assert.Equal(t, pubkey_hash, "0xd8d5fb6a6caef06aa3dc2abdcdc240987e5330fe")
	msg := []uint8{0,1,2,3,4,5,6}
	signature, err := signer.SignMusig(msg)
	assert.Nil(t, err)
	assert.NotNil(t, signature)
	is_ok, _ := zklink_sdk.VerifyMusig(signature, msg)
	assert.Equal(t, is_ok, true)
}
