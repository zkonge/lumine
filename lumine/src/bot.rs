use std::{fmt::Debug, pin::Pin, sync::Arc};

use anyhow::Result;
use futures::Future;
use log::{info, warn};
use tokio::{
    net::{TcpListener, ToSocketAddrs},
    runtime,
};

use crate::{
    handler,
    protocol::{
        event::{message::MessageEvent, meta::MetaEvent, Event},
        handshake::HandshakeCallback,
    },
};

pub trait StaticFn = Sync + Send + 'static;

pub type AsyncFnReturnType<T = ()> = Pin<Box<dyn Future<Output = T> + Send + Sync>>;

pub type EventHandlerType = Box<dyn Fn(Arc<Bot>, Event) -> AsyncFnReturnType<()> + StaticFn>;
pub type MetaHandlerType = Box<dyn Fn(Arc<Bot>, MetaEvent) -> AsyncFnReturnType<()> + StaticFn>;
pub type MessageHandlerType =
    Box<dyn Fn(Arc<Bot>, MessageEvent) -> AsyncFnReturnType<()> + StaticFn>;
// pub type MessageEventHandlerType = Box<dyn Fn(MessageContext, Event) -> AsyncFnReturnType<()> + StaticFn>;

pub struct BotConfig {
    /// 请求密钥
    pub(crate) access_token: &'static str,
}

impl BotConfig {
    pub fn new(access_token: &'static str) -> Self {
        BotConfig { access_token }
    }
}

#[derive(Default)]
pub struct BotHandler {
    pub(crate) event_handler: Vec<EventHandlerType>,
    pub(crate) meta_handler: Vec<MetaHandlerType>,
    pub(crate) message_handler: Vec<MessageHandlerType>,
}

pub struct Bot {
    pub(crate) config: BotConfig,
    pub(crate) handler: BotHandler,
}

impl Bot {
    pub fn new(config: BotConfig) -> Self {
        Bot {
            config,
            handler: BotHandler::default(),
        }
    }

    pub fn on_event(
        mut self,
        f: impl Fn(Arc<Bot>, Event) -> AsyncFnReturnType<()> + StaticFn,
    ) -> Self {
        self.handler.event_handler.push(Box::new(f));
        self
    }

    pub fn on_meta(
        mut self,
        f: impl Fn(Arc<Bot>, MetaEvent) -> AsyncFnReturnType<()> + StaticFn,
    ) -> Self {
        self.handler.meta_handler.push(Box::new(f));
        self
    }

    pub fn on_message(
        mut self,
        f: impl Fn(Arc<Bot>, MessageEvent) -> AsyncFnReturnType<()> + StaticFn,
    ) -> Self {
        self.handler.message_handler.push(Box::new(f));
        self
    }

    pub fn run<T: ToSocketAddrs + Debug>(self, bind_address: T) -> Result<()> {
        let rt = runtime::Builder::new_current_thread()
            .enable_io()
            .build()
            .unwrap();

        rt.block_on(async move {
            let try_socket = TcpListener::bind(&bind_address).await;
            let listener = try_socket.expect("Bind address failed");
            info!("Listening on: {:?}", bind_address);

            let access_token = self.config.access_token;
            let bot = Arc::new(self);

            while let Ok((stream, address)) = listener.accept().await {
                info!("Receive connection from: {}", address);

                //TODO: reuse callback
                let cb = HandshakeCallback::new(&access_token);
                let bot = bot.clone();

                match tokio_tungstenite::accept_hdr_async(stream, cb).await {
                    Ok(stream) => {
                        tokio::spawn(handler::handle_connection(stream, bot));
                    }
                    Err(e) => {
                        warn!("Websocket failure, detail: {:?}", e);
                        continue;
                    }
                }
            }
        });

        Ok(())
    }
}
