use redis::aio::Connection;
use serenity::model::id::{ChannelId, GuildId};
use std::error::Error;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct GuildSettings {
    pub enabled_channels: Vec<ChannelId>
}

impl GuildSettings {
    pub fn enabled_on(&self, channel: ChannelId) -> bool {
        for chan in self.enabled_channels.iter() {
            if chan.0 == channel.0 {
                return true;
            }
        }
        false
    }
}

pub async fn try_count(
    guild: GuildId,
    channel: ChannelId,
    count: u64,
    conn: Connection,
) -> Result<(), Box<dyn Error>> {
    // TODO: check that the channel/guild combination are correct
    Ok(())
}
