use std::{
    fmt::Debug,
    pin::Pin,
    sync::{atomic::AtomicUsize, Arc},
};

use anyhow::Result;
use futures::Future;
use log::{info, warn};
use tokio::{
    net::{TcpListener, ToSocketAddrs},
    runtime,
};

use crate::{
    context::MessageContext,
    handler,
    protocol::{
        event::{message::MessageEvent, meta::MetaEvent, Event},
        handshake::HandshakeCallback,
    },
    rule::keyword::{KeywordRule, KeywordRuleBuilder},
};

pub trait StaticFn = Sync + Send + 'static;

pub type AsyncFnReturnType<T = ()> = Pin<Box<dyn Future<Output = T> + Send>>;

pub type EventHandlerType = Box<dyn Fn(Arc<Bot>, Event) -> AsyncFnReturnType<()> + StaticFn>;
pub type MetaHandlerType = Box<dyn Fn(Arc<Bot>, MetaEvent) -> AsyncFnReturnType<()> + StaticFn>;
pub type MessageHandlerType =
    Box<dyn Fn(MessageContext, MessageEvent) -> AsyncFnReturnType<()> + StaticFn>;
// pub type MessageEventHandlerType = Box<dyn Fn(MessageContext, Event) -> AsyncFnReturnType<()> + StaticFn>;

pub struct BotHandler {
    pub(crate) event_handler: Vec<EventHandlerType>,
    pub(crate) meta_handler: Vec<MetaHandlerType>,
    pub(crate) message_handler: Vec<MessageHandlerType>,
    pub(crate) keyword_handler: KeywordRule,
}

pub struct Bot {
    pub(crate) access_token: &'static str,
    pub(crate) entry_point: &'static str,
    pub(crate) sequence_number: AtomicUsize,
    pub(crate) handler: BotHandler,
}

impl Bot {
    pub fn run<T: ToSocketAddrs + Debug>(self, bind_address: T) -> Result<()> {
        let rt = runtime::Builder::new_current_thread()
            .enable_io()
            .build()
            .unwrap();

        rt.block_on(async move {
            let try_socket = TcpListener::bind(&bind_address).await;
            let listener = try_socket.expect("Bind address failed");
            info!("Listening on: {:?}", bind_address);

            let access_token = self.access_token;
            let entry_point = self.entry_point;
            let bot = Arc::new(self);

            while let Ok((stream, address)) = listener.accept().await {
                info!("Receive connection from: {}", address);

                //TODO: reuse callback
                let cb = HandshakeCallback::new(access_token, entry_point);
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

pub struct BotBuilder {
    pub(crate) access_token: &'static str,
    pub(crate) entry_point: &'static str,
    pub(crate) event_handler: Vec<EventHandlerType>,
    pub(crate) meta_handler: Vec<MetaHandlerType>,
    pub(crate) message_handler: Vec<MessageHandlerType>,
    pub(crate) keyword_handler: Vec<(&'static str, MessageHandlerType)>,
}

impl BotBuilder {
    pub fn new(access_token: &'static str, entry_point: &'static str) -> Self {
        BotBuilder {
            access_token,
            entry_point,
            event_handler: Vec::new(),
            meta_handler: Vec::new(),
            message_handler: Vec::new(),
            keyword_handler: Vec::new(),
        }
    }

    pub fn on_event(
        mut self,
        f: impl Fn(Arc<Bot>, Event) -> AsyncFnReturnType<()> + StaticFn,
    ) -> Self {
        self.event_handler.push(Box::new(f));
        self
    }

    pub fn on_meta(
        mut self,
        f: impl Fn(Arc<Bot>, MetaEvent) -> AsyncFnReturnType<()> + StaticFn,
    ) -> Self {
        self.meta_handler.push(Box::new(f));
        self
    }

    pub fn on_message(
        mut self,
        f: impl Fn(MessageContext, MessageEvent) -> AsyncFnReturnType<()> + StaticFn,
    ) -> Self {
        self.message_handler.push(Box::new(f));
        self
    }
    pub fn on_keyword(
        mut self,
        keyword: &'static str,
        f: impl Fn(MessageContext, MessageEvent) -> AsyncFnReturnType<()> + StaticFn,
    ) -> Self {
        self.keyword_handler.push((keyword, Box::new(f)));
        self
    }

    pub fn build(self) -> Bot {
        let mut keyword_handler_builder = KeywordRuleBuilder::new();
        self.keyword_handler
            .into_iter()
            .for_each(|(k, f)| keyword_handler_builder.insert(k, f));
        let keyword_handler = keyword_handler_builder.build();

        Bot {
            access_token: self.access_token,
            entry_point: self.entry_point,
            sequence_number: AtomicUsize::new(0),
            handler: BotHandler {
                event_handler: self.event_handler,
                meta_handler: self.meta_handler,
                message_handler: self.message_handler,
                keyword_handler,
            },
        }
    }
}
