use crate::{core::discord, service::xbox};
use discord_rich_presence as discord_ipc;
use futures::future::BoxFuture;
use snafu::prelude::*;
use tap::prelude::*;
use tauri::Manager;

#[derive(Debug, Snafu)]
pub enum Error {
    DiscordIpcClose { source: crate::core::DiscordIpcError },
    DiscordIpcNew { source: crate::core::DiscordIpcError },
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
    const DISCORD_APPLICATION_ID: &str = "1053777655020912710";
    const TICK_RATE: u64 = 10;

    pub fn new(app: tauri::AppHandle) -> Result<Self, Error> {
        let discord_client = discord_ipc::DiscordIpcClient::new(Self::DISCORD_APPLICATION_ID)
            .map_err(Into::into)
            .context(DiscordIpcNewSnafu)?;
        let discord_presence = None;
        let xbox_presence = None;
        Ok(Self {
            app,
            discord_client,
            discord_presence,
            xbox_presence,
        })
    }

    pub fn start(mut self) -> tauri::async_runtime::JoinHandle<Result<(), Error>> {
        tauri::async_runtime::spawn(async move { self.run().await })
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
            xbox::authorize(&self.app, reauthorize)
                .await
                .context(XboxAuthorizeSnafu)?;
        }
        if let Some(xsts) = &*model.session.xbox.read().await {
            self.xbox_presence = xbox::presence(xsts)
                .await
                .context(XboxPresenceSnafu)?
                .conv::<Option<_>>();
            self.process_xbox_presence().await?;
            // if let Some(xbox_presence) = self.noteworthy_presence()? {
            // }
            // if let Some(presence) = self.noteworthy_presence()
        }

        Ok(())
    }

    async fn process_xbox_presence(&mut self) -> Result<(), Error> {
        if let Some(xbox_presence) = &self.xbox_presence {
            self.discord_presence = discord::Presence::from_xbox(&xbox_presence)
                .await
                .context(DiscordPresenceFromXboxSnafu)?;
        }
        self.refresh_discord_presence()?;
        Ok(())
    }

    fn refresh_discord_presence(&mut self) -> Result<(), Error> {
        use discord_rich_presence::DiscordIpc;
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
                .set_activity(activity)
                .map_err(Into::into)
                .context(DiscordIpcSetActivitySnafu)?;
        } else {
            self.discord_client
                .close()
                .map_err(Into::into)
                .context(DiscordIpcCloseSnafu)?;
        }
        Ok(())
    }
}
