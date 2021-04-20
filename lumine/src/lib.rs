#![feature(trait_alias)]
pub use lumine_proc::handler_fn;

pub use crate::bot::{Bot, BotConfig};
pub use crate::bot::AsyncFnReturnType;

pub mod bot;
pub mod context;
pub mod handler;
pub mod protocol;
pub mod rule;

