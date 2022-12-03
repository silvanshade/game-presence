#[tauri::command]
pub async fn build_info() -> Result<crate::app::data::BuildInfo, String> {
    crate::app::data::BuildInfo::collect().map_err(|err| err.to_string())
}

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
