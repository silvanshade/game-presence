use snafu::prelude::*;
use std::sync::Arc;
use tokio::sync::RwLock;

mod build;
pub mod config;
mod gui;
mod presence;
mod session;

pub use build::BuildInfo;
pub use config::Config;
pub use gui::Gui;

#[derive(Clone)]
pub struct Model {
    pub config: Arc<RwLock<Config>>,
    pub gui: Arc<RwLock<Gui>>,
    pub notifiers: Notifiers,
}

#[derive(Clone, Default)]
pub struct Notifiers {
    pub gui: Arc<tokio::sync::Notify>,
    pub exit: Arc<tokio::sync::Notify>,
    pub nintendo_auth_ready: Arc<tokio::sync::Notify>,
    pub playstation_auth_ready: Arc<tokio::sync::Notify>,
    pub steam_auth_ready: Arc<tokio::sync::Notify>,
    pub xbox_auth_ready: Arc<tokio::sync::Notify>,
}

#[derive(Debug, Snafu)]
pub enum Error {
    ConfigLoad { source: crate::app::model::config::Error },
    ConfigSave { source: crate::app::model::config::Error },
    ConfigTryIntoGui { source: crate::app::model::gui::Error },
    GuiSynchronizeWithConfig { source: crate::app::model::gui::Error },
}

impl Model {
    pub async fn init() -> Result<Self, Error> {
        let config = crate::app::model::Config::load().await.context(ConfigLoadSnafu)?;

        let mut gui = crate::app::model::Gui::default();
        gui.synchronize_with_config(&config)
            .context(GuiSynchronizeWithConfigSnafu)?;

        let model = Self {
            config: Arc::new(RwLock::new(config)),
            gui: Arc::new(RwLock::new(gui)),
            notifiers: Default::default(),
        };

        Ok(model)
    }

    pub async fn read_config(&self) -> tokio::sync::RwLockReadGuard<crate::app::model::Config> {
        self.config.read().await
    }

    pub async fn write_config(&self, config: crate::app::model::Config) -> Result<(), Error> {
        let mut gui = self.gui.write().await;
        gui.synchronize_with_config(&config)
            .context(GuiSynchronizeWithConfigSnafu)?;
        config.save().await.context(ConfigSaveSnafu)?;
        *self.config.write().await = config;
        self.notifiers.gui.notify_waiters();
        Ok(())
    }

    pub async fn write_gui(&self, gui: crate::app::model::Gui) -> Result<(), Error> {
        let mut config = self.config.write().await;
        config.synchronize_with_gui(&gui);
        config.save().await.context(ConfigSaveSnafu)?;
        *self.gui.write().await = gui;
        Ok(())
    }
}
