use crate::web_socket::ws_message::topic::Topic;
use serde::{Deserialize, Serialize};
use zklink_sdk_types::basic_types::SubAccountId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryEvent {
    pub topic: Topic,
    pub offset: ClientOffset,
}

impl QueryEvent {
    pub fn query_tx_created(sub_account_id: u8, offset: ClientOffset) -> Self {
        Self {
            topic: Topic::PriorityEvent {
                sub_account_id: SubAccountId(sub_account_id),
            },
            offset,
        }
    }
    pub fn query_tx_result(sub_account_id: u8, offset: ClientOffset) -> Self {
        Self {
            topic: Topic::TxExecuteResult {
                sub_account_id: SubAccountId(sub_account_id),
            },
            offset,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientOffset {
    /// give the msg from topic index, the result will be included
    pub from_topic_index_included: i64,
    /// limit event result, max and default to 100, if not given
    pub limit: Option<usize>,
}
