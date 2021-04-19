use serde::{Deserialize, Serialize};

// use serde_json::Value;

// use crate::protocol::message::MessageSegment;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum PrivateSubType {
    Friend,
    Group,
    Other,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum GroupSubType {
    Normal,
    Anonymous,
    Notice,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Anonymous {
    pub id: i64,
    pub name: String,
    pub flag: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Sex {
    Male,
    Female,
    Unknown,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Owner,
    Admin,
    Member,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Sender {
    pub user_id: i64,
    pub nickname: String,
    pub sex: Option<Sex>,
    pub age: Option<i32>,
    pub card: Option<String>,
    pub area: Option<String>,
    pub level: Option<String>,
    pub role: Option<Role>,
    pub title: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "message_type")]
#[serde(rename_all = "snake_case")]
pub enum MessageEvent {
    Private {
        sub_type: PrivateSubType,
        message_id: i32,
        user_id: i64,
        message: String,
        raw_message: String,
        font: i32,
        sender: Sender,
    },
    Group {
        sub_type: GroupSubType,
        message_id: i32,
        group_id: i32,
        user_id: i64,
        anonymous: Option<Anonymous>,
        message: String,
        raw_message: String,
        font: i32,
        sender: Sender,
    },
}
