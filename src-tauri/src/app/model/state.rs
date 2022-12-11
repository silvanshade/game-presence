use snafu::prelude::*;

#[derive(Clone)]
pub struct State {
    pub config: crate::app::model::config::Channels,
}

#[derive(Debug, Snafu)]
pub enum Error {
    ConfigChannelsInit {
        source: crate::app::model::config::Error,
    },
    ConfigLoad {
        source: crate::app::model::config::Error,
    },
    ConfigWrite {
        source: crate::app::data::config::Error,
    },
    TokioWatchSend {
        source: tokio::sync::watch::error::SendError<crate::app::ipc::Payload<crate::app::model::Config>>,
    },
}

impl State {
    pub fn init() -> Result<Self, Error> {
        let config = crate::app::model::config::Channels::init().context(ConfigChannelsInitSnafu)?;
        Ok(Self { config })
    }

    async fn update(
        &self,
        data: crate::app::model::Config,
        provenience: crate::app::ipc::Provenience,
    ) -> Result<(), Error> {
        crate::app::data::Config::from(data.clone())
            .write()
            .await
            .context(ConfigWriteSnafu)?;
        let payload = crate::app::ipc::Payload { provenience, data };
        self.config.tx.lock().await.send(payload).context(TokioWatchSendSnafu)?;
        Ok(())
    }

    pub async fn update_without_rebroadcast(&self, config: crate::app::model::Config) -> Result<(), Error> {
        self.update(config, crate::app::ipc::Provenience::Frontend).await
    }

    pub async fn update_with_broadcast(&self, config: crate::app::model::Config) -> Result<(), Error> {
        self.update(config, crate::app::ipc::Provenience::Backend).await
    }
}
