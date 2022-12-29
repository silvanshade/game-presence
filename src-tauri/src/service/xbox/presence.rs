use super::{
    Error,
    ModelPresenceTwitchUrlSnafu,
    ReqwestRequestSendSnafu,
    ReqwestResponseJsonSnafu,
    SerdeJsonFromValueSnafu,
    UrlParseSnafu,
};
use serde::Deserialize;
use snafu::prelude::*;
use tap::prelude::*;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceRecord {
    pub state: String,
    pub devices: Option<Vec<DeviceRecord>>,
    pub last_seen: Option<LastSeenRecord>,
}

impl PresenceRecord {
    pub fn relevant_name(&self) -> Option<&str> {
        self.devices
            .iter()
            .map(|devices| {
                devices.iter().flat_map(|device| &device.titles).find_map(|title| {
                    if !["Home", "Online"].contains(&title.name.as_str()) {
                        Some(title.name.as_str())
                    } else {
                        None
                    }
                })
            })
            .next()
            .flatten()
    }

    pub async fn into_discord_presence(&self) -> Result<Option<crate::app::model::Presence>, Error> {
        let name = self.relevant_name();
        if let Some(name) = name {
            if name == "" {
                println!("xbox_presence: empty name; skipping");
                return Ok(None);
            }
            let autosuggest = super::autosuggest(name).await;
            if let Some(suggest) = autosuggest? {
                let details = name.conv::<String>();
                let state = "playing on pc/xbox".conv::<String>();
                let assets_large_image = suggest.image_url()?.conv::<String>();
                let assets_large_text = details.clone();
                let assets_small_image = "small-icon".conv::<String>();
                let assets_small_text = state.clone();
                let time_start = time::OffsetDateTime::now_utc();
                let button_store = Some(("xbox.com".conv::<String>(), suggest.store_url()?));
                let button_twitch = Some((
                    "twitch".conv::<String>(),
                    crate::app::model::Presence::twitch_url(name).context(ModelPresenceTwitchUrlSnafu)?,
                ));
                return Ok(Some(crate::app::model::Presence::new(
                    details,
                    state,
                    assets_large_image,
                    assets_large_text,
                    assets_small_image,
                    assets_small_text,
                    time_start,
                    button_store,
                    button_twitch,
                )));
            }
        }
        Ok(None)
    }
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
    reqwest::Client::new()
        .get(url)
        .header("Accept", "application/json")
        .header("Accept-Language", "en-US")
        .header("Authorization", format!("XBL3.0 x={};{}", user_hash, token))
        .header("x-xbl-contract-version", "3")
        .send()
        .await
        .context(ReqwestRequestSendSnafu)?
        .tap(|request| println!("presence: {:#?}", request.status()))
        .json::<serde_json::Value>()
        .await
        .context(ReqwestResponseJsonSnafu)?
        .pipe(serde_json::from_value::<PresenceRecord>)
        .context(SerdeJsonFromValueSnafu)?
        // .tap(|value| println!("presence_record: {:#?}", value))
        .pipe(Ok)
}
