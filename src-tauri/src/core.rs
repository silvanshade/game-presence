use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    TokioSpawnJoin { source: tauri::Error },
}

pub type ServiceLoop = tauri::async_runtime::JoinHandle<Result<(), Error>>;

pub struct Core {
    nintendo: ServiceLoop,
    playstation: ServiceLoop,
    steam: ServiceLoop,
    xbox: ServiceLoop,
}

impl Core {
    const NINTENDO_TICK_RATE: u64 = u64::MAX;
    const PLAYSTATION_TICK_RATE: u64 = u64::MAX;
    const STEAM_TICK_RATE: u64 = u64::MAX;
    const XBOX_TICK_RATE: u64 = 5;

    pub fn new(model: crate::app::Model) -> Self {
        let nintendo = tauri::async_runtime::spawn(Self::nintendo(model.clone()));
        let playstation = tauri::async_runtime::spawn(Self::playstation(model.clone()));
        let steam = tauri::async_runtime::spawn(Self::steam(model.clone()));
        let xbox = tauri::async_runtime::spawn(Self::xbox(model.clone()));
        Self {
            nintendo,
            playstation,
            steam,
            xbox,
        }
    }

    fn exit(model: &crate::app::Model) -> tokio::sync::futures::Notified {
        model.notifiers.exit.notified()
    }

    async fn tick(secs: u64) {
        tokio::time::sleep(tokio::time::Duration::from_secs(secs)).await
    }

    async fn nintendo(model: crate::app::Model) -> Result<(), Error> {
        loop {
            tokio::select! {
                _ = Self::exit(&model) => {
                    break;
                }
                _ = Self::tick(Self::XBOX_TICK_RATE) => {
                    if !model.config.read().await.services.nintendo.enabled {
                        continue;
                    }
                }
            }
        }
        Ok(())
    }

    async fn playstation(model: crate::app::Model) -> Result<(), Error> {
        loop {
            tokio::select! {
                _ = Self::exit(&model) => {
                    break;
                }
                _ = Self::tick(Self::XBOX_TICK_RATE) => {
                    if !model.config.read().await.services.playstation.enabled {
                        continue;
                    }
                }
            }
        }
        Ok(())
    }

    async fn steam(model: crate::app::Model) -> Result<(), Error> {
        loop {
            tokio::select! {
                _ = Self::exit(&model) => {
                    break;
                }
                _ = Self::tick(Self::XBOX_TICK_RATE) => {
                    if !model.config.read().await.services.steam.enabled {
                        continue;
                    }
                }
            }
        }
        Ok(())
    }

    async fn xbox(model: crate::app::Model) -> Result<(), Error> {
        loop {
            tokio::select! {
                _ = Self::exit(&model) => {
                    break;
                }
                _ = Self::tick(Self::XBOX_TICK_RATE) => {
                    if !model.config.read().await.services.xbox.enabled {
                        continue;
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn finish(self) -> Result<(), Error> {
        self.nintendo.await.context(TokioSpawnJoinSnafu)??;
        self.playstation.await.context(TokioSpawnJoinSnafu)??;
        self.steam.await.context(TokioSpawnJoinSnafu)??;
        self.xbox.await.context(TokioSpawnJoinSnafu)??;
        Ok(())
    }
}
