#[tauri::command]
pub async fn build_info() -> Result<crate::app::data::BuildInfo, String> {
    crate::app::data::BuildInfo::collect().map_err(|err| err.to_string())
}
