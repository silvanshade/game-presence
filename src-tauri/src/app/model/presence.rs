use super::{Error, StdTimeDurationSinceSnafu, UrlParseSnafu};
use snafu::prelude::*;
use tap::prelude::*;

#[derive(Debug, Eq, PartialEq)]
pub struct Presence {
    pub details: String,
    pub state: String,
    pub assets_large_image: String,
    pub assets_large_text: String,
    pub assets_small_image: String,
    pub assets_small_text: String,
    pub time_start: u64,
    pub time_end: Option<u64>,
    pub button_store: Option<(String, url::Url)>,
    pub button_twitch: Option<(String, url::Url)>,
}

impl Presence {
    pub fn timestamp() -> Result<u64, Error> {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .context(StdTimeDurationSinceSnafu)?
            .as_secs()
            .pipe(Ok)
    }

    pub fn twitch_url(title: &str) -> Result<url::Url, Error> {
        use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
        let encoded_title = utf8_percent_encode(title, NON_ALPHANUMERIC).to_string();
        let base = "https://www.twitch.tv/directory/game";
        url::Url::parse(&format!("{}/{}", base, encoded_title)).context(UrlParseSnafu)
    }
}
