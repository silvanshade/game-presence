use snafu::prelude::*;
use tokio::sync::RwLock;

mod config;
pub use self::config::Config;

#[derive(Debug, Snafu)]
pub enum Error {
    Config { source: self::config::Error },
}

#[derive(Debug, Default)]
pub struct Model {
    pub config: RwLock<self::Config>,
}

impl Model {
    pub async fn load(&self) -> Result<(), self::Error> {
        let config = self::Config::load().await.context(ConfigSnafu)?;
        *self.config.write().await = config;
        Ok(())
    }
}
