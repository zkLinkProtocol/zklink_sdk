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
    eth_signer, err := sdk.NewEthSigner(s)
    assert.Nil(t, err)
	zklink_signer, err := sdk.ZkLinkSignerNewFromHexEthSigner(s)
    assert.Nil(t, err)
    pubkey_hash := sdk.PubKeyHash("0xd8d5fb6a6caef06aa3dc2abdcdc240987e5330fe")
    builder := sdk.ChangePubKeyBuilder {
        sdk.ChainId(1),
        sdk.AccountId(1),
        sdk.SubAccountId(1),
        pubkey_hash,
        sdk.TokenId(1),
        *big.NewInt(1),
        sdk.Nonce(1),
        &packed_eth_signature,
        sdk.TimeStamp(1),
    };
    tx := sdk.NewChangePubKey(builder)

    // create auth data
    eth_signature, err := sdk.EthSignatureOfChangePubkey(tx, eth_signer);
    assert.Nil(t, err)
    eth_auth_data := sdk.ChangePubKeyAuthDataEthEcdsa {
        EthSignature: eth_signature,
    }

    // sign tx
    tx, err = sdk.CreateSignedChangePubkey(zklink_signer, tx, eth_auth_data)
    assert.Nil(t, err)
    valid := tx.IsSignatureValid();
    assert.Equal(t, valid, true)
    fmt.Printf("%v\n", tx.JsonStr())
    // create ZkLinkTx
    zkinkTx := tx.ToZklinkTx();
    fmt.Printf("%s\n", zkinkTx)

    // submitter signature
    txHash := tx.TxHash()
    submitterSignature, err := zklink_signer.SignMusig(txHash)
    assert.Nil(t, err)
    fmt.Printf("submitter signature: %v\n", submitterSignature)
}


func TestSignForcedExit(t *testing.T) {
	s := "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"
	zklink_signer, err := sdk.ZkLinkSignerNewFromHexEthSigner(s)
    assert.Nil(t, err)
    address := sdk.ZkLinkAddress("0xAFAFf3aD1a0425D792432D9eCD1c3e26Ef2C42E9")
    builder := sdk.ForcedExitBuilder{
        sdk.ChainId(1),
        sdk.AccountId(1),
        sdk.SubAccountId(1),
        address,
        sdk.SubAccountId(1),
        sdk.TokenId(18),
        sdk.TokenId(18),
        sdk.Nonce(1),
        *big.NewInt(100000),
        true,
        sdk.TimeStamp(1693472232),
    }
    tx := sdk.NewForcedExit(builder)
    signed_tx, err := tx.CreateSignedTx(zklink_signer)
    assert.Nil(t, err)
    should_be_valid := signed_tx.IsSignatureValid();
    assert.Equal(t, should_be_valid, true)
    fmt.Printf("signed forced exit:%v\n", signed_tx.JsonStr())
    zklink_tx := tx.ToZklinkTx()
    fmt.Printf("forced exit tx: %s", zklink_tx)
}

func TestSignTransfer(t *testing.T) {
    packed_eth_signature := sdk.PackedEthSignature("0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001b")
    assert.NotNil(t, packed_eth_signature)

	s := "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"
    eth_signer, err := sdk.NewEthSigner(s)
    assert.Nil(t, err)
	zklink_signer, err := sdk.ZkLinkSignerNewFromHexEthSigner(s)
    assert.Nil(t, err)
    address := sdk.ZkLinkAddress("0xAFAFf3aD1a0425D792432D9eCD1c3e26Ef2C42E9")
    builder := sdk.TransferBuilder{
        sdk.AccountId(1),
        address,
        sdk.SubAccountId(1),
        sdk.SubAccountId(1),
        sdk.TokenId(18),
        *big.NewInt(100000),
        *big.NewInt(100),
        sdk.Nonce(1),
        sdk.TimeStamp(1693472232),
    }
    tx := sdk.NewTransfer(builder)
    signed_tx, err := tx.CreateSignedTx(zklink_signer)
    assert.Nil(t, err)
    should_be_valid := signed_tx.IsSignatureValid();
    assert.Equal(t, should_be_valid, true)
    fmt.Printf("%v\n", signed_tx.JsonStr())
    // get eth signature
    eth_signature, err := signed_tx.EthSignature(eth_signer, "USDT")
    assert.Nil(t, err)
    fmt.Printf("eth signature: %v\n", eth_signature)
    // get ZklinkTx
    zklinkTx := tx.ToZklinkTx()
    fmt.Printf("zklink Tx: %s\n", zklinkTx)
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
        false,
        2,
        5,
        nil,
    )
    taker, err = taker.CreateSignedOrder(zklink_signer)
    assert.Nil(t, err)
    assert.NotNil(t, taker.GetSignature())
    fmt.Printf("taker signature:%v\n", taker.GetSignature())

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
         false,
         2,
         5,
         nil,
    )
    maker, err = maker.CreateSignedOrder(zklink_signer)
    assert.Nil(t, err)
    assert.NotNil(t, maker.GetSignature())
    fmt.Printf("maker signature:%v\n", maker.GetSignature())

    builder := sdk.OrderMatchingBuilder{
        sdk.AccountId(3),
        sdk.SubAccountId(1),
        taker,
        maker,
        *big.NewInt(1000),
        sdk.TokenId(18),
        []sdk.ContractPrice {
            sdk.ContractPrice {
                PairId: sdk.PairId(1),
                MarketPrice: *big.NewInt(100000),
            },
            sdk.ContractPrice {
                PairId: sdk.PairId(2),
                MarketPrice: *big.NewInt(100000),
            },
        },
        []sdk.SpotPriceInfo {
            sdk.SpotPriceInfo {
                TokenId: sdk.TokenId(1),
                Price: *big.NewInt(100000),
            },
            sdk.SpotPriceInfo {
                TokenId: sdk.TokenId(2),
                Price: *big.NewInt(100000),
            },
        },
        *big.NewInt(808077878),
        *big.NewInt(5479779),
    }
    tx := sdk.NewOrderMatching(builder)
    signed_tx, err := tx.CreateSignedTx(zklink_signer)
    assert.Nil(t, err)
    should_be_valid := signed_tx.IsSignatureValid();
    assert.Equal(t, should_be_valid, true)
    fmt.Printf("order matching: %v\n", signed_tx.JsonStr())
    zklinkTx := tx.ToZklinkTx()
    fmt.Printf("zklink tx: %s\n", zklinkTx)
}

func TestDeposit(t *testing.T) {
    fromAddress := sdk.ZkLinkAddress("0xAFAFf3aD1a0425D792432D9eCD1c3e26Ef2C42E9")
    toAddress := sdk.ZkLinkAddress("0xAFAFf3aD1a0425D792432D9eCD1c3e26Ef2C42E9")
    l2Hash := "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
    builder := sdk.DepositBuilder {
      fromAddress,
      toAddress,
      sdk.ChainId(1),
      sdk.SubAccountId(2),
      sdk.TokenId(3),
      sdk.TokenId(4),
      *big.NewInt(100),
      100,
      l2Hash,
      nil,
    }
    tx := sdk.NewDeposit(builder)
    assert.NotNil(t, tx)
}

func TestSignWithdraw(t *testing.T) {
    packed_eth_signature := sdk.PackedEthSignature("0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001b")
    assert.NotNil(t, packed_eth_signature)
	s := "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"
    eth_signer, err := sdk.NewEthSigner(s)
    assert.Nil(t, err)
	zklink_signer, err := sdk.ZkLinkSignerNewFromHexEthSigner(s)
    assert.Nil(t, err)
    address := sdk.ZkLinkAddress("0xAFAFf3aD1a0425D792432D9eCD1c3e26Ef2C42E9")
    builder := sdk.WithdrawBuilder{
        sdk.AccountId(1),
        sdk.SubAccountId(1),
        sdk.ChainId(1),
        address,
        sdk.TokenId(18),
        sdk.TokenId(18),
        *big.NewInt(100000),
        *big.NewInt(100),
        sdk.Nonce(1),
        50,
        true,
        sdk.TimeStamp(1693472232),
    }
    tx := sdk.NewWithdraw(builder)
    signedTx, err := tx.CreateSignedTx(zklink_signer)
    assert.Nil(t, err)
    should_be_valid := signedTx.IsSignatureValid();
    assert.Equal(t, should_be_valid, true)
    fmt.Printf("signed withraw tx: %v\n", signedTx.JsonStr())

    // eth signature
    l2SourceTokenSymbol := "USDC"
    ethSignature, err := signedTx.EthSignature(eth_signer, l2SourceTokenSymbol)
    assert.Nil(t, err)
    fmt.Printf("eth signature: %v\n", ethSignature)

    // create zklink tx
    zklinkTx := tx.ToZklinkTx()
    fmt.Printf("zklink tx: %s\n", zklinkTx)

    // test signer
    signer, err := sdk.NewSigner(s, sdk.L1TypeEth);
    assert.Nil(t, err)
    tx_signature, err := signer.SignWithdraw(tx, l2SourceTokenSymbol)
    assert.Nil(t, err)

    // test submitter
    submitterSignature, err := tx.SubmitterSignature(zklink_signer)
    submitterSignature2, err := signer.SubmitterSignature(tx_signature.Tx)
    assert.Nil(t, err)
    fmt.Printf("submitter signature: %s\n", submitterSignature)
    fmt.Printf("submitter signature: %s\n", submitterSignature2)
    assert.Equal(t, submitterSignature, submitterSignature2)
}


func TestAutoDeleveraging(t *testing.T) {
	s := "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"
	zklink_signer, err := sdk.ZkLinkSignerNewFromHexEthSigner(s)
    assert.Nil(t, err)
    builder := sdk.AutoDeleveragingBuilder{
        AccountId: sdk.AccountId(1),
        SubAccountId: sdk.SubAccountId(1),
        SubAccountNonce: sdk.Nonce(1),
        ContractPrices: []sdk.ContractPrice {
            sdk.ContractPrice {
                PairId: sdk.PairId(1),
                MarketPrice: *big.NewInt(100000),
            },
            sdk.ContractPrice {
                PairId: sdk.PairId(2),
                MarketPrice: *big.NewInt(100000),
            },
        },
        MarginPrices: []sdk.SpotPriceInfo {
            sdk.SpotPriceInfo {
                TokenId: sdk.TokenId(1),
                Price: *big.NewInt(100000),
            },
            sdk.SpotPriceInfo {
                TokenId: sdk.TokenId(2),
                Price: *big.NewInt(100000),
            },
        },
        AdlAccountId: sdk.AccountId(18),
        PairId: sdk.PairId(18),
        AdlSize: *big.NewInt(100000),
        AdlPrice: *big.NewInt(100),
        Fee: *big.NewInt(100),
        FeeToken: sdk.TokenId(1),
    }
    tx := sdk.NewAutoDeleveraging(builder)
    signedTx, err := tx.CreateSignedTx(zklink_signer)
    assert.Nil(t, err)
    should_be_valid := signedTx.IsSignatureValid();
    assert.Equal(t, should_be_valid, true)
    fmt.Printf("signed auto deleveraging tx: %v\n", signedTx.JsonStr())
}

func TestContractMatching(t *testing.T) {
	s := "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"
	zklink_signer, err := sdk.ZkLinkSignerNewFromHexEthSigner(s)
    assert.Nil(t, err)
    contract_builder := sdk.ContractBuilder {
        AccountId: sdk.AccountId(1),
        SubAccountId: sdk.SubAccountId(1),
        SlotId: sdk.SlotId(1),
        Nonce: sdk.Nonce(10),
        PairId: sdk.PairId(1),
        Size: *big.NewInt(100),
        Price: *big.NewInt(100),
        Direction: false,
        MakerFeeRate: 10,
        TakerFeeRate: 20,
        HasSubsidy: false,
    }
    taker := sdk.NewContract(contract_builder)
    maker1 := sdk.NewContract(contract_builder)
    maker2 := sdk.NewContract(contract_builder)
    builder := sdk.ContractMatchingBuilder {
        sdk.AccountId(1),
        sdk.SubAccountId(1),
        taker,
        []*sdk.Contract {maker1, maker2},
        *big.NewInt(100),
        sdk.TokenId(1),
    }
    tx := sdk.NewContractMatching(builder)
    signedTx, err := tx.CreateSignedTx(zklink_signer)
    assert.Nil(t, err)
    should_be_valid := signedTx.IsSignatureValid();
    assert.Equal(t, should_be_valid, true)
    fmt.Printf("signed contract matching tx: %v\n", signedTx.JsonStr())
}

func TestFunding(t *testing.T) {
	s := "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"
	zklink_signer, err := sdk.ZkLinkSignerNewFromHexEthSigner(s)
    assert.Nil(t, err)
    builder := sdk.FundingBuilder {
        AccountId: sdk.AccountId(1),
        SubAccountId: sdk.SubAccountId(1),
        SubAccountNonce: sdk.Nonce(1),
        FundingAccountIds: []sdk.AccountId{sdk.AccountId(1), sdk.AccountId(2)},
        Fee: *big.NewInt(100),
        FeeToken: sdk.TokenId(2),
    }
    tx := sdk.NewFunding(builder)
    signedTx, err := tx.CreateSignedTx(zklink_signer)
    assert.Nil(t, err)
    should_be_valid := signedTx.IsSignatureValid();
    assert.Equal(t, should_be_valid, true)
    fmt.Printf("signed funding tx: %v\n", signedTx.JsonStr())
}

func TestLiquidation(t *testing.T) {
	s := "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"
	zklink_signer, err := sdk.ZkLinkSignerNewFromHexEthSigner(s)
    assert.Nil(t, err)
    builder := sdk.LiquidationBuilder {
        AccountId: sdk.AccountId(1),
        SubAccountId: sdk.SubAccountId(1),
        SubAccountNonce: sdk.Nonce(1),
        ContractPrices: []sdk.ContractPrice {
            sdk.ContractPrice {
                PairId: sdk.PairId(1),
                MarketPrice: *big.NewInt(100000),
            },
            sdk.ContractPrice {
                PairId: sdk.PairId(2),
                MarketPrice: *big.NewInt(100000),
            },
        },
        MarginPrices: []sdk.SpotPriceInfo {
            sdk.SpotPriceInfo {
                TokenId: sdk.TokenId(1),
                Price: *big.NewInt(100000),
            },
            sdk.SpotPriceInfo {
                TokenId: sdk.TokenId(2),
                Price: *big.NewInt(100000),
            },
        },
        LiquidationAccountId: sdk.AccountId(1),
        Fee: *big.NewInt(100),
        FeeToken: sdk.TokenId(2),
    }
    tx := sdk.NewLiquidation(builder)
    signedTx, err := tx.CreateSignedTx(zklink_signer)
    assert.Nil(t, err)
    should_be_valid := signedTx.IsSignatureValid();
    assert.Equal(t, should_be_valid, true)
    fmt.Printf("signed liquidation tx: %v\n", signedTx.JsonStr())
}
