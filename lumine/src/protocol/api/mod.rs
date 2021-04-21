use serde::{Deserialize, Serialize};

// {
//     "action": "send_private_msg",
//     "params": {
//         "user_id": 10001000,
//         "message": "你好"
//     },
//     "echo": "123"
// }

macro_rules! api_item {
    (
        $(#[$a:meta])*
        pub enum API {
            $($item:ident,)+
        }
    ) => {
        $(#[$a])*
        pub enum API{
            $($item { params: $item, echo: usize },)+
        }
    };
}

trait APIItem{}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SendPrivateMsg {
    pub user_id: i64,
    pub message: String,
}
impl APIItem for SendPrivateMsg{}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SendGroupMsg {
    pub group_id: i32,
    pub message: String,
}
impl APIItem for SendGroupMsg{}

api_item! {
    #[derive(Serialize, Deserialize, Clone, Debug)]
    #[serde(tag = "action")]
    #[serde(rename_all = "snake_case")]
    pub enum API {
        SendPrivateMsg,
        SendGroupMsg,
    }
}

impl API {
    pub fn build(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
