use std::sync::Arc;

use env_logger;
use log::info;

use lumine::{bot::BotBuilder, handler_fn, protocol::event::meta::MetaEvent};
use lumine::{
    protocol::event::{message::MessageEvent, Event},
    Bot,
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

    let bot = BotBuilder::new("", "/cqhttp/ws")
        .on_keyword("qaq", message_handler)
        .build();

    bot.run("127.0.0.1:11001").unwrap();
}
