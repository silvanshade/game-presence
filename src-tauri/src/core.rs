use discord_rich_presence as discord;
use snafu::prelude::*;
use tap::prelude::*;

mod xbox;

fn twitch_url(title: &str) -> Result<url::Url, Error> {
    use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
    let encoded_title = utf8_percent_encode(title, NON_ALPHANUMERIC).to_string();
    let base = "https://www.twitch.tv/directory/game";
    url::Url::parse(&format!("{}/{}", base, encoded_title)).context(UrlParseSnafu)
}

pub struct DiscordError(Box<dyn std::error::Error + 'static + Sync + Send>);

impl std::fmt::Debug for DiscordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("DiscordError").field(&self.0).finish()
    }
}

impl std::fmt::Display for DiscordError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for DiscordError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

impl From<Box<dyn std::error::Error>> for DiscordError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        error
            .to_string()
            .conv::<Box<dyn std::error::Error + 'static + Sync + Send>>()
            .pipe(DiscordError)
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    DiscordClearActivity {
        source: DiscordError,
    },
    DiscordClose {
        source: DiscordError,
    },
    DiscordConnect {
        source: DiscordError,
    },
    DiscordNew {
        source: DiscordError,
    },
    DiscordReconnect {
        source: DiscordError,
    },
    DiscordSetActivity {
        source: DiscordError,
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

#[derive(Debug, Eq, PartialEq)]
struct DiscordPresence {
    name: String,
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
        use crate::service::xbox::api;
        use discord::{DiscordIpc, DiscordIpcClient};
        use discord_rich_presence as discord;
        use std::sync::Arc;
        use tauri::Manager;
        use tokio::sync::RwLock;

        let prev_presence = Arc::new(RwLock::new(None::<DiscordPresence>));

        let mut discord = DiscordIpcClient::new(Self::XBOX_DISCORD_APPLICATION_ID)
            .map_err(Into::into)
            .context(DiscordNewSnafu)?;
        discord.connect().map_err(Into::into).context(DiscordConnectSnafu)?;
        let discord = Arc::new(RwLock::new(discord));

        let is_noteworthy = |presence: serde_json::Value| {
            let state = presence.get("state").context(SerdeJsonGetSnafu)?;
            if state == "Online" {
                let devices = presence
                    .get("devices")
                    .context(SerdeJsonGetSnafu)?
                    .pipe(Clone::clone)
                    .pipe(serde_json::from_value::<Vec<serde_json::Value>>)
                    .context(SerdeJsonFromSnafu)?;
                for device in devices {
                    let titles = device
                        .get("titles")
                        .context(SerdeJsonGetSnafu)?
                        .pipe(Clone::clone)
                        .pipe(serde_json::from_value::<Vec<serde_json::Value>>)
                        .context(SerdeJsonFromSnafu)?;
                    for title in titles {
                        let name = title.get("name").context(SerdeJsonGetSnafu)?;
                        if name != "Online" {
                            let name = serde_json::from_value::<String>(name.clone()).context(SerdeJsonFromSnafu)?;
                            return Ok(Some(DiscordPresence { name }));
                        }
                    }
                }
            }
            Ok::<Option<DiscordPresence>, Error>(None)
        };

        let tick = move |app: tauri::AppHandle,
                         discord: Arc<RwLock<DiscordIpcClient>>,
                         prev_presence: Arc<RwLock<Option<DiscordPresence>>>| async move {
            let model = app.state::<crate::app::Model>();
            if !model.config.read().await.services.xbox.enabled {
                return Ok(());
            }
            if model.session.xbox.read().await.is_none() {
                let reauthorize = false;
                api::authorize(&app, reauthorize)
                    .await
                    .context(XboxApiAuthorizeFlowSnafu)?;
            }
            if let Some(xbox) = &*model.session.xbox.read().await {
                let url = "https://userpresence.xboxlive.com/users/me";
                let user_hash = &xbox.display_claims.xui.uhs;
                let token = &xbox.token;

                let presence_response = reqwest::Client::new()
                    .get(url)
                    .header("Accept", "application/json")
                    .header("Accept-Language", "en-US")
                    .header("Authorization", format!("XBL3.0 x={};{}", user_hash, token))
                    .header("x-xbl-contract-version", "3")
                    .send()
                    .await
                    .context(ReqwestRequestSendSnafu)?
                    .json::<serde_json::Value>()
                    .await
                    .context(ReqwestRequestJsonSnafu)?;

                if let Some(next_presence) = is_noteworthy(presence_response)? {
                    if prev_presence.read().await.as_ref() == Some(&next_presence) {
                        return Ok(());
                    }
                    discord
                        .write()
                        .await
                        .reconnect()
                        .map_err(Into::into)
                        .context(DiscordReconnectSnafu)?;

                    if let Some(suggest) = api::autosuggest(&next_presence.name).await.context(ServiceXboxSnafu)? {
                        let large_image = suggest.image_url().context(XboxSuggestImageUrlSnafu)?;
                        let large_text = next_presence.name.clone();
                        let small_image = "small-icon";
                        let small_text = "playing on xbox";

                        let assets = discord::activity::Assets::new()
                            .large_image(large_image.as_str())
                            .large_text(large_text.as_str())
                            .small_image(small_image)
                            .small_text(small_text);

                        let timestamps = {
                            let start = std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .context(StdTimeDurationSinceSnafu)?
                                .as_secs() as i64;
                            discord::activity::Timestamps::new().start(start)
                        };

                        let store_url = suggest.store_url().context(XboxSuggestStoreUrlSnafu)?;
                        let store_button = discord::activity::Button::new("xbox.com", store_url.as_str());

                        let twitch_url = twitch_url(&next_presence.name)?;
                        let twitch_button = discord::activity::Button::new("twitch", twitch_url.as_str());

                        let buttons = vec![store_button, twitch_button];

                        let activity = discord::activity::Activity::new()
                            .details(&next_presence.name)
                            .assets(assets)
                            .timestamps(timestamps)
                            .buttons(buttons);

                        discord
                            .write()
                            .await
                            .set_activity(activity)
                            .map_err(Into::into)
                            .context(DiscordSetActivitySnafu)?;

                        *prev_presence.write().await = Some(DiscordPresence {
                            name: next_presence.name,
                        });

                        println!("presence: updated: {:#?}", prev_presence.read().await);
                    }
                } else {
                    discord
                        .write()
                        .await
                        .close()
                        .map_err(Into::into)
                        .context(DiscordCloseSnafu)?;
                    *prev_presence.write().await = None;
                    return Ok(());
                }
            }
            Ok(())
        };
        loop {
            println!("xbox: loop");
            tokio::select! {
                _ = Self::exit(&model) => {
                    break;
                }
                _ = Self::tick(Self::XBOX_TICK_RATE) => {
                    println!("xbox: tick");
                    tick(app.clone(), discord.clone(), prev_presence.clone()).await?;
                }
            }
        }
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
