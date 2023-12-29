use crate::response::TxResp;
use crate::web_socket::proto::event::Event;
use crate::web_socket::ws_message::message::response::ServerEvent::{
    FullExitEvent, TxExecuteResult,
};
use crate::web_socket::ws_message::topic::{Topic, TopicType};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use zklink_sdk_types::basic_types::tx_hash::TxHash;
use zklink_sdk_types::tx_type::zklink_tx::ZkLinkTx;

pub type TxTopicEvent = Event<Topic, ServerEvent>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerEvent {
    FullExitEvent(FullExitEventResp),
    TxExecuteResult(TxResp),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullExitEventResp {
    pub tx_hash: TxHash,
    pub tx: ZkLinkTx,
}

impl ServerEvent {
    pub fn from_topic_msg(topic_type: TopicType, value: Value) -> anyhow::Result<Self> {
        match topic_type {
            TopicType::FullExitEvent => Ok(FullExitEvent(serde_json::from_value(value)?)),
            TopicType::TxExecuteResult => Ok(TxExecuteResult(serde_json::from_value(value)?)),
        }
    }
}
