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

    async fn nintendo(model: crate::app::Model) -> Result<(), Error> {
        loop {
            let done = model.notifiers.exit.notified();
            let tick = async {
                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                if !model.config.read().await.services.nintendo.enabled {
                    println!("nintendo: skip");
                    return;
                }
                println!("nintendo: tick");
            };
            tokio::select! {
                _ = done => {
                    println!("nintendo: exit");
                    break;
                }
                _ = tick => {
                    continue;
                }
            }
        }
        Ok(())
    }

    async fn playstation(model: crate::app::Model) -> Result<(), Error> {
        loop {
            let done = model.notifiers.exit.notified();
            let task = async {
                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                if !model.config.read().await.services.playstation.enabled {
                    println!("playstation: skip");
                    return;
                }
                println!("playstation: tick");
            };
            tokio::select! {
                _ = done => {
                    println!("playstation: exit");
                    break;
                }
                _ = task => {
                    continue;
                }
            }
        }
        Ok(())
    }

    async fn steam(model: crate::app::Model) -> Result<(), Error> {
        loop {
            let done = model.notifiers.exit.notified();
            let tick = async {
                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                if !model.config.read().await.services.steam.enabled {
                    println!("steam: skip");
                    return;
                }
                println!("steam: tick");
            };
            tokio::select! {
                _ = done => {
                    println!("steam: exit");
                    break;
                }
                _ = tick => {
                    continue;
                }
            }
        }
        Ok(())
    }

    async fn xbox(model: crate::app::Model) -> Result<(), Error> {
        loop {
            let done = model.notifiers.exit.notified();
            let tick = async {
                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                if !model.config.read().await.services.xbox.enabled {
                    println!("xbox: skip");
                    return;
                }
                println!("xbox: tick");
            };
            tokio::select! {
                _ = done => {
                    println!("xbox: exit");
                    break;
                }
                _ = tick => {
                    continue;
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
