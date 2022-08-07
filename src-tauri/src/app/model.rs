use snafu::prelude::*;
use tauri::async_runtime::RwLock;

pub mod config;
pub use self::config::Config;

#[derive(Debug, Snafu)]
pub enum Error {
    Config { source: self::config::Error },
}

#[derive(Default)]
pub struct Model {
    pub config: RwLock<self::Config>,
    pub discord_ipc_client: RwLock<Option<discord_rich_presence::DiscordIpcClient>>,
}

impl Model {
    pub async fn load(&self) -> Result<(), self::Error> {
        let config = self::Config::load().await.context(ConfigSnafu)?;
        *self.config.write().await = config;
        Ok(())
    }
}
