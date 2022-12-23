#[tauri::command]
pub async fn service_playstation_authorization_flow(app: tauri::AppHandle, reauthorize: bool) -> Result<(), String> {
    crate::service::playstation::authorization_flow(&app, reauthorize)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn service_twitch_authorization_flow(app: tauri::AppHandle, reauthorize: bool) -> Result<(), String> {
    crate::service::twitch::authorization_flow(&app, reauthorize)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn service_xbox_authorization_flow(app: tauri::AppHandle, reauthorize: bool) -> Result<(), String> {
    crate::service::xbox::authorize(&app, reauthorize)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn nintendo_auth_ready(app: tauri::AppHandle) -> Result<(), String> {
    use tauri::Manager;
    app.state::<crate::app::Model>()
        .notifiers
        .nintendo_auth_ready
        .notify_waiters();
    Ok(())
}

#[tauri::command]
pub async fn playstation_auth_ready(app: tauri::AppHandle) -> Result<(), String> {
    use tauri::Manager;
    app.state::<crate::app::Model>()
        .notifiers
        .playstation_auth_ready
        .notify_waiters();
    Ok(())
}

#[tauri::command]
pub async fn steam_auth_ready(app: tauri::AppHandle) -> Result<(), String> {
    use tauri::Manager;
    app.state::<crate::app::Model>()
        .notifiers
        .steam_auth_ready
        .notify_waiters();
    Ok(())
}

#[tauri::command]
pub async fn xbox_auth_ready(app: tauri::AppHandle) -> Result<(), String> {
    use tauri::Manager;
    app.state::<crate::app::Model>()
        .notifiers
        .xbox_auth_ready
        .notify_waiters();
    Ok(())
}
