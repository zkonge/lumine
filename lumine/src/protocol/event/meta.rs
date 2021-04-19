use serde::{Deserialize, Serialize};

use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(tag = "meta_event_type")]
#[serde(rename_all = "snake_case")]
pub enum MetaEventType {
    Lifecycle,
    Heartbeat,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "meta_event_type")]
#[serde(rename_all = "snake_case")]
pub enum MetaEvent {
    Lifecycle { sub_type: String },
    Heartbeat { status: Value, interval: i64 },
}
