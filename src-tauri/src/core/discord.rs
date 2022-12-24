use crate::service::xbox;
use snafu::prelude::*;
use tap::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    StdTimeDurationSince { source: std::time::SystemTimeError },
    UrlParse { source: url::ParseError },
    XboxAutosuggest { source: crate::service::xbox::Error },
    XboxSuggestImageUrl { source: crate::service::xbox::Error },
    XboxSuggestStoreUrl { source: crate::service::xbox::Error },
}

#[derive(Debug, Eq, PartialEq)]
pub struct Presence {
    pub details: String,
    pub state: Option<String>,
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
    fn timestamp() -> Result<u64, Error> {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .context(StdTimeDurationSinceSnafu)?
            .as_secs()
            .pipe(Ok)
    }

    fn twitch_url(title: &str) -> Result<url::Url, Error> {
        use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
        let encoded_title = utf8_percent_encode(title, NON_ALPHANUMERIC).to_string();
        let base = "https://www.twitch.tv/directory/game";
        url::Url::parse(&format!("{}/{}", base, encoded_title)).context(UrlParseSnafu)
    }

    pub async fn from_xbox(xbox_presence: &xbox::PresenceRecord) -> Result<Option<Self>, Error> {
        if let Some(name) = xbox_presence
            // .tap(|p| println!("xbox_presence: {:#?}", p))
            .devices
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
        {
            if name == "" {
                println!("xbox_presence: empty name; skipping");
                return Ok(None);
            }
            let autosuggest_result = xbox::autosuggest(name).await.context(XboxAutosuggestSnafu);
            if let Some(suggest) = autosuggest_result? {
                let details = name.into();
                let state = None;
                let assets_large_image = suggest.image_url().context(XboxSuggestImageUrlSnafu)?.into();
                let assets_large_text = name.into();
                let assets_small_image = "small-icon".into();
                let assets_small_text = "playing on xbox".into();
                let time_start = Self::timestamp()?;
                let time_end = None;
                let button_store = Some((
                    String::from("xbox.com"),
                    suggest.store_url().context(XboxSuggestStoreUrlSnafu)?,
                ));
                let button_twitch = Some((String::from("twitch"), Self::twitch_url(name)?));
                return Ok(Some(Self {
                    details,
                    state,
                    assets_large_image,
                    assets_large_text,
                    assets_small_image,
                    assets_small_text,
                    time_start,
                    time_end,
                    button_store,
                    button_twitch,
                }));
            }
        }
        Ok(None)
    }

    pub fn differs_modulo_time(lhs: &Option<Self>, rhs: &Option<Self>) -> bool {
        let mut result = false;
        match (lhs, rhs) {
            (None, None) => {},
            (Some(lhs), Some(rhs)) => {
                result |= lhs.details != rhs.details;
                result |= lhs.state != rhs.state;
                result |= lhs.assets_large_image != rhs.assets_large_image;
                result |= lhs.assets_large_text != rhs.assets_large_text;
                result |= lhs.assets_small_image != rhs.assets_small_image;
                result |= lhs.assets_small_text != rhs.assets_small_text;
                result |= lhs.button_store != rhs.button_store;
                result |= lhs.button_twitch != rhs.button_twitch;
            },
            _ => {
                result |= true;
            },
        }
        result
    }
}
