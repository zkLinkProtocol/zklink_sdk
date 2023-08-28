package binding_tests

import (
	"testing"

	"github.com/zkLinkProtocol/zklink_sdk/binding_tests/generated/uniffi/zklink_crypto"
	"github.com/stretchr/testify/assert"
)

func TestCrypto(t *testing.T) {
        eth_private_key := "be725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4"
        zk_signer, err := ZkLinkSigner::ZkLinkSignerNewFromHexEthSigner(eth_private_key)
        assert.NoError(t, err)
        pub_key, err := zk_signer.GetPublicKey();
        assert.NoError(t, err)
}
