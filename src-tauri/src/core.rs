use discord_rich_presence as discord_ipc;
use snafu::prelude::*;
use tap::prelude::*;

mod discord;
mod xbox;

fn twitch_url(title: &str) -> Result<url::Url, Error> {
    use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
    let encoded_title = utf8_percent_encode(title, NON_ALPHANUMERIC).to_string();
    let base = "https://www.twitch.tv/directory/game";
    url::Url::parse(&format!("{}/{}", base, encoded_title)).context(UrlParseSnafu)
}

pub struct DiscordIpcError(Box<dyn std::error::Error + 'static + Sync + Send>);

impl std::fmt::Debug for DiscordIpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("DiscordError").field(&self.0).finish()
    }
}

impl std::fmt::Display for DiscordIpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for DiscordIpcError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

impl From<Box<dyn std::error::Error>> for DiscordIpcError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        error
            .to_string()
            .conv::<Box<dyn std::error::Error + 'static + Sync + Send>>()
            .pipe(DiscordIpcError)
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    DiscordClearActivity {
        source: DiscordIpcError,
    },
    DiscordClose {
        source: DiscordIpcError,
    },
    DiscordConnect {
        source: DiscordIpcError,
    },
    DiscordNew {
        source: DiscordIpcError,
    },
    DiscordReconnect {
        source: DiscordIpcError,
    },
    DiscordSetActivity {
        source: DiscordIpcError,
    },
    RegexNew {
        source: regex::Error,
    },
    ReqwestRequestJson {
        source: reqwest::Error,
    },
    ReqwestRequestSend {
        source: reqwest::Error,
    },
    SerdeJsonGet,
    SerdeJsonFrom {
        source: serde_json::Error,
    },
    ServiceXbox {
        source: crate::service::xbox::Error,
    },
    // SuggestImageUrl { source: crate::service::xbox::Error },
    // SuggestStoreUrl { source: crate::service::xbox::Error },
    StdTimeDurationSince {
        source: std::time::SystemTimeError,
    },
    TauriSpawn {
        source: tauri::Error,
    },
    TokioSyncOneshotReceive {
        source: tokio::sync::oneshot::error::RecvError,
    },
    XboxApiAuthorizeFlow {
        source: crate::service::xbox::Error,
    },
    XboxSuggestImageUrl {
        source: crate::service::xbox::Error,
    },
    XboxSuggestStoreUrl {
        source: crate::service::xbox::Error,
    },
    UrlParse {
        source: url::ParseError,
    },
}

// fn twitch_url(title: &str) -> Result<url::Url, Error> {
//     use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
//     let encoded_title = utf8_percent_encode(title, NON_ALPHANUMERIC).to_string();
//     let base = "https://www.twitch.tv/directory/game";
//     url::Url::parse(&format!("{}/{}", base, encoded_title)).context(UrlParseSnafu)
// }

pub type ServiceLoop = tauri::async_runtime::JoinHandle<Result<(), Error>>;

pub struct Core {
    nintendo: ServiceLoop,
    playstation: ServiceLoop,
    steam: ServiceLoop,
    xbox: ServiceLoop,
}

impl Core {
    // const NINTENDO_DISCORD_APPLICATION_ID: &str = "1000779677092286524";
    const NINTENDO_TICK_RATE: u64 = u64::MAX;
    // const PLAYSTATION_DISCORD_APPLICATION_ID: &str = "1053772210713657345";
    const PLAYSTATION_TICK_RATE: u64 = u64::MAX;
    // const STEAM_DISCORD_APPLICATION_ID: &str = "1053777465245437953";
    const STEAM_TICK_RATE: u64 = u64::MAX;
    const XBOX_DISCORD_APPLICATION_ID: &str = "1053777655020912710";
    const XBOX_TICK_RATE: u64 = 10;

    pub fn new(
        rx: tokio::sync::oneshot::Receiver<tauri::AppHandle>,
    ) -> tauri::async_runtime::JoinHandle<Result<Self, Error>> {
        use tauri::Manager;
        tauri::async_runtime::spawn(async move {
            let app = rx.await.context(TokioSyncOneshotReceiveSnafu)?;
            let model = app.state::<crate::app::Model>().inner().clone();
            let nintendo = tauri::async_runtime::spawn(Self::nintendo(app.clone(), model.clone()));
            let playstation = tauri::async_runtime::spawn(Self::playstation(app.clone(), model.clone()));
            let steam = tauri::async_runtime::spawn(Self::steam(app.clone(), model.clone()));
            let xbox = tauri::async_runtime::spawn(Self::xbox(app.clone(), model.clone()));
            Ok(Self {
                nintendo,
                playstation,
                steam,
                xbox,
            })
        })
    }

    fn exit(model: &crate::app::Model) -> tokio::sync::futures::Notified {
        model.notifiers.exit.notified()
    }

    async fn tick(secs: u64) {
        tokio::time::sleep(tokio::time::Duration::from_secs(secs)).await
    }

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

    async fn xbox(app: tauri::AppHandle, model: crate::app::Model) -> Result<(), Error> {
        Ok(())
    }

    pub async fn finish(self) -> Result<(), Error> {
        self.nintendo.await.context(TauriSpawnSnafu)??;
        self.playstation.await.context(TauriSpawnSnafu)??;
        self.steam.await.context(TauriSpawnSnafu)??;
        self.xbox.await.context(TauriSpawnSnafu)??;
        Ok(())
    }
}
