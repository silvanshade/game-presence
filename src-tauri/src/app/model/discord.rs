use snafu::prelude::*;
use std::sync::{atomic::AtomicBool, Arc};
use tauri::async_runtime::RwLock;

#[derive(Debug)]
pub struct DiscordError(String);

impl From<Box<dyn std::error::Error>> for DiscordError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        DiscordError(error.to_string())
    }
}

impl std::fmt::Display for DiscordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for DiscordError {
}

#[derive(Debug, Snafu)]
pub enum Error {
    DiscordActivity { source: DiscordError },
    DiscordConnect { source: DiscordError },
    DiscordCreation { source: DiscordError },
}

pub struct Discord {
    pub config: Arc<RwLock<crate::app::model::Config>>,
    pub connected: AtomicBool,
    pub ipc: discord::DiscordIpcClient,
}

impl Discord {
    pub fn new(config: Arc<RwLock<crate::app::model::Config>>) -> Result<Self, self::Error> {
        let client_id = crate::api::discord::CLIENT_ID;
        let ipc = discord::DiscordIpcClient::new(client_id)
            .map_err(Into::into)
            .context(DiscordCreationSnafu)?;
        let ipc = ipc;
        let connected = AtomicBool::from(false);
        Ok(Self { config, connected, ipc })
    }

    pub fn connect(&mut self) -> Result<(), self::Error> {
        use discord::DiscordIpc;
        self.ipc.connect().map_err(Into::into).context(DiscordConnectSnafu)?;
        #[cfg(feature = "debug")]
        tracing::info!("connected");
        self.connected.store(true, std::sync::atomic::Ordering::Release);
        Ok(())
    }

    pub fn update_presence(&mut self, data: Option<crate::api::steam::PlayerSummaries>) -> Result<(), self::Error> {
        use discord::DiscordIpc;

        if !self.connected.load(std::sync::atomic::Ordering::Acquire) {
            #[cfg(feature = "debug")]
            tracing::info!("skipping presence update; ipc not connected");
            return Ok(());
        }

        let mut state = String::from("Not currently in-game");
        let mut large_image = String::from("steam-presence-logo");
        let mut small_image = String::from("");

        if let Some(player_summaries) = data {
            if let Some(player) = player_summaries.response.players.get(0) {
                if let Some(gameextrainfo) = &player.gameextrainfo {
                    state = format!("Playing {}", gameextrainfo);
                }
                if let Some(gameid) = &player.gameid {
                    large_image = format!(
                        "https://cdn.cloudflare.steamstatic.com/steam/apps/{}/library_600x900.jpg",
                        gameid
                    );
                }
                if let Some(avatarfull) = &player.avatarfull {
                    small_image = avatarfull.clone();
                }
            }
        }

        let assets = discord::activity::Assets::new()
            .large_image(&large_image)
            .small_image(&small_image);
        let activity = discord::activity::Activity::new().state(&state).assets(assets);

        self.ipc
            .set_activity(activity)
            .map_err(Into::into)
            .context(DiscordActivitySnafu)?;

        #[cfg(feature = "debug")]
        tracing::info!("activity set");

        Ok(())
    }
}
