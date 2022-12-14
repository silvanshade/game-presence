use snafu::prelude::*;
use std::sync::Arc;
use tokio::sync::{
    watch::{Receiver, Sender},
    Mutex,
    RwLock,
};

mod build;
pub mod config;

pub use build::BuildInfo;
pub use config::Config;

#[derive(Clone)]
pub struct Model {
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

impl Model {
    pub async fn init() -> Result<Self, Error> {
        let payload = {
            let provenience = crate::app::ipc::Provenience::default();
            let config = crate::app::model::Config::read().await.context(ConfigLoadSnafu)?;
            crate::app::ipc::Payload { provenience, config }
        };
        let model = {
            let (tx, rx) = tokio::sync::watch::channel(payload.clone());
            let (tx, rx) = (Arc::new(Mutex::new(tx)), Arc::new(RwLock::new(rx)));
            Self { tx, rx }
        };
        model.tx.lock().await.send(payload).context(TokioWatchSendSnafu)?;
        Ok(model)
    }

    async fn update(
        &self,
        data: crate::app::model::Config,
        provenience: crate::app::ipc::Provenience,
    ) -> Result<(), Error> {
        data.write().await.context(ConfigSaveSnafu)?;
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
