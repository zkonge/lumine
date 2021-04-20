use std::pin::Pin;

use futures::Future;
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;

pub use lumine_proc::handler_callback_fn;

pub use crate::bot::{Bot, BotConfig};
use crate::protocol::event::Event;

pub mod bot;
pub mod context;
pub mod handler;
pub mod protocol;
pub mod rule;

pub type AsyncCallbackReturnType = Pin<Box<dyn Future<Output = ()> + Send + Sync>>;
pub type AsyncCallbackType =
    Box<dyn Fn(Event, Sender) -> AsyncCallbackReturnType + Send + Sync + 'static>;
pub type Sender = UnboundedSender<Message>;
