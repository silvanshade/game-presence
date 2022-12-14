pub mod service {
    #[tauri::command]
    pub async fn authorization_flow_playstation(
        app: tauri::AppHandle<tauri::Wry>,
        reauthorize: bool,
    ) -> Result<(), String> {
        crate::service::playstation::authorization_flow(&app, reauthorize)
            .await
            .map_err(|err| err.to_string())
    }

    #[tauri::command]
    pub async fn authorization_flow_twitch(app: tauri::AppHandle<tauri::Wry>, reauthorize: bool) -> Result<(), String> {
        crate::service::twitch::authorization_flow(&app, reauthorize)
            .await
            .map_err(|err| err.to_string())
    }
}
