use std::sync::atomic::Ordering;
use std::sync::Arc;

use futures::{SinkExt, StreamExt};
use log::{debug, error, info, warn};
use serde_json::{from_str, from_value, Value};
use tokio::sync::mpsc;
use tokio::{net::TcpStream, sync::mpsc::UnboundedSender};
use tokio_tungstenite::{
    tungstenite::{Error, Message},
    WebSocketStream,
};

use crate::{
    context::MessageContext,
    protocol::event::{message::MessageEvent, Event},
    Bot,
};

async fn dispatcher(bot: &Arc<Bot>, ws_message: String, sender: UnboundedSender<Message>) {
    let undetermined_message: Value = from_str(&ws_message).unwrap();
    if undetermined_message.get("post_type").is_some() {
        match from_value::<Event>(undetermined_message) {
            Ok(e) => {
                for f in bot.handler.event_handler.iter() {
                    f(bot.clone(), e.clone()).await;
                }
                match e {
                    Event::Message { event, .. } => {
                        let (user_id, group_id) = match event {
                            MessageEvent::Private {
                                user_id,
                                ref message,
                                ..
                            } => {
                                info!("Message from {}: {}", user_id, message);
                                (user_id, None)
                            }
                            MessageEvent::Group {
                                user_id,
                                group_id,
                                ref message,
                                ..
                            } => {
                                info!(
                                    "Message from {} in group {}: {}",
                                    user_id, group_id, message
                                );
                                (user_id, Some(group_id))
                            }
                        };
                        for f in bot.handler.message_handler.iter() {
                            let msg_ctx = MessageContext::new(
                                user_id,
                                group_id,
                                bot.sequence_number.fetch_add(1, Ordering::Relaxed),
                                sender.clone(),
                                bot.clone(),
                            );
                            f(msg_ctx, event.clone()).await;
                        }
                        let text_message = match event {
                            MessageEvent::Private { ref message, .. } => message,
                            MessageEvent::Group { ref message, .. } => message,
                        };
                        let f = bot.handler.keyword_handler.find(text_message);
                        if let Some(f) = f {
                            let msg_ctx = MessageContext::new(
                                user_id,
                                group_id,
                                bot.sequence_number.fetch_add(1, Ordering::Relaxed),
                                sender.clone(),
                                bot.clone(),
                            );
                            f(msg_ctx, event.clone()).await;
                        }
                    }
                    Event::MetaEvent { event, .. } => {
                        for f in bot.handler.meta_handler.iter() {
                            f(bot.clone(), event.clone()).await;
                        }
                    }
                    _ => (),
                }
            }
            Err(e) => warn!("Unknown message: {:?}", e),
        }
    } else {
        //TODO handle api resp
    }
}

pub(crate) async fn handle_connection(
    stream: WebSocketStream<TcpStream>,
    bot: Arc<Bot>,
) -> Result<(), Error> {
    let (mut writer, mut reader) = stream.split();

    let (tx, mut rx) = mpsc::unbounded_channel();

    let write_proc = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            debug!("Send websocket data: {:?}", msg);
            writer.send(msg).await.unwrap()
        }
    });

    let read_proc = tokio::spawn(async move {
        loop {
            if let Some(result) = reader.next().await {
                debug!("Get websocket data: {:?}", result);
                match result {
                    Ok(message) => match message {
                        Message::Text(text) => dispatcher(&bot, text, tx.clone()).await,
                        Message::Binary(_) => unimplemented!(),
                        Message::Ping(frame) => tx.send(Message::Pong(frame)).unwrap(),
                        Message::Pong(_) => unimplemented!(),
                        Message::Close(frame) => {
                            info!("Remote disconnect: {:?}", frame);
                            break;
                        }
                    },
                    Err(error) => {
                        error!(
                            "Error when handle websocket connection, message: {:?}",
                            error
                        );
                        break;
                    }
                }
            }
        }
    });

    read_proc.await.unwrap();
    write_proc.await.unwrap();

    Ok(())
}
