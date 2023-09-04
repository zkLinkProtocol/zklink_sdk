package binding_tests

import (
	sdk "github.com/zkLinkProtocol/zklink_sdk/binding_tests/generated/uniffi/zklink_sdk"
	"github.com/stretchr/testify/assert"
	"math/big"
	"testing"
)

func TestTypeDeposit(t *testing.T) {
    from_address := "0x0000000000000000000000000000000000000000";
    to_address := "0x0000000000000000000000000000000000000001";
    from_chain_id := sdk.ChainId(1)
    sub_account_id := sdk.SubAccountId(2)
    l2_target_token := sdk.TokenId(1)
    l1_source_token := sdk.TokenId(2)
    amount := *big.NewInt(123)
    serial_id := uint64(123)
    eth_hash := "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"

    deposit := sdk.NewDeposit(from_chain_id, from_address, sub_account_id, to_address, l2_target_token, l1_source_token, amount, serial_id, eth_hash);
    bytes := deposit.GetBytes()
    assert.NotNil(t, bytes)
}

func TestTypeTransfer(t *testing.T) {
    address_to := sdk.ZkLinkAddress("0xAFAFf3aD1a0425D792432D9eCD1c3e26Ef2C42E9")
    ts := sdk.TimeStamp(1693472232)
    account_id := sdk.AccountId(10)
    from_sub_account_id := sdk.SubAccountId(1)
    to_sub_account_id := sdk.SubAccountId(1)
    token := sdk.TokenId(18)
    amount := *big.NewInt(10000)
    fee := *big.NewInt(3)
    nonce := sdk.Nonce(1)
    tx := sdk.NewTransfer(
        account_id,
        address_to,
        from_sub_account_id,
        to_sub_account_id,
        token,
        amount,
        fee,
        nonce,
        ts,
    )
    bytes := tx.GetBytes()
    bytes_expected := []byte {
        4, 0, 0, 0, 10, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 175, 175, 243, 173, 26, 4, 37,
        215, 146, 67, 45, 158, 205, 28, 62, 38, 239, 44, 66, 233, 1, 0, 18, 0, 0, 4, 226, 0, 0,
        96, 0, 0, 0, 1, 100, 240, 85, 232,
    }
    assert.Equal(t, bytes, bytes_expected)
}
