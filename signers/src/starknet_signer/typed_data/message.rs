use serde::{Deserialize, Serialize, Serializer};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TxMessage {
    pub transaction: String,
    pub amount: String,
    pub fee: String,
    pub token: String,
    pub to: String,
    pub nonce: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub data: String,
}

#[derive(Debug)]
pub enum TypedDataMessage {
    CreateL2Key { message: Message },
    Transaction { message: TxMessage },
}

impl Serialize for TypedDataMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            TypedDataMessage::CreateL2Key { message } => message.serialize(serializer),
            TypedDataMessage::Transaction { message } => message.serialize(serializer),
        }
    }
}
