use std::sync::Arc;

use env_logger;
use heim::process;
use log::info;

use lumine::handler_fn;
use lumine::{
    bot::BotBuilder,
    context::MessageContext,
    protocol::event::{message::MessageEvent, meta::MetaEvent, Event},
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
async fn message_handler(context: MessageContext, event: MessageEvent) {
    info!("Get message event: {:?}", event);
    if let Ok(r) = process::current().await {
        if let Ok(r) = r.memory().await {
            context
                .send(&format!(
                    "Physical Memory: {:.1}KiB\nVirtual Memory: {:.1}KiB",
                    r.rss().value as f64/1024.,
                    r.vms().value as f64/1024.
                ))
                .await;
        };
    };
}

fn main() {
    env_logger::init();

    let bot = BotBuilder::new("", "/cqhttp/ws")
        .on_keyword("/memory", message_handler)
        .build();

    bot.run("127.0.0.1:11001").unwrap();
}
