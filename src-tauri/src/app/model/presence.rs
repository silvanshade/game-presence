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
    pub hash: String,
}

impl Presence {
    pub fn new(
        details: String,
        state: String,
        assets_large_image: String,
        assets_large_text: String,
        assets_small_image: String,
        assets_small_text: String,
        time_start: time::OffsetDateTime,
        button_store: Option<(String, url::Url)>,
        button_twitch: Option<(String, url::Url)>,
    ) -> Self {
        use std::{
            collections::hash_map::DefaultHasher,
            hash::{Hash, Hasher},
        };

        let hasher = &mut DefaultHasher::new();
        details.hash(hasher);
        state.hash(hasher);
        assets_large_image.hash(hasher);
        assets_large_text.hash(hasher);
        assets_small_image.hash(hasher);
        assets_small_text.hash(hasher);
        time_start.hash(hasher);
        button_store.hash(hasher);
        button_twitch.hash(hasher);
        let hash = hasher.finish().to_string();

        Self {
            details,
            state,
            assets_large_image,
            assets_large_text,
            assets_small_image,
            assets_small_text,
            time_start,
            button_store,
            button_twitch,
            hash,
        }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn twitch_url(title: &str) -> Result<url::Url, Error> {
        use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
        let encoded_title = utf8_percent_encode(title, NON_ALPHANUMERIC).to_string();
        let base = "https://www.twitch.tv/directory/game";
        url::Url::parse(&format!("{}/{}", base, encoded_title)).context(UrlParseSnafu)
    }
}
