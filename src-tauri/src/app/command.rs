pub mod api {
    pub mod twitch {
        #[tauri::command]
        pub async fn authorization_flow<R: tauri::Runtime>(app: tauri::AppHandle<R>) -> Result<(), String> {
            crate::api::twitch::authorization_flow(&app)
                .await
                .map_err(|err| err.to_string())
        }
    }
}

#[tauri::command]

pub async fn config_load<R: tauri::Runtime>(app: tauri::AppHandle<R>) -> Result<(), String> {
    use tauri::Manager;
    let data = crate::app::model::Config::load().await.map_err(|err| err.to_string())?;
    let state = app.state::<crate::app::model::State>();
    state.update_with_broadcast(data).await.map_err(|err| err.to_string())?;
    Ok(())
}
