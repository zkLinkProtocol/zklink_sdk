use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::hash::Hash;

pub trait TopicTrait:
    Debug + Clone + Sync + Send + Eq + Hash + PartialEq + 'static + Serialize + DeserializeOwned
{
    /// return true if self in one of the topics
    fn matched(&self, topics: &[Self]) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TopicMethod {
    Subscribe,
    UnSubscribe,
}
