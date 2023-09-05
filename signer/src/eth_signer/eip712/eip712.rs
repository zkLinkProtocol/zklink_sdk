use crate::eth_signer::error::EthSignerError;
use ethers::utils::keccak256;
use ethers_primitives::{Address, H256, U256};
use serde::{Deserialize, Serialize};
pub use serde_eip712::*;
use std::collections::HashMap;

///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EIP712Domain {
    /// the user readable name of signing domain, i.e. the name of the DApp or the protocol.
    pub name: String,
    /// the current major version of the signing domain. Signatures from different versions are not compatible.
    pub version: String,
    /// the EIP-155 chain id. The user-agent should refuse signing if it does not match the currently active chain.
    pub chain_id: U256,
    /// the address of the contract that will verify the signature. The user-agent may do contract specific phishing prevention
    pub verifying_contract: Address,
}

impl EIP712Domain {
    pub fn new(
        name: String,
        version: String,
        layer_one_chain_id: u32,
        eth_contract_addr: String,
    ) -> Result<Self, EthSignerError> {
        Ok(EIP712Domain {
            name,
            version,
            chain_id: U256::from(layer_one_chain_id),
            verifying_contract: Address::try_from(eth_contract_addr.as_str())
                .map_err(|e| EthSignerError::Eip712Failed(e.to_string()))?,
        })
    }

    pub fn new_zklink_domain(
        layer_one_chain_id: u32,
        eth_contract_addr: String,
    ) -> Result<Self, EthSignerError> {
        EIP712Domain::new(
            "ZkLink".into(),
            "1".into(),
            layer_one_chain_id,
            eth_contract_addr,
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TypedData<M>
where
    M: Serialize,
{
    /// The custom types used by signing message.
    pub types: HashMap<String, TypeDefinition>,
    /// The type of the message.
    pub primary_type: String,
    /// Signing domain metadata. The signing domain is the intended context for the signature (e.g.
    /// the dapp, protocol, etc. that it's intended for). This data is used to construct the domain
    /// seperator of the message.
    pub domain: EIP712Domain,
    /// The message to be signed.
    pub message: M,
}

impl<M> TypedData<M>
where
    M: Serialize,
{
    /// Create eth_signTypedData payload.
    pub fn new(domain: EIP712Domain, value: M) -> Result<TypedData<M>, EthSignerError> {
        // Get primary type.

        let encode_type =
            eip712_encode_type(&value).map_err(|e| EthSignerError::Eip712Failed(e.to_string()))?;

        let pos = encode_type.find('(').unwrap();

        let primary_type = encode_type[..pos].to_string();

        let mut type_definitions = eip712_type_definitions(&domain)
            .map_err(|e| EthSignerError::Eip712Failed(e.to_string()))?;

        type_definitions.extend(
            eip712_type_definitions(&value)
                .map_err(|e| EthSignerError::Eip712Failed(e.to_string()))?,
        );

        Ok(TypedData {
            types: type_definitions,
            primary_type,
            domain,
            message: value,
        })
    }
    pub fn sign_hash(&self) -> Result<H256, EthSignerError> {
        Ok(keccak256(self.encode()?).into())
    }

    pub fn encode(&self) -> Result<[u8; 66], EthSignerError> {
        let domain = eip712_hash_struct("EIP712Domain", &self.types, &self.domain)
            .map_err(|e| EthSignerError::Eip712Failed(e.to_string()))?;

        let message = eip712_hash_struct(self.primary_type.as_str(), &self.types, &self.message)
            .map_err(|e| EthSignerError::Eip712Failed(e.to_string()))?;

        // "\x19\x01" ‖ domainSeparator ‖ hashStruct(message)
        let mut buff = [0u8; 66];

        buff[0..2].copy_from_slice(&[0x19, 0x01]);
        buff[2..34].copy_from_slice(&domain);
        buff[34..66].copy_from_slice(&message);

        Ok(buff)
    }
}
