pub mod message;

use serde::{Deserialize, Serialize};
use crate::starknet_signer::typed_data::message::TypedDataMessage;
use crate::starknet_signer::error::StarkSignerError;
use num::{BigUint, Num};
use starknet_crypto::FieldElement;
use starknet::core::utils::starknet_keccak;

#[derive(Serialize, Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct StarknetDomain {
    pub name: String,
    pub version: String,
    pub chain_id: String
}

#[derive(Serialize, Deserialize,Debug,Clone)]
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
    pub fn new(message: TypedDataMessage,chain_id: String) -> Self {
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
            chain_id,
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
                    TypeDefine { name: "nonce".to_string(), r#type: "string".to_string() },
                ]
            },
        }
    }

    fn string_to_hex(s: &str) -> String {
        if let Ok(num) = BigUint::from_str_radix(s.trim_start_matches("0x"),16) {
            format!("0x{}",num.to_str_radix(16))
        } else {
            format!("0x{}",hex::encode(s))
        }
    }

    pub fn get_type_define(&self,struct_type: &str) -> Vec<TypeDefine> {
        if struct_type == "StarkNetDomain" {
            self.types.stark_net_domain.clone()
        } else if struct_type == "Message" {
            self.types.message.clone()
        } else {
            vec![]
        }
    }

    pub fn encode_type(&self,struct_type: &str) -> String {
        let td = self.get_type_define(struct_type);
        let mut ret = struct_type.to_string() + "(";
        let mut fields = vec![];
        for t in td {
            let field = format!("{}:{}",t.name,t.r#type);
            fields.push(field);
        }
        ret += &fields.join(",");
        ret += ")";
        ret
    }

    pub fn compute_hash_on_elements(data: Vec<String>) -> Result<FieldElement,StarkSignerError> {
        let mut result = FieldElement::from(0u32);
        for e in &data {
            let fe = FieldElement::from_hex_be(e)
                .map_err(|e| StarkSignerError::SignError(e.to_string()))?;
            result = starknet_crypto::pedersen_hash(&result, &fe);
        }

        let data_len = FieldElement::from(data.len());
        Ok(starknet_crypto::pedersen_hash(&result, &data_len))
    }

    pub fn get_struct_hash<T: Serialize>(&self,struct_type: &str,data: &T) -> Result<[u8;32],StarkSignerError> {
        let mut types_arry = vec![];
        let mut data_arry = vec![];
        types_arry.push("felt".to_string());
        let encoded_type = self.encode_type(struct_type);
        println!("{}",encoded_type);
        let type_hash = starknet_keccak(self.encode_type(struct_type).as_bytes());
        data_arry.push(format!("0x{}",hex::encode(type_hash.to_bytes_be())));
        let data_value = serde_json::to_value(data)
            .map_err(|e| StarkSignerError::SignError(e.to_string()))?;
        let data_map = data_value.as_object().unwrap();
        //type must be exist
        let td = self.get_type_define(struct_type);
        if td.is_empty() {
            return Err(StarkSignerError::SignError("Invalid type name".to_string()));
        }

        for t in td {
            types_arry.push(t.r#type.clone());
            let v_str = data_map.get(&t.name).unwrap().as_str().unwrap();
            let v = Self::string_to_hex(&v_str);
            data_arry.push(v);
        }

        println!("{:?}",data_arry);
        let result = Self::compute_hash_on_elements(data_arry)?;
        Ok(result.to_bytes_be())
    }

    pub fn encode(&self,addr: String) -> Result<Vec<String>, StarkSignerError> {
        let domain = self.get_struct_hash("StarkNetDomain", &self.domain)
            .map_err(|e| StarkSignerError::SignError(e.to_string()))?;
        let message = self.get_struct_hash("Message",&self.message)
            .map_err(|e| StarkSignerError::SignError(e.to_string()))?;
        //StarkNet Message
        let stark_net_message = Self::string_to_hex("StarkNet Message");
        Ok(vec![stark_net_message,format!("0x{}",hex::encode(&domain)),addr,format!("0x{}",hex::encode(&message))])
    }

    pub fn get_message_hash(&self,addr: String) -> Result<FieldElement,StarkSignerError> {
        let data = self.encode(addr)?;
        println!("{:?}",data);
        Ok(Self::compute_hash_on_elements(data)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::starknet_signer::typed_data::message::Message;
    use num::Num;

    #[test]
    fn test_typed_data() {
        let addr = "04a69b67bcabfa7d3ccb96e1d25c2e6fc93589fe24a6fd04566b8700ff97a71a";
        let message = Message {
            data: "Create zkLink L2".to_string()
        };

        let s = "0x5505a8cd4594dbf79d8c59c0df1414ab871ca896";
        BigUint::from_str_radix(s.trim_start_matches("0x"),16).unwrap();
        let typed_data = TypedData::new(TypedDataMessage::CreateL2Key(message),"SN_GOERLI".to_string());
        let data = typed_data.encode(
            "0x04a69b67bcabfa7d3ccb96e1d25c2e6fc93589fe24a6fd04566b8700ff97a71a".to_string()).unwrap();
        println!("{:?}",data);

    }
}