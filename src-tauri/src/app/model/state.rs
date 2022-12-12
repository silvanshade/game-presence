use snafu::prelude::*;
use std::sync::Arc;
use tokio::sync::{
    watch::{Receiver, Sender},
    Mutex,
    RwLock,
};

#[derive(Clone)]
pub struct State {
    pub tx: Arc<Mutex<Sender<crate::app::ipc::Payload>>>,
    pub rx: Arc<RwLock<Receiver<crate::app::ipc::Payload>>>,
}

#[derive(Debug, Snafu)]
pub enum Error {
    ConfigChannelsInit {
        source: crate::app::model::config::Error,
    },
    ConfigLoad {
        source: crate::app::model::config::Error,
    },
    ConfigSave {
        source: crate::app::model::config::Error,
    },
    TokioWatchSend {
        source: tokio::sync::watch::error::SendError<crate::app::ipc::Payload>,
    },
}

impl State {
    pub async fn init() -> Result<Self, Error> {
        let (tx, rx) = tokio::sync::watch::channel(crate::app::ipc::Payload::default());
        let (tx, rx) = (Arc::new(Mutex::new(tx)), Arc::new(RwLock::new(rx)));
        let state = Self { tx, rx };
        let config = crate::app::model::Config::load().await.context(ConfigLoadSnafu)?;
        state.update_with_broadcast(config).await?;
        Ok(state)
    }

    async fn update(
        &self,
        data: crate::app::model::Config,
        provenience: crate::app::ipc::Provenience,
    ) -> Result<(), Error> {
        data.save().await.context(ConfigSaveSnafu)?;
        let payload = crate::app::ipc::Payload {
            provenience,
            config: data,
        };
        self.tx.lock().await.send(payload).context(TokioWatchSendSnafu)?;
        Ok(())
    }

    pub async fn update_without_rebroadcast(&self, config: crate::app::model::Config) -> Result<(), Error> {
        self.update(config, crate::app::ipc::Provenience::Frontend).await
    }

    pub async fn update_with_broadcast(&self, config: crate::app::model::Config) -> Result<(), Error> {
        self.update(config, crate::app::ipc::Provenience::Backend).await
    }
}
