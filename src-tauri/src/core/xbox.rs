use discord_rich_presence as discord;
use futures::future::BoxFuture;
use serde::Deserialize;
use snafu::prelude::*;
use tauri::Manager;

#[derive(Debug, Snafu)]
enum Error {
    DiscordNew { source: crate::core::DiscordError },
    XboxApiAuthorizeFlow { source: crate::service::xbox::Error },
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct XboxPresenceRecord {
    state: String,
    devices: Vec<XboxDeviceRecord>,
    last_seen: XboxLastSeenRecord,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct XboxActivityRecord {
    rich_presence: String,
    media: Option<serde_json::Value>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct XboxDeviceRecord {
    #[serde(rename = "type")]
    r#type: String,
    titles: Vec<XboxTitleRecord>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct XboxLastSeenRecord {
    device_type: String,
    title_id: u32,
    title_name: String,
    #[serde(with = "time::serde::iso8601")]
    timestamp: time::OffsetDateTime,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct XboxTitleRecord {
    id: u32,
    name: String,
    activity: Option<XboxActivityRecord>,
    #[serde(with = "time::serde::iso8601")]
    last_modified: time::OffsetDateTime,
    placement: String,
    state: String,
}

struct XboxCore {
    app: tauri::AppHandle,
    discord_client: discord::DiscordIpcClient,
    discord_presence: Option<crate::core::DiscordPresence>,
    xbox_presence: Option<XboxPresenceRecord>,
}

impl XboxCore {
    const DISCORD_APPLICATION_ID: &str = "1053777655020912710";
    const TICK_RATE: u64 = 10;

    pub fn new(app: tauri::AppHandle) -> Result<Self, Error> {
        let discord_client = discord::DiscordIpcClient::new(Self::DISCORD_APPLICATION_ID)
            .map_err(Into::into)
            .context(DiscordNewSnafu)?;
        let discord_presence = None;
        let xbox_presence = None;
        Ok(Self {
            app,
            discord_client,
            discord_presence,
            xbox_presence,
        })
    }

    pub fn start(self) -> tauri::async_runtime::JoinHandle<Result<(), Error>> {
        tauri::async_runtime::spawn(async move { self.run().await })
    }

    fn run(&self) -> BoxFuture<Result<(), Error>> {
        Box::pin(async {
            loop {
                tokio::select! {
                    _ = self.exit() => break,
                    _ = self.wait() => self.tick().await?,
                }
            }
            Ok(())
        })
    }

    async fn exit(&self) -> tokio::sync::futures::Notified {
        self.app.state::<crate::app::Model>().inner().notifiers.exit.notified()
    }

    async fn wait(&self) {
        use tokio::time;
        time::sleep(time::Duration::from_secs(Self::TICK_RATE)).await
    }

    async fn tick(&self) -> Result<(), Error> {
        use crate::service::xbox;

        let model = self.app.state::<crate::app::Model>();

        if !model.config.read().await.services.xbox.enabled {
            return Ok(());
        }

        if model.session.xbox.read().await.is_none() {
            let reauthorize = false;
            xbox::authorize(&self.app, reauthorize)
                .await
                .context(XboxApiAuthorizeFlowSnafu)?;
        }

        if let Some(xbox) = &*model.session.xbox.read().await {}

        Ok(())
    }

    fn noteworthy_presence(&self) -> Result<Option<crate::core::DiscordPresence>, Error> {
        // let state = presence.get("state").context(SerdeJsonGetSnafu)?;
        // if state == "Online" {
        //     let devices = presence
        //         .get("devices")
        //         .context(SerdeJsonGetSnafu)?
        //         .pipe(Clone::clone)
        //         .pipe(serde_json::from_value::<Vec<serde_json::Value>>)
        //         .context(SerdeJsonFromSnafu)?;
        //     for device in devices {
        //         let titles = device
        //             .get("titles")
        //             .context(SerdeJsonGetSnafu)?
        //             .pipe(Clone::clone)
        //             .pipe(serde_json::from_value::<Vec<serde_json::Value>>)
        //             .context(SerdeJsonFromSnafu)?;
        //         for title in titles {
        //             let name = title.get("name").context(SerdeJsonGetSnafu)?;
        //             if name != "Online" {
        //                 let name =
        // serde_json::from_value::<String>(name.clone()).context(SerdeJsonFromSnafu)?;
        // return Ok(Some(DiscordPresence { name }));             }
        //         }
        //     }
        // }
        // Ok(None)
        Ok(None)
    }
}
