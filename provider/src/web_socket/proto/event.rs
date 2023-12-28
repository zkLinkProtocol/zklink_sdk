use crate::web_socket::proto::topic::TopicTrait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use warp::ws::Message;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Event<T, D> {
    /// topic
    pub topic: T,
    pub topic_index: i64,
    /// data
    pub data: D,
    /// timestamp of microseconds
    pub timestamp: i64,
}

impl<T, D> Event<T, D>
where
    T: TopicTrait,
    D: Serialize,
{
    pub fn new(topic: T, topic_index: i64, data: D, timestamp: Option<DateTime<Utc>>) -> Self {
        let timestamp = timestamp.unwrap_or_else(Utc::now);
        Self {
            topic,
            topic_index,
            data,
            timestamp: timestamp.timestamp_micros(),
        }
    }

    pub fn ws_message(&self) -> Result<Message, serde_json::Error> {
        let s = serde_json::to_string(self)?;
        Ok(Message::text(s))
    }
}
