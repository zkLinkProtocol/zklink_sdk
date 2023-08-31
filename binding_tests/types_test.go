package binding_tests

import (
	"github.com/zkLinkProtocol/zklink_sdk/binding_tests/generated/uniffi/zklink_sdk"
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestTypeDeposit(t *testing.T) {
    from_address := "0x0000000000000000000000000000000000000000";
    to_address := "0x0000000000000000000000000000000000000001";
    from_chain_id := zklink_sdk.ChainId(1)
    sub_account_id := zklink_sdk.SubAccountId(2)
    l2_target_token := zklink_sdk.TokenId(1)
    l1_source_token := zklink_sdk.TokenId(2)
    amount := zklink_sdk.BigUint("123")
    serial_id := uint64(123)
    eth_hash := "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"

    deposit := zklink_sdk.NewDeposit(from_chain_id, from_address, sub_account_id, to_address, l2_target_token, l1_source_token, amount, serial_id, eth_hash);
    bytes := deposit.GetBytes()
    assert.NotNil(t, bytes)
}
