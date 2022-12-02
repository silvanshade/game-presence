use snafu::prelude::*;

#[derive(Clone)]
pub struct State {
    pub config: crate::app::model::config::Channels,
}

#[derive(Debug, Snafu)]
pub enum Error {
    ConfigChannelsInit { source: crate::app::model::config::Error },
}

impl State {
    pub fn init() -> Result<Self, Error> {
        let config = crate::app::model::config::Channels::init().context(ConfigChannelsInitSnafu)?;
        Ok(Self { config })
    }
}
