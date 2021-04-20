use std::{fmt::Debug, sync::Arc};

use anyhow::Result;
use log::{info, warn};
use tokio::{
    net::{TcpListener, ToSocketAddrs},
    runtime,
    sync::mpsc::UnboundedSender,
};
use tokio_tungstenite::tungstenite::Message;

use crate::{
    handler,
    protocol::{event::Event, handshake::HandshakeCallback},
    AsyncCallbackReturnType, AsyncCallbackType,
};

pub struct BotConfig {
    /// 请求密钥
    pub(crate) access_token: &'static str,
}

impl BotConfig {
    pub fn new(access_token: &'static str) -> Self {
        BotConfig { access_token }
    }
}

pub struct Bot {
    pub(crate) config: BotConfig,
    pub(crate) handlers: Vec<AsyncCallbackType>,
}

impl Bot {
    pub fn new(config: BotConfig) -> Self {
        Bot {
            config,
            handlers: Vec::new(),
        }
    }

    pub fn on_event(
        mut self,
        f: impl Fn(Event, UnboundedSender<Message>) -> AsyncCallbackReturnType + Send + Sync + 'static,
    ) -> Self {
        self.handlers.push(Box::new(f));
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
