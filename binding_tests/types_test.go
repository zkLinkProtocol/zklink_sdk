package binding_tests

import (
	"github.com/zkLinkProtocol/zklink_sdk/binding_tests/generated/uniffi/zklink_sdk"
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestTypeZkLinkAddress(t *testing.T) {
    // test zero address
    slice := make([]byte, 20)
    address, err := zklink_sdk.ZkLinkAddressFromSlice(slice);
    assert.Nil(t, err)
    assert.Equal(t, address.IsZero(), true)
    assert.Equal(t, address.ToString(), "0x0000000000000000000000000000000000000000")
    // test global account address
    s := "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
    address, err = zklink_sdk.ZkLinkAddressFromStr(s)
    assert.Nil(t, err)
    assert.Equal(t, address.IsGlobalAccountAddress(), true)
    // invlaid address
    slice = []byte{0,1,2,3,4,5}
    address, err = zklink_sdk.ZkLinkAddressFromSlice(slice)
    assert.NotNil(t, err)
}

func TestTypeTxHash(t *testing.T) {
    slice := make([]byte, 32)
    hash, err := zklink_sdk.TxHashFromSlice(slice);
    assert.Nil(t, err)
    hash_str := hash.ToString()
    hash2, err := zklink_sdk.TxHashFromStr(hash_str)
    assert.Equal(t, hash_str, hash2.ToString())
}
