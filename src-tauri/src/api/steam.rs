use serde::Deserialize;
use snafu::prelude::*;
use std::sync::Arc;
use tauri::async_runtime::{Mutex, RwLock};

#[derive(Debug, Snafu)]
pub enum Error {
    DiscordUpdatePresence { source: crate::app::model::discord::Error },
    JsonUndefined,
    SerdeJsonDeserialize { source: serde_json::Error },
    TauriApi { source: tauri::api::Error },
}

#[derive(Debug, Deserialize)]
pub struct PlayerSummaries {
    pub response: get_player_summaries::Response,
}

pub mod get_player_summaries {
    use serde::Deserialize;
    use snafu::prelude::*;
    use tauri::api::http::{ClientBuilder, HttpRequestBuilder};

    #[derive(Debug, Deserialize)]
    pub struct Response {
        pub players: Vec<Player>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Player {
        pub avatarfull: Option<String>,
        pub gameid: Option<String>,
        pub gameextrainfo: Option<String>,
    }

    static METHOD: &str = "GET";
    static URL: &str = "http://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002";

    fn request(steam_user_id: String, steam_user_key: String) -> Result<HttpRequestBuilder, super::Error> {
        let query = [("key".into(), steam_user_key), ("steamids".into(), steam_user_id)]
            .into_iter()
            .collect();
        let request = HttpRequestBuilder::new(METHOD, URL)
            .context(super::TauriApiSnafu)?
            .query(query);
        Ok(request)
    }

    pub async fn fetch(
        steam_user_id: Option<String>,
        steam_user_key: Option<String>,
    ) -> Result<Option<super::PlayerSummaries>, super::Error> {
        if let (Some(steam_user_id), Some(steam_user_key)) = (steam_user_id, steam_user_key) {
            let http_req = self::request(steam_user_id, steam_user_key)?;
            let http_rsp = ClientBuilder::new()
                .build()
                .context(super::TauriApiSnafu)?
                .send(http_req)
                .await
                .context(super::TauriApiSnafu)?;
            let rsp_data = http_rsp.read().await.context(super::TauriApiSnafu)?;
            let rsp_json = rsp_data.data;
            #[cfg(feature = "debug")]
            tracing::info!(?rsp_json);
            let player_summaries = serde_json::from_value(rsp_json).context(super::SerdeJsonDeserializeSnafu)?;
            Ok(Some(player_summaries))
        } else {
            Ok(None)
        }
    }
}

pub async fn poll_loop(
    config: Arc<RwLock<crate::app::model::Config>>,
    discord: Arc<Mutex<crate::app::model::Discord>>,
    status: Arc<RwLock<crate::app::model::steam::Status>>,
) -> Result<(), self::Error> {
    use tokio::time::{sleep, Duration};
    loop {
        if *status.read().await == crate::app::model::steam::Status::Stop {
            break;
        }
        let crate::app::model::Config {
            steam_user_id,
            steam_user_key,
            steam_api_poll_rate_secs,
            ..
        } = config.read().await.clone();
        let data = get_player_summaries::fetch(steam_user_id, steam_user_key).await?;
        #[cfg(feature = "debug")]
        tracing::info!(?data);
        discord
            .lock()
            .await
            .update_presence(data)
            .context(DiscordUpdatePresenceSnafu)?;
        sleep(Duration::from_secs(steam_api_poll_rate_secs)).await;
    }
    Ok(())
}
