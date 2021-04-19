use serde::{Deserialize, Serialize};

pub mod message;
pub mod meta;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EventInfo {
    pub time: i64,
    pub self_id: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "post_type")]
#[serde(rename_all = "snake_case")]
pub enum Event {
    Message {
        #[serde(flatten)]
        info: EventInfo,
        #[serde(flatten)]
        event: message::MessageEvent,
    },
    Notice {
        #[serde(flatten)]
        info: EventInfo,
    },
    Request {
        #[serde(flatten)]
        info: EventInfo,
    },
    MetaEvent {
        #[serde(flatten)]
        info: EventInfo,
        #[serde(flatten)]
        event_type: meta::MetaEventType,
        #[serde(flatten)]
        event: meta::MetaEvent,
    },
}
