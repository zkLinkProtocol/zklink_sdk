pub mod message;

use crate::starknet_signer::error::StarkSignerError;
use crate::starknet_signer::typed_data::message::TypedDataMessage;
use num::{BigUint, Num};
use serde::{Deserialize, Serialize};

use starknet_core::crypto::compute_hash_on_elements;
use starknet_core::types::FieldElement;
use starknet_core::utils::starknet_keccak;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StarknetDomain {
    pub name: String,
    pub version: String,
    pub chain_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TypeDefine {
    pub name: String,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DataType {
    pub stark_net_domain: Vec<TypeDefine>,
    pub message: Vec<TypeDefine>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypedData {
    pub types: DataType,
    pub primary_type: String,
    pub domain: StarknetDomain,
    pub message: TypedDataMessage,
}

impl TypedData {
    pub fn new(message: TypedDataMessage, chain_id: String) -> Self {
        let starknet_domain_type = vec![
            TypeDefine {
                name: "name".to_string(),
                r#type: "string".to_string(),
            },
            TypeDefine {
                name: "version".to_string(),
                r#type: "string".to_string(),
            },
            TypeDefine {
                name: "chainId".to_string(),
                r#type: "string".to_string(),
            },
        ];
        let message_type = Self::get_message_type(&message);
        let types = DataType {
            stark_net_domain: starknet_domain_type,
            message: message_type,
        };
        let domain = StarknetDomain {
            name: "zklink".to_string(),
            version: "1".to_string(),
            chain_id,
        };
        Self {
            types,
            primary_type: "Message".to_string(),
            domain,
            message,
        }
    }

    pub fn get_message_type(data_type: &TypedDataMessage) -> Vec<TypeDefine> {
        match data_type {
            TypedDataMessage::CreateL2Key { .. } => {
                vec![TypeDefine {
                    name: "data".to_string(),
                    r#type: "string".to_string(),
                }]
            }
            TypedDataMessage::Transaction { .. } => {
                vec![
                    TypeDefine {
                        name: "transaction".to_string(),
                        r#type: "string".to_string(),
                    },
                    TypeDefine {
                        name: "amount".to_string(),
                        r#type: "string".to_string(),
                    },
                    TypeDefine {
                        name: "fee".to_string(),
                        r#type: "string".to_string(),
                    },
                    TypeDefine {
                        name: "token".to_string(),
                        r#type: "string".to_string(),
                    },
                    TypeDefine {
                        name: "to".to_string(),
                        r#type: "string".to_string(),
                    },
                    TypeDefine {
                        name: "nonce".to_string(),
                        r#type: "string".to_string(),
                    },
                ]
            }
        }
    }

    fn string_to_hex(s: &str) -> String {
        if let Ok(num) = BigUint::from_str_radix(s.trim_start_matches("0x"), 16) {
            format!("0x{}", num.to_str_radix(16))
        } else {
            format!("0x{}", hex::encode(s.replace('\n', "")))
        }
    }

    pub fn get_type_define(&self, struct_type: &str) -> Vec<TypeDefine> {
        if struct_type == "StarkNetDomain" {
            self.types.stark_net_domain.clone()
        } else if struct_type == "Message" {
            self.types.message.clone()
        } else {
            vec![]
        }
    }

    pub fn encode_type(&self, struct_type: &str) -> String {
        let td = self.get_type_define(struct_type);
        let mut ret = struct_type.to_string() + "(";
        let mut fields = vec![];
        for t in td {
            let field = format!("{}:{}", t.name, t.r#type);
            fields.push(field);
        }
        ret += &fields.join(",");
        ret += ")";
        ret
    }

    pub fn get_struct_hash<M: Serialize>(&self, struct_type: &str, data: &M) -> Result<FieldElement, StarkSignerError> {
        let mut types_array = vec![];
        let mut data_array = vec![];
        types_array.push("felt".to_string());
        let type_hash = starknet_keccak(self.encode_type(struct_type).as_bytes());
        data_array.push(type_hash);
        let data_value = serde_json::to_value(data).map_err(|e| StarkSignerError::SignError(e.to_string()))?;
        let data_map = data_value.as_object().unwrap();
        //type must be exist
        let td = self.get_type_define(struct_type);
        if td.is_empty() {
            return Err(StarkSignerError::SignError("Invalid type name".to_string()));
        }

        for t in td {
            types_array.push(t.r#type.clone());
            let v_str = data_map.get(&t.name).unwrap().as_str().unwrap();
            let v = Self::string_to_hex(v_str);
            let v = FieldElement::from_hex_be(&v).map_err(|e| StarkSignerError::SignError(e.to_string()))?;
            data_array.push(v);
        }

        Ok(compute_hash_on_elements(&data_array))
    }

    pub fn encode(&self, addr: FieldElement) -> Result<Vec<FieldElement>, StarkSignerError> {
        let domain = self
            .get_struct_hash("StarkNetDomain", &self.domain)
            .map_err(|e| StarkSignerError::SignError(e.to_string()))?;
        let message = self
            .get_struct_hash("Message", &self.message)
            .map_err(|e| StarkSignerError::SignError(e.to_string()))?;
        //StarkNet Message
        let stark_net_message = FieldElement::from_str(&Self::string_to_hex("StarkNet Message")).unwrap();
        Ok(vec![stark_net_message, domain, addr, message])
    }

    pub fn get_message_hash(&self, addr: FieldElement) -> Result<FieldElement, StarkSignerError> {
        let data = self.encode(addr)?;
        Ok(compute_hash_on_elements(&data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::starknet_signer::typed_data::message::TxMessage;

    #[test]
    fn test_struct_hash() {
        let transfer = TxMessage {
            amount: "0.0012345678998".to_string(),
            fee: "0.00000001".to_string(),
            nonce: "1".to_string(),
            to: "0x0322546b712D87B8565C33530A6396D85f024F2C99ff564019a5Fc4c38e0F740".to_string(),
            token: "USDC".to_string(),
            transaction: "Transfer".to_string(),
        };

        let message = transfer.clone();
        let starknet_chain_id = "SN_GOERLI".to_string();
        let typed_data = TypedData::new(
            TypedDataMessage::Transaction {
                message: message.clone(),
            },
            starknet_chain_id.clone(),
        );
        let domain = StarknetDomain {
            name: "zklink".to_string(),
            version: "1".to_string(),
            chain_id: starknet_chain_id,
        };
        let domain_hash = typed_data.get_struct_hash("StarkNetDomain", &domain).unwrap();
        let message_hash = typed_data.get_struct_hash("Message", &message).unwrap();
        assert_eq!(
            hex::encode(domain_hash.to_bytes_be()),
            "0676111d4742900503a856514c9ae6d075566fe29007e31626f1f0a5ab4fd1fe"
        );
        assert_eq!(
            hex::encode(message_hash.to_bytes_be()),
            "03277b1075fc75aa5224b51140c9576733fb1be95c200fcb7ec264ad7b3fb010"
        );
    }
}
