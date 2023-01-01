use snafu::prelude::*;
use tauri::async_runtime::JoinHandle;

mod xbox;

pub struct DiscordIpcErrorChain {
    error: Box<dyn std::error::Error + 'static + Sync + Send>,
    source: Option<Box<DiscordIpcErrorChain>>,
}

impl DiscordIpcErrorChain {
    fn chain(error: &dyn std::error::Error) -> Self {
        let source = error.source().map(DiscordIpcErrorChain::chain).map(Box::new);
        let error = error.to_string().into();
        Self { error, source }
    }

    fn from(error: Box<dyn std::error::Error>) -> Self {
        DiscordIpcErrorChain::chain(&*error)
    }
}

impl std::fmt::Debug for DiscordIpcErrorChain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DiscordIpcErrorChain")
            .field("error", &self.error)
            .field("source", &self.source)
            .finish()
    }
}

impl std::fmt::Display for DiscordIpcErrorChain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for DiscordIpcErrorChain {
    fn source(&self) -> Option<&(dyn snafu::Error + 'static)> {
        self.source.as_ref().map(|chain| &*chain.error as &_)
    }

    #[allow(deprecated)]
    fn description(&self) -> &str {
        self.error.description()
    }

    fn cause(&self) -> Option<&dyn snafu::Error> {
        self.source()
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to spawn a future onto the async runtime"))]
    TauriSpawnFuture {
        backtrace: snafu::Backtrace,
        source: tauri::Error,
    },
    #[snafu(display("Failed to get tauri managed state"))]
    TauriTryState { backtrace: snafu::Backtrace },
    #[snafu(display("Failed to get receive on oneshot channel"))]
    TokioSyncOneshotReceive {
        backtrace: snafu::Backtrace,
        source: tokio::sync::oneshot::error::RecvError,
    },
    #[snafu(display("Failed to start xbox core loop"))]
    XboxCoreStart { source: crate::core::xbox::Error },
}

#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Core {
    nintendo: JoinHandle<Result<(), Error>>,
    playstation: JoinHandle<Result<(), Error>>,
    steam: JoinHandle<Result<(), Error>>,
    xbox: JoinHandle<Result<(), crate::core::xbox::Error>>,
}

impl Core {
    // const NINTENDO_DISCORD_APPLICATION_ID: &str = "1000779677092286524";
    const NINTENDO_TICK_RATE: u64 = u64::MAX;
    // const PLAYSTATION_DISCORD_APPLICATION_ID: &str = "1053772210713657345";
    const PLAYSTATION_TICK_RATE: u64 = u64::MAX;
    // const STEAM_DISCORD_APPLICATION_ID: &str = "1053777465245437953";
    const STEAM_TICK_RATE: u64 = u64::MAX;

    // const XBOX_DISCORD_APPLICATION_ID: &str = "1053777655020912710";
    // const XBOX_TICK_RATE: u64 = 10;

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn init(
        rx: tokio::sync::oneshot::Receiver<tauri::AppHandle>,
    ) -> tauri::async_runtime::JoinHandle<Result<Self, Error>> {
        use tauri::Manager;
        tauri::async_runtime::spawn(async move {
            let app = rx.await.context(TokioSyncOneshotReceiveSnafu)?;
            let model = app
                .try_state::<crate::app::Model>()
                .context(TauriTryStateSnafu)?
                .inner()
                .clone();
            let nintendo = tauri::async_runtime::spawn(Self::nintendo(app.clone(), model.clone()));
            let playstation = tauri::async_runtime::spawn(Self::playstation(app.clone(), model.clone()));
            let steam = tauri::async_runtime::spawn(Self::steam(app.clone(), model.clone()));
            // let xbox = tauri::async_runtime::spawn(Self::xbox(app.clone(), model.clone()));
            let xbox = crate::core::xbox::XboxCore::start(&app);
            Ok(Self {
                nintendo,
                playstation,
                steam,
                xbox,
            })
        })
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    fn exit(model: &crate::app::Model) -> tokio::sync::futures::Notified {
        model.notifiers.exit.notified()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    async fn tick(secs: u64) {
        tokio::time::sleep(tokio::time::Duration::from_secs(secs)).await
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    async fn nintendo(_app: tauri::AppHandle, model: crate::app::Model) -> Result<(), Error> {
        let tick = || async {
            if !model.config.read().await.services.nintendo.enabled {
                return;
            }
        };
        loop {
            tokio::select! {
                _ = Self::exit(&model) => {
                    break;
                }
                _ = Self::tick(Self::NINTENDO_TICK_RATE) => {
                    tick().await;
                }
            }
        }
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    async fn playstation(_app: tauri::AppHandle, model: crate::app::Model) -> Result<(), Error> {
        let tick = || async {
            if !model.config.read().await.services.playstation.enabled {
                return;
            }
        };
        loop {
            tokio::select! {
                _ = Self::exit(&model) => {
                    break;
                }
                _ = Self::tick(Self::PLAYSTATION_TICK_RATE) => {
                    tick().await;
                }
            }
        }
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    async fn steam(_app: tauri::AppHandle, model: crate::app::Model) -> Result<(), Error> {
        let tick = || async {
            if !model.config.read().await.services.steam.enabled {
                return;
            }
        };
        loop {
            tokio::select! {
                _ = Self::exit(&model) => {
                    break;
                }
                _ = Self::tick(Self::STEAM_TICK_RATE) => {
                    tick().await;
                }
            }
        }
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub async fn terminate(self) -> Result<(), Error> {
        self.nintendo.await.context(TauriSpawnFutureSnafu)??;
        self.playstation.await.context(TauriSpawnFutureSnafu)??;
        self.steam.await.context(TauriSpawnFutureSnafu)??;
        self.xbox
            .await
            .context(TauriSpawnFutureSnafu)?
            .context(XboxCoreStartSnafu)?;
        Ok(())
    }
}
