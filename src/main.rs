#[macro_use]
extern crate log;

mod countnite;
mod config;

use async_trait::async_trait;
use std::error::Error;
use redis::AsyncCommands;
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    info!("Starting Countnite...");

    let config = config::CountniteConfig::get()?;

    let redis_client = redis::Client::open(config.redis_connect)?;
    let mut discord_client = serenity::Client::new(config.discord_token)
        .event_handler(DiscordEventHandler{ redis_client }).await?;

    discord_client.start().await?;
    Ok(())
}

struct DiscordEventHandler {
    redis_client: redis::Client
}

#[async_trait]
impl EventHandler for DiscordEventHandler {
    async fn message(&self, _: Context, msg: Message) {
        // Extract guild ID and channel ID
        let guild_id = if let Some(id) = msg.guild_id {
            id
        } else {
            error!("Couldn't extract a guild ID!");
            return;
        };
        let channel_id = msg.channel_id;

        // Get a redis connection
        let mut conn = match self.redis_client.get_async_connection().await {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to get a redis connection: {}", e);
                return;
            }
        };
        let guild_settings_raw: String = match conn.get(format!("{}/settings", guild_id)).await {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to retrieve settings for guild {}: {}", guild_id, e);
                return;
            }
        };
        let guild_settings: countnite::GuildSettings = match serde_json::from_str(&guild_settings_raw) {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to deserialize settings for guild {}: {}", guild_id, e);
                return;
            }
        };

        if !guild_settings.enabled_on(channel_id) {
            return;
        }

        let submitted_num: u64 = if let Some(num_raw) = msg.content.split(" ").next() {
            match num_raw.parse() {
                Ok(num) => num,
                // TODO: take action on bad formatting
                Err(_) => return,
            }
        } else {
            // TODO: take action on empty message
            return;
        };
        countnite::try_count(guild_id, channel_id, submitted_num, conn).await;
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected to discord as {}", ready.user.name);
    }
}
