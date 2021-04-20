use std::sync::Arc;

use env_logger;
use log::info;

use lumine::{handler_fn, protocol::event::meta::MetaEvent};
use lumine::{
    protocol::event::{message::MessageEvent, Event},
    Bot, BotConfig,
};

#[handler_fn]
async fn event_handler(_context: Arc<Bot>, event: Event) {
    info!("Get event: {:?}", event);
}

#[handler_fn]
async fn meta_handler(_context: Arc<Bot>, event: MetaEvent) {
    info!("Get meta event: {:?}", event);
}

#[handler_fn]
async fn message_handler(_context: Arc<Bot>, event: MessageEvent) {
    info!("Get message event: {:?}", event);
}

fn main() {
    env_logger::init();
    Bot::new(BotConfig::new(""))
        .on_event(event_handler)
        .on_meta(meta_handler)
        .on_message(message_handler)
        .run("127.0.0.1:11001")
        .unwrap();
}
