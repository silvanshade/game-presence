use crate::{core::discord, service::xbox};
use discord_ipc::DiscordIpc;
use discord_rich_presence as discord_ipc;
use futures::future::BoxFuture;
use snafu::prelude::*;
use tap::prelude::*;
use tauri::Manager;

#[derive(Debug, Snafu)]
pub enum Error {
    DiscordIpcClose { source: crate::core::DiscordIpcError },
    DiscordIpcConnect { source: crate::core::DiscordIpcError },
    DiscordIpcNew { source: crate::core::DiscordIpcError },
    DiscordIpcReconnect { source: crate::core::DiscordIpcError },
    DiscordIpcSetActivity { source: crate::core::DiscordIpcError },
    DiscordPresenceFromXbox { source: crate::core::discord::Error },
    XboxAuthorize { source: crate::service::xbox::Error },
    XboxPresence { source: crate::service::xbox::Error },
}

pub struct XboxCore {
    app: tauri::AppHandle,
    discord_client: discord_ipc::DiscordIpcClient,
    discord_presence: Option<discord::Presence>,
    xbox_presence: Option<xbox::PresenceRecord>,
}

impl XboxCore {
    const DISCORD_APPLICATION_ID: &str = "1056148753528131654";
    const TICK_RATE: u64 = 7;

    fn new(app: tauri::AppHandle) -> Result<Self, Error> {
        let mut discord_client = discord_ipc::DiscordIpcClient::new(Self::DISCORD_APPLICATION_ID)
            .map_err(Into::into)
            .context(DiscordIpcNewSnafu)?;
        discord_client
            .connect()
            .map_err(Into::into)
            .context(DiscordIpcConnectSnafu)?;
        let discord_presence = None;
        let xbox_presence = None;
        Ok(Self {
            app,
            discord_client,
            discord_presence,
            xbox_presence,
        })
    }

    pub fn start(app: &tauri::AppHandle) -> tauri::async_runtime::JoinHandle<Result<(), Error>> {
        let this = Self::new(app.clone());
        tauri::async_runtime::spawn(async move { this?.run().await })
    }

    fn run(&mut self) -> BoxFuture<Result<(), Error>> {
        Box::pin(async {
            loop {
                tokio::select! {
                    _ = self.exit().notified() => break,
                    _ = self.wait() => self.tick().await?,
                }
            }
            Ok(())
        })
    }

    fn exit(&self) -> &tokio::sync::Notify {
        &*self.app.state::<crate::app::Model>().inner().notifiers.exit
    }

    async fn wait(&self) {
        use tokio::time;
        time::sleep(time::Duration::from_secs(Self::TICK_RATE)).await
    }

    async fn tick(&mut self) -> Result<(), Error> {
        let app = self.app.clone();
        let model = app.state::<crate::app::Model>();
        if !model.config.read().await.services.xbox.enabled {
            return Ok(());
        }
        if model.session.xbox.read().await.is_none() {
            let reauthorize = false;
            let authorize_result = xbox::authorize(&self.app, reauthorize).await;
            authorize_result.context(XboxAuthorizeSnafu)?;
        }
        if let Some(xsts) = &*model.session.xbox.read().await {
            let xbox_presence_result = xbox::presence(xsts).await;
            self.xbox_presence = xbox_presence_result.context(XboxPresenceSnafu)?.pipe(Some);
            self.process_xbox_presence().await?;
        }
        Ok(())
    }

    async fn process_xbox_presence(&mut self) -> Result<(), Error> {
        if let Some(xbox_presence) = &self.xbox_presence {
            let discord_presence = discord::Presence::from_xbox(&xbox_presence)
                .await
                .context(DiscordPresenceFromXboxSnafu)?;
            if discord::Presence::differs_modulo_time(&self.discord_presence, &discord_presence) {
                self.discord_presence = discord_presence;
                self.refresh_discord_presence()?;
            }
        }
        Ok(())
    }

    fn refresh_discord_presence(&mut self) -> Result<(), Error> {
        if let Some(discord_presence) = &self.discord_presence {
            use discord_ipc::activity::{Activity, Assets, Button, Timestamps};
            let details = &discord_presence.details;
            let assets = Assets::new()
                .large_image(&discord_presence.assets_large_image)
                .large_text(&discord_presence.assets_large_text)
                .small_image(&discord_presence.assets_small_image)
                .small_text(&discord_presence.assets_small_text);
            let timestamps = Timestamps::new().start(discord_presence.time_start as i64);
            let buttons = std::iter::empty()
                .chain(&discord_presence.button_store)
                .chain(&discord_presence.button_twitch)
                .map(|(label, url)| Button::new(label, url.as_str()))
                .collect();
            let activity = Activity::new()
                .details(details)
                .assets(assets)
                .timestamps(timestamps)
                .buttons(buttons);

            self.discord_client
                .reconnect()
                .map_err(Into::into)
                .context(DiscordIpcReconnectSnafu)?;
            self.discord_client
                .set_activity(activity)
                .map_err(Into::into)
                .context(DiscordIpcSetActivitySnafu)?;

            println!("presence updated");
        } else {
            self.discord_client
                .close()
                .map_err(Into::into)
                .context(DiscordIpcCloseSnafu)?;
        }
        Ok(())
    }
}
