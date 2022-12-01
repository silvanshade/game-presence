use snafu::prelude::*;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct State {
    pub config: Arc<RwLock<crate::app::model::config::Channels>>,
}

#[derive(Debug, Snafu)]
pub enum Error {
    ConfigChannelsInit { source: crate::app::model::config::Error },
}

impl State {
    pub fn init() -> Result<Self, Error> {
        let config = crate::app::model::config::Channels::init().context(ConfigChannelsInitSnafu)?;
        let config = Arc::new(RwLock::new(config));
        Ok(Self { config })
    }
}
