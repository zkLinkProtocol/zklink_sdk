package binding_tests

import (
	"github.com/zkLinkProtocol/zklink_sdk/binding_tests/generated/uniffi/types"
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestTypeZkLinkAddress(t *testing.T) {
    // test zero address
    slice := make([]byte, 20)
    address, err := types.ZkLinkAddressFromSlice(slice);
    assert.Nil(t, err)
    assert.Equal(t, address.IsZero(), true)
    assert.Equal(t, address.ToString(), "0x0000000000000000000000000000000000000000")
    // test global account address
    s := "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
    address, err = types.ZkLinkAddressFromStr(s)
    assert.Nil(t, err)
    assert.Equal(t, address.IsGlobalAccountAddress(), true)
    // invlaid address
    slice = []byte{0,1,2,3,4,5}
    address, err = types.ZkLinkAddressFromSlice(slice)
    assert.NotNil(t, err)
}

func TestTypeTxHash(t *testing.T) {
    slice := make([]byte, 32)
    hash, err := types.TxHashFromSlice(slice);
    assert.Nil(t, err)
    hash_str := hash.ToString()
    hash2, err := types.TxHashFromStr(hash_str)
    assert.Equal(t, hash_str, hash2.ToString())
}
