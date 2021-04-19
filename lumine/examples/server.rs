use env_logger;
use log::info;
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;

use lumine::handler_callback_fn;
use lumine::protocol::api::{SendGroupMsg, SendPrivateMsg, API};
use lumine::{
    protocol::event::{message::MessageEvent, Event},
    Bot, BotConfig,
};

#[handler_callback_fn]
async fn my_callback(event: Event, ctx: UnboundedSender<Message>) {
    match event {
        Event::Message { event, .. } => match event {
            MessageEvent::Private {
                user_id,
                message,
                sender,
                ..
            } => {
                info!(
                    "Get Private message from {}({}): {}",
                    sender.nickname, user_id, message
                );
                if message.starts_with("/echo") {
                    let params = SendPrivateMsg {
                        user_id,
                        message: message.strip_prefix("/echo").unwrap().trim().to_owned(),
                    };
                    ctx.send(Message::text(
                        API::SendPrivateMsg { params, echo: 666 }.build(),
                    ))
                    .unwrap();
                }
            }
            MessageEvent::Group {
                group_id,
                user_id,
                message,
                sender,
                ..
            } => {
                info!(
                    "Get Group message from {}({}) in group {}: {}",
                    sender.nickname, user_id, group_id, message
                );
                if message.starts_with("/group_echo") {
                    let params = SendGroupMsg {
                        group_id: group_id as i64,
                        message: message.strip_prefix("/group_echo").unwrap().trim().to_owned(),
                    };
                    ctx.send(Message::text(
                        API::SendGroupMsg { params, echo: 666 }.build(),
                    ))
                    .unwrap();
                }
            }
        },
        Event::MetaEvent { event_type, .. } => {
            info!("Get Meta event {:?}", event_type);
        }
        _ => (),
    }
}

fn main() {
    env_logger::init();
    Bot::new(BotConfig::new(""))
        .on_event(my_callback)
        .run("127.0.0.1:11001")
        .unwrap();
}
