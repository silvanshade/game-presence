use snafu::prelude::*;

#[derive(Debug, Snafu)]
enum Error {
    TauriTryState,
}

#[cfg_attr(feature = "tracing", tracing::instrument)]
#[tauri::command]
pub async fn service_playstation_authorization_flow(_app: tauri::AppHandle, _reauthorize: bool) -> Result<(), String> {
    // crate::service::playstation::authorization_flow(&app, reauthorize)
    //     .await
    //     .map_err(|err| err.to_string())?;
    Ok(())
}

#[cfg_attr(feature = "tracing", tracing::instrument)]
#[tauri::command]
pub async fn service_twitch_authorization_flow(app: tauri::AppHandle, reauthorize: bool) -> Result<(), String> {
    crate::service::twitch::authorization_flow(&app, reauthorize)
        .await
        .map_err(|err| err.to_string())
}

#[cfg_attr(feature = "tracing", tracing::instrument)]
#[tauri::command]
pub async fn service_xbox_authorization_flow(app: tauri::AppHandle, reauthorize: bool) -> Result<(), String> {
    crate::service::xbox::authorize(&app, reauthorize)
        .await
        .map_err(|err| err.to_string())
}

#[cfg_attr(feature = "tracing", tracing::instrument)]
#[tauri::command]
pub async fn nintendo_auth_ready(app: tauri::AppHandle) -> Result<(), String> {
    use tauri::Manager;
    app.try_state::<crate::app::Model>()
        .context(TauriTryStateSnafu)
        .map_err(|err| err.to_string())?
        .notifiers
        .nintendo_auth_ready
        .notify_waiters();
    Ok(())
}

#[cfg_attr(feature = "tracing", tracing::instrument)]
#[tauri::command]
pub async fn playstation_auth_ready(app: tauri::AppHandle) -> Result<(), String> {
    use tauri::Manager;
    app.try_state::<crate::app::Model>()
        .context(TauriTryStateSnafu)
        .map_err(|err| err.to_string())?
        .notifiers
        .playstation_auth_ready
        .notify_waiters();
    Ok(())
}

#[cfg_attr(feature = "tracing", tracing::instrument)]
#[tauri::command]
pub async fn steam_auth_ready(app: tauri::AppHandle) -> Result<(), String> {
    use tauri::Manager;
    app.try_state::<crate::app::Model>()
        .context(TauriTryStateSnafu)
        .map_err(|err| err.to_string())?
        .notifiers
        .steam_auth_ready
        .notify_waiters();
    Ok(())
}

#[cfg_attr(feature = "tracing", tracing::instrument)]
#[tauri::command]
pub async fn xbox_auth_ready(app: tauri::AppHandle) -> Result<(), String> {
    use tauri::Manager;
    app.try_state::<crate::app::Model>()
        .context(TauriTryStateSnafu)
        .map_err(|err| err.to_string())?
        .notifiers
        .xbox_auth_ready
        .notify_waiters();
    Ok(())
}
