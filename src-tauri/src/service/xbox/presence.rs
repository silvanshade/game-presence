use super::Error;
use serde::Deserialize;
use snafu::prelude::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceRecord {
    pub state: String,
    pub devices: Vec<DeviceRecord>,
    pub last_seen: LastSeenRecord,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityRecord {
    pub rich_presence: String,
    pub media: Option<serde_json::Value>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceRecord {
    #[serde(rename = "type")]
    pub r#type: String,
    pub titles: Vec<TitleRecord>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastSeenRecord {
    pub device_type: String,
    pub title_id: u32,
    pub title_name: String,
    #[serde(with = "time::serde::iso8601")]
    pub timestamp: time::OffsetDateTime,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TitleRecord {
    pub id: u32,
    pub name: String,
    pub activity: Option<ActivityRecord>,
    #[serde(with = "time::serde::iso8601")]
    pub last_modified: time::OffsetDateTime,
    pub placement: String,
    pub state: String,
}

pub async fn request(query: &str) -> Result<Option<PresenceRecord>, Error> {
    Ok(None)
}
