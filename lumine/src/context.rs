use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;

use crate::protocol::api::{SendGroupMsg, SendPrivateMsg, API};

#[derive(Debug)]
pub struct MessageContext {
    pub user_id: i64,
    pub group_id: Option<i64>,
    sequence_number: usize,
    sender: UnboundedSender<Message>,
}

impl MessageContext {
    pub fn new(
        user_id: i64,
        group_id: Option<i64>,
        sequence_number: usize,
        sender: UnboundedSender<Message>,
    ) -> Self {
        MessageContext {
            user_id,
            group_id,
            sequence_number,
            sender,
        }
    }

    pub async fn send(&self, message: &str) {
        let api = match self.group_id {
            Some(group_id) => {
                let params = SendGroupMsg {
                    group_id,
                    message: message.to_owned(),
                };
                API::SendGroupMsg {
                    params,
                    echo: self.sequence_number,
                }
            }
            None => {
                let params = SendPrivateMsg {
                    user_id: self.user_id,
                    message: message.to_owned(),
                };
                API::SendPrivateMsg {
                    params,
                    echo: self.sequence_number,
                }
            }
        };

        self.sender.send(Message::text(api.build())).unwrap();
    }
}
