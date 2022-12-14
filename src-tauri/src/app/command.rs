#[tauri::command]
pub async fn service_playstation_authorization_flow(
    app: tauri::AppHandle<tauri::Wry>,
    reauthorize: bool,
) -> Result<(), String> {
    crate::service::playstation::authorization_flow(&app, reauthorize)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn service_twitch_authorization_flow(
    app: tauri::AppHandle<tauri::Wry>,
    reauthorize: bool,
) -> Result<(), String> {
    crate::service::twitch::authorization_flow(&app, reauthorize)
        .await
        .map_err(|err| err.to_string())
}
