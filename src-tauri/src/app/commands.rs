use serde::{Deserialize, Serialize};
use snafu::prelude::*;
use tauri::Manager;

#[derive(Debug, Snafu)]
pub enum Error {
    AppGetState,
    Config { source: crate::app::model::config::Error },
}

#[tauri::command]
pub async fn init_app(app: tauri::AppHandle) -> Result<(), String> {
    async fn inner(app: tauri::AppHandle) -> Result<(), self::Error> {
        let model = app.try_state::<crate::app::Model>().context(AppGetStateSnafu)?;
        let mut config = model.config.write().await;
        *config = crate::app::model::Config::load().await.context(ConfigSnafu)?;
        Ok(())
    }
    inner(app).await.map_err(|err| err.to_string())
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    steam_user_id: String,
    steam_user_key: String,
}

#[tauri::command]
pub async fn get_settings(app: tauri::AppHandle) -> Result<self::Settings, String> {
    async fn inner(app: tauri::AppHandle) -> Result<self::Settings, self::Error> {
        let model = app.try_state::<crate::app::Model>().context(AppGetStateSnafu)?;
        let config = model.config.read().await;
        Ok(Settings {
            steam_user_id: config.steam_user_id.clone().unwrap_or_default(),
            steam_user_key: config.steam_user_key.clone().unwrap_or_default(),
        })
    }
    inner(app).await.map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn set_settings(app: tauri::AppHandle, payload: Settings) -> Result<(), String> {
    async fn inner(app: tauri::AppHandle, payload: Settings) -> Result<(), self::Error> {
        let model = app.try_state::<crate::app::Model>().context(AppGetStateSnafu)?;
        let mut config = model.config.write().await;
        config.steam_user_id = Some(payload.steam_user_id);
        config.steam_user_key = Some(payload.steam_user_key);
        config.save().await.context(ConfigSnafu)?;
        Ok(())
    }
    inner(app, payload).await.map_err(|err| err.to_string())
}
