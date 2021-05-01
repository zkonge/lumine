use std::sync::Arc;

use env_logger;
use log::info;
use perf_monitor::mem::get_process_memory_info;

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
    if let Ok(r) = get_process_memory_info() {
        context
            .send(&format!(
                "Physical Memory: {}\nVirtual Memory: {}",
                r.resident_set_size,
                r.virtual_memory_size
            ))
            .await;
    };
}

fn main() {
    env_logger::init();

    let bot = BotBuilder::new("", "/cqhttp/ws")
        .on_keyword("/memory", message_handler)
        .build();

    bot.run("127.0.0.1:11001").unwrap();
}
