use std::sync::Arc;

use futures::{SinkExt, StreamExt};
use log::{debug, error, info, warn};
use serde_json::from_str;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_tungstenite::{
    tungstenite::{Error, Message},
    WebSocketStream,
};

use crate::{protocol::event::Event, Bot};

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
                        Message::Text(text) => {
                            let message = from_str::<Event>(&text);
                            match message {
                                Ok(e) => {
                                    for f in bot.handler.event_handler.iter() {
                                        f(bot.clone(), e.clone()).await;
                                    }
                                    match e {
                                        Event::Message { event, .. } => {
                                            for f in bot.handler.message_handler.iter() {
                                                f(bot.clone(), event.clone()).await;
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
                        }
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
