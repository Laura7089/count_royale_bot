use redis::{ConnectionAddr, ConnectionInfo};
use std::env;
use std::error::Error;
use structopt::StructOpt;

#[derive(StructOpt)]
struct CliConfig {
    /// The address to use to connect to the Redis instance.
    /// Falls back to the value of the REDIS_ADDRESS environment variable.
    #[structopt(short, long)]
    redis_address: Option<String>,
    /// The token to use to authenticate with discord.
    /// Falls back to the value of the DISCORD_TOKEN environment variable.
    ///
    /// To get one, create an application + bot in the discord developer portal.
    #[structopt(short, long)]
    discord_token: Option<String>,
    /// The port to use to connect to the Redis instance
    /// Falls back to 6379
    #[structopt(short, long)]
    redis_port: Option<u16>,
    /// The username for the Redis database. Optional.
    #[structopt(short, long)]
    redis_user: Option<String>,
    /// The password for the Redis database. Optional.
    #[structopt(short, long)]
    redis_pass: Option<String>,
    /// The database number to use.
    ///
    /// Defaults to 0.
    #[structopt(short, long)]
    redis_db: Option<i64>,
}

pub struct CountniteConfig {
    pub redis_connect: ConnectionInfo,
    pub discord_token: String,
}

impl CountniteConfig {
    pub fn get() -> Result<Self, Box<dyn Error>> {
        let cli_config = CliConfig::from_args();

        Ok(Self {
            discord_token: cli_config
                .discord_token
                .unwrap_or(env::var("DISCORD_TOKEN")?),
            redis_connect: ConnectionInfo {
                addr: Box::new(ConnectionAddr::Tcp(
                    cli_config
                        .redis_address
                        .unwrap_or(env::var("REDIS_ADDRESS")?),
                    cli_config.redis_port.unwrap_or(6379),
                )),
                db: 0,
                username: None,
                passwd: None,
            },
        })
    }
}
