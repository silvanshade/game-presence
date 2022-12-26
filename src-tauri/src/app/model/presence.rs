use super::{Error, UrlParseSnafu};
use serde::{Deserialize, Serialize};
use snafu::prelude::*;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Presence {
    pub details: String,
    pub state: String,
    pub assets_large_image: String,
    pub assets_large_text: String,
    pub assets_small_image: String,
    pub assets_small_text: String,
    #[serde(with = "time::serde::iso8601")]
    pub time_start: time::OffsetDateTime,
    pub button_store: Option<(String, url::Url)>,
    pub button_twitch: Option<(String, url::Url)>,
}

impl Presence {
    pub fn twitch_url(title: &str) -> Result<url::Url, Error> {
        use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
        let encoded_title = utf8_percent_encode(title, NON_ALPHANUMERIC).to_string();
        let base = "https://www.twitch.tv/directory/game";
        url::Url::parse(&format!("{}/{}", base, encoded_title)).context(UrlParseSnafu)
    }
}
