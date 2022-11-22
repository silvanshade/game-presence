#[tauri::command]
pub async fn build_info() -> Result<crate::app::data::BuildInfo, String> {
    crate::app::data::BuildInfo::collect().map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn api_twitch_authorization_window_open<R: tauri::Runtime>(app: tauri::AppHandle<R>) -> Result<(), String> {
    crate::api::twitch::window::authorization::open(&app)
        .await
        .map_err(|err| err.to_string())
}
