use crate::{core::DiscordIpcErrorChain, service::xbox};
use discord_ipc::DiscordIpc;
use discord_rich_presence as discord_ipc;
use futures::future::BoxFuture;
use snafu::prelude::*;
use tauri::Manager;

#[derive(Debug, Snafu)]
pub enum Error {
    DiscordIpcClose {
        #[snafu(source(from(Box<dyn std::error::Error>, DiscordIpcErrorChain::from)))]
        source: DiscordIpcErrorChain,
    },
    DiscordIpcConnect {
        #[snafu(source(from(Box<dyn std::error::Error>, DiscordIpcErrorChain::from)))]
        source: DiscordIpcErrorChain,
    },
    DiscordIpcNew {
        #[snafu(source(from(Box<dyn std::error::Error>, DiscordIpcErrorChain::from)))]
        source: DiscordIpcErrorChain,
    },
    DiscordIpcReconnect {
        #[snafu(source(from(Box<dyn std::error::Error>, DiscordIpcErrorChain::from)))]
        source: DiscordIpcErrorChain,
    },
    DiscordIpcSetActivity {
        #[snafu(source(from(Box<dyn std::error::Error>, DiscordIpcErrorChain::from)))]
        source: DiscordIpcErrorChain,
    },
    ModelUpdateGui {
        source: crate::app::model::Error,
    },
    TauriTryState,
    XboxAuthorize {
        source: crate::service::xbox::Error,
    },
    XboxPresence {
        source: crate::service::xbox::Error,
    },
    XboxPresenceIntoDiscordPresence {
        source: crate::service::xbox::Error,
    },
}

pub struct XboxCore {
    app: tauri::AppHandle,
    discord_client: discord_ipc::DiscordIpcClient,
    xbox_presence: Option<xbox::PresenceRecord>,
}

#[cfg(feature = "debug")]
impl std::fmt::Debug for XboxCore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XboxCore")
            .field("app", &self.app)
            .field("xbox_presence", &self.xbox_presence)
            .finish_non_exhaustive()
    }
}

impl XboxCore {
    const DISCORD_APPLICATION_ID: &str = "1056148753528131654";
    const TICK_RATE: u64 = 15;

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    fn new(app: tauri::AppHandle) -> Result<Self, Error> {
        let mut discord_client =
            discord_ipc::DiscordIpcClient::new(Self::DISCORD_APPLICATION_ID).context(DiscordIpcNewSnafu)?;
        discord_client.connect().context(DiscordIpcConnectSnafu)?;
        let xbox_presence = None;
        Ok(Self {
            app,
            discord_client,
            xbox_presence,
        })
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn start(app: &tauri::AppHandle) -> tauri::async_runtime::JoinHandle<Result<(), Error>> {
        let this = Self::new(app.clone());
        tauri::async_runtime::spawn(async move { this?.run().await })
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    fn run(&mut self) -> BoxFuture<Result<(), Error>> {
        Box::pin(async {
            loop {
                tokio::select! {
                    _ = self.exit()?.notified() => break,
                    _ = self.wait() => self.tick().await?,
                }
            }
            Ok(())
        })
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    fn exit(&self) -> Result<&tokio::sync::Notify, Error> {
        let result = &*self
            .app
            .try_state::<crate::app::Model>()
            .context(TauriTryStateSnafu)?
            .inner()
            .notifiers
            .exit;
        Ok(result)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    async fn wait(&self) {
        use tokio::time;
        time::sleep(time::Duration::from_secs(Self::TICK_RATE)).await
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    async fn tick(&mut self) -> Result<(), Error> {
        let app = self.app.clone();
        let model = app.try_state::<crate::app::Model>().context(TauriTryStateSnafu)?;
        if !model.config.read().await.services.xbox.enabled {
            return Ok(());
        }
        if model.session.xbox.read().await.is_none() {
            let reauthorize = false;
            let authorize = xbox::authorize(&self.app, reauthorize).await;
            authorize.context(XboxAuthorizeSnafu)?;
        }
        if let Some(xsts) = &*model.session.xbox.read().await {
            let xbox_presence = xbox::presence(xsts).await.context(XboxPresenceSnafu)?;
            if self.xbox_presence.as_ref().map(|xp| xp.relevant_name()).flatten() == xbox_presence.relevant_name() {
                return Ok(());
            }
            self.xbox_presence = Some(xbox_presence);
            self.process_xbox_presence().await?;
        }
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    async fn process_xbox_presence(&mut self) -> Result<(), Error> {
        if let Some(xbox_presence) = &self.xbox_presence {
            let presence = xbox_presence
                .into_discord_presence()
                .await
                .context(XboxPresenceIntoDiscordPresenceSnafu)?;
            let model = self.app.state::<crate::app::Model>();
            model
                .update_gui(|gui| {
                    if let Some(data) = &mut gui.services.xbox.data {
                        println!("process_xbox_presence: subscription");
                        data.presence = presence;
                    }
                })
                .await
                .context(ModelUpdateGuiSnafu)?;
            model.notifiers.gui.notify_waiters();
            self.refresh_discord_activity().await?;
        }
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    async fn refresh_discord_activity(&mut self) -> Result<(), Error> {
        if let Some(discord_presence) = self
            .app
            .try_state::<crate::app::Model>()
            .context(TauriTryStateSnafu)?
            .gui
            .read()
            .await
            .services
            .xbox
            .data
            .iter()
            .flat_map(|data| &data.presence)
            .next()
        {
            use discord_ipc::activity::{Activity, Assets, Button, Timestamps};
            let details = &discord_presence.details;
            let state = &discord_presence.state;
            let assets = Assets::new()
                .large_image(&discord_presence.assets_large_image)
                .large_text(&discord_presence.assets_large_text);
            // .small_image(&discord_presence.assets_small_image)
            // .small_text(&discord_presence.assets_small_text);
            let timestamps = Timestamps::new().start(discord_presence.time_start.unix_timestamp());
            let buttons = std::iter::empty()
                .chain(&discord_presence.button_store)
                .chain(&discord_presence.button_twitch)
                .map(|(label, url)| Button::new(label, url.as_str()))
                .collect();
            let activity = Activity::new()
                .details(details)
                .state(state)
                .assets(assets)
                .timestamps(timestamps)
                .buttons(buttons);

            self.discord_client.reconnect().context(DiscordIpcReconnectSnafu)?;
            self.discord_client
                .set_activity(activity)
                .context(DiscordIpcSetActivitySnafu)?;
            println!("presence updated");
        } else {
            self.discord_client.close().context(DiscordIpcCloseSnafu)?;
        }
        Ok(())
    }
}
