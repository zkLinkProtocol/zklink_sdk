use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Debug)]
pub struct TxMessage {
    pub transaction: String,
    pub amount: String,
    pub fee: String,
    pub token: String,
    pub to: String,
    pub nonce: u64,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Message {
    pub data: String
}

#[derive(Serialize, Deserialize,Debug)]
#[serde(untagged)]
pub enum TypedDataMessage {
    CreateL2Key(Message),
    Transaction(TxMessage),
}