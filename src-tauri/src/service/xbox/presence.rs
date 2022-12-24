use super::{Error, ReqwestRequestSendSnafu, ReqwestResponseJsonSnafu, SerdeJsonFromValueSnafu, UrlParseSnafu};
use serde::Deserialize;
use snafu::prelude::*;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceRecord {
    pub state: String,
    pub devices: Vec<DeviceRecord>,
    pub last_seen: Option<LastSeenRecord>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityRecord {
    pub rich_presence: String,
    pub media: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceRecord {
    #[serde(rename = "type")]
    pub r#type: String,
    pub titles: Vec<TitleRecord>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastSeenRecord {
    pub device_type: String,
    pub title_id: String,
    pub title_name: String,
    #[serde(with = "time::serde::iso8601")]
    pub timestamp: time::OffsetDateTime,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TitleRecord {
    pub id: String,
    pub name: String,
    pub activity: Option<ActivityRecord>,
    #[serde(with = "time::serde::iso8601")]
    pub last_modified: time::OffsetDateTime,
    pub placement: String,
    pub state: String,
}

fn endpoint() -> Result<url::Url, Error> {
    url::Url::parse("https://userpresence.xboxlive.com/users/me").context(UrlParseSnafu)
}

pub async fn request(xsts: &super::XstsToken) -> Result<PresenceRecord, Error> {
    let url = endpoint()?;
    let user_hash = &xsts.display_claims.xui.uhs;
    let token = &xsts.token;

    let value = reqwest::Client::new()
        .get(url)
        .header("Accept", "application/json")
        .header("Accept-Language", "en-US")
        .header("Authorization", format!("XBL3.0 x={};{}", user_hash, token))
        .header("x-xbl-contract-version", "3")
        .send()
        .await
        .context(ReqwestRequestSendSnafu)?
        .json::<serde_json::Value>()
        .await
        .context(ReqwestResponseJsonSnafu)?;
    println!("value: {:#?}", value);
    let presence_record = serde_json::from_value(value).context(SerdeJsonFromValueSnafu);
    println!("presence_record: {:#?}", presence_record);
    Ok(presence_record.unwrap())
}
