pub mod message;

use serde::{Deserialize, Serialize};
use crate::starknet_signer::typed_data::message::TypedDataMessage;

#[derive(Serialize, Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct StarknetDomain {
    pub name: String,
    pub version: String,
    pub chain_id: String
}

#[derive(Serialize, Deserialize,Debug)]
pub struct TypeDefine {
    pub name: String,
    pub r#type: String,
}

#[derive(Serialize, Deserialize,Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DataType {
    pub stark_net_domain: Vec<TypeDefine>,
    pub message: Vec<TypeDefine>
}

#[derive(Serialize, Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypedData {
    pub types: DataType,
    pub primary_type: String,
    pub domain: StarknetDomain,
    pub message: TypedDataMessage,
}

impl TypedData {
    pub fn new(message: TypedDataMessage) -> Self {
        let starknet_domain_type = vec![
            TypeDefine { name: "name".to_string(), r#type: "string".to_string() },
            TypeDefine { name: "version".to_string(), r#type: "string".to_string() },
            TypeDefine { name: "chainId".to_string(), r#type: "string".to_string() },
        ];
        let message_type = Self::get_message_type(&message);
        let types = DataType {
            stark_net_domain: starknet_domain_type,
            message: message_type
        };
        let domain = StarknetDomain {
            name: "zklink".to_string(),
            version: "1".to_string(),
            chain_id: "SN_GOERLI".to_string()
        };
        Self {
            types,
            primary_type: "Message".to_string(),
            domain,
            message
        }
    }

    pub fn get_message_type(data_type: &TypedDataMessage) -> Vec<TypeDefine> {
        match data_type {
            TypedDataMessage::CreateL2Key(_) => {
                vec![
                    TypeDefine { name: "data".to_string(), r#type: "string".to_string() },
                ]
            },
            TypedDataMessage::Transaction(_) => {
                vec![
                    TypeDefine { name: "transaction".to_string(), r#type: "string".to_string() },
                    TypeDefine { name: "amount".to_string(), r#type: "string".to_string() },
                    TypeDefine { name: "fee".to_string(), r#type: "string".to_string() },
                    TypeDefine { name: "token".to_string(), r#type: "string".to_string() },
                    TypeDefine { name: "to".to_string(), r#type: "string".to_string() },
                    TypeDefine { name: "nonce".to_string(), r#type: "felt".to_string() },
                ]
            },
        }
    }
}