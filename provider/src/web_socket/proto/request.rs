use crate::web_socket::proto::topic::TopicMethod;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use tokio_tungstenite::tungstenite::Message;

/// this request is used to subscribe/unsubscribe topics
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TopicsRequest<T> {
    pub method: TopicMethod,
    pub topics: Vec<T>,
    pub id: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClientMessage<M, T> {
    TopicsRequest(TopicsRequest<T>),
    Query(M),
}

impl<M, T> ClientMessage<M, T>
where
    T: Serialize,
    M: Serialize,
{
    pub fn to_message(&self) -> Result<Message, serde_json::Error> {
        let s = serde_json::to_string(self)?;
        let msg = Message::Text(s);
        Ok(msg)
    }
}
