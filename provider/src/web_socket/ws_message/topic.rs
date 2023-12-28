use crate::web_socket::proto::topic::TopicTrait;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;
use zklink_sdk_types::basic_types::SubAccountId;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum Topic {
    /// App subscribe this event and then call `confirmFullExit`
    FullExitEvent { sub_account_id: SubAccountId },
    /// Get all(L1 and L2) txs executed result that accepted by api
    TxExecuteResult { sub_account_id: SubAccountId },
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum TopicType {
    FullExitEvent,
    TxExecuteResult,
}

impl ToString for TopicType {
    fn to_string(&self) -> String {
        match self {
            Self::FullExitEvent => "FullExitEvent".to_string(),
            Self::TxExecuteResult => "TxExecuteResult".to_string(),
        }
    }
}

impl Topic {
    pub fn get_type(&self) -> TopicType {
        match self {
            Topic::FullExitEvent { .. } => TopicType::FullExitEvent,
            Topic::TxExecuteResult { .. } => TopicType::TxExecuteResult,
        }
    }
}

impl Serialize for Topic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = self.to_string();
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for Topic {
    fn deserialize<D>(deserializer: D) -> Result<Topic, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StrVisitor;
        impl<'de> Visitor<'de> for StrVisitor {
            type Value = Topic;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string expected")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let topic = Topic::from_str(value).map_err(|e| E::custom(e.to_string()))?;
                Ok(topic)
            }
        }
        deserializer.deserialize_str(StrVisitor)
    }
}

impl TopicTrait for Topic {
    fn matched(&self, topics: &[Self]) -> bool {
        topics.contains(self)
    }
}

impl ToString for Topic {
    fn to_string(&self) -> String {
        match self {
            Self::FullExitEvent { sub_account_id } => format!("fullExitEvent@{sub_account_id}"),
            Self::TxExecuteResult { sub_account_id } => format!("txExecuteResult@{sub_account_id}"),
        }
    }
}

impl FromStr for Topic {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items: Vec<_> = s.split('@').collect();
        match items[0] {
            t @ ("fullExitEvent" | "txExecuteResult") => {
                if items.len() != 2 {
                    anyhow::bail!("Invalid string Topic")
                }
                let sub_account_id = items[1].parse()?;
                if t == "fullExitEvent" {
                    Ok(Self::FullExitEvent { sub_account_id })
                } else {
                    Ok(Self::TxExecuteResult { sub_account_id })
                }
            }
            _ => {
                anyhow::bail!("Invalid string topic")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topic() {
        let t1 = Topic::FullExitEvent {
            sub_account_id: SubAccountId(1),
        };
        let t2 = Topic::TxExecuteResult {
            sub_account_id: SubAccountId(1),
        };
        let s1 = t1.to_string();
        let s2 = t2.to_string();
        assert_eq!(s1, "fullExitEvent@1");
        assert_eq!(s2, "txExecuteResult@1");
        let tt1 = Topic::from_str(&s1).unwrap();
        let tt2 = Topic::from_str(&s2).unwrap();
        assert_eq!(t1, tt1);
        assert_eq!(t2, tt2);

        let topics = vec![
            Topic::FullExitEvent {
                sub_account_id: SubAccountId(1),
            },
            Topic::FullExitEvent {
                sub_account_id: SubAccountId(2),
            },
        ];
        assert!(t1.matched(&topics));
        let topics2 = vec![
            Topic::TxExecuteResult {
                sub_account_id: SubAccountId(1),
            },
            Topic::FullExitEvent {
                sub_account_id: SubAccountId(2),
            },
        ];
        assert!(!t1.matched(&topics2));
        let s = serde_json::to_string(&topics2).unwrap();
        println!("{s}");
        let s_expect = r#"["txExecuteResult@1","fullExitEvent@2"]"#;
        assert_eq!(s, s_expect);
    }
}
