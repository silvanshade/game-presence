use snafu::prelude::*;
use std::sync::Arc;
use tauri::async_runtime::{Mutex, RwLock};

pub mod config;
pub mod discord;
pub mod steam;
pub use self::{config::Config, discord::Discord};

#[derive(Debug, Snafu)]
pub enum Error {
    ConfigLoad { source: self::config::Error },
    DiscordNew { source: self::discord::Error },
}

pub struct Model {
    pub config: Arc<RwLock<self::Config>>,
    pub discord: Arc<Mutex<self::discord::Discord>>,
    pub steam: self::steam::Steam,
}

impl Model {
    pub fn new() -> Result<Self, self::Error> {
        let config = Arc::<RwLock<self::Config>>::default();
        let discord = self::discord::Discord::new(config.clone()).context(DiscordNewSnafu)?;
        let discord = Arc::new(Mutex::new(discord));
        let steam = self::steam::Steam::new(config.clone(), discord.clone());
        Ok(Self { config, discord, steam })
    }
}
