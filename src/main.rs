#[macro_use]
extern crate log;

mod config;

use async_trait::async_trait;
use std::error::Error;
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    info!("Starting Countnite...");

    let config = config::CountniteConfig::get()?;

    let discord_client = serenity::Client::new(config.discord_token, DiscordEventHandler {})?;
    let redis_client = redis::Client::open(config.redis_connect)?;

    let bot_client = Countnite {
        discord_client,
        redis_client
    };

    bot_client.discord_client.start().await?;

    Ok(())
}

struct Countnite {
    discord_client: serenity::Client,
    redis_client: redis::Client,
}

struct DiscordEventHandler;

#[async_trait]
impl EventHandler for DiscordEventHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.channel_id.0 == 1u64 {
            info!("bingo!");
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected to discord as {}", ready.user.name);
    }
}
