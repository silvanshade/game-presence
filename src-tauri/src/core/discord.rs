use crate::service::xbox;
use snafu::prelude::*;
use tap::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    StdTimeDurationSince { source: std::time::SystemTimeError },
    UrlParse { source: url::ParseError },
    XboxAutosuggest { source: crate::service::xbox::Error },
    XboxPresenceEmptyName,
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
        println!("from_xbox: 0");
        if let Some(name) = xbox_presence
            .tap(|p| println!("xbox_presence: {:#?}", p))
            .devices
            .iter()
            .flat_map(|device| &device.titles)
            .find_map(|title| {
                if title.name != "Online" {
                    Some(title.name.as_str())
                } else {
                    None
                }
            })
        {
            if name == "" {
                return Err(Error::XboxPresenceEmptyName);
            }
            println!("from_xbox::name: {:#?}", name);
            println!("from_xbox: 1");
            let autosuggest_result = xbox::autosuggest(name).await.context(XboxAutosuggestSnafu);
            println!("autosuggest: {:#?}", autosuggest_result);
            if let Some(suggest) = autosuggest_result? {
                println!("from_xbox: 2");
                let details = name.into();
                println!("from_xbox: 3");
                let state = None;
                println!("from_xbox: 4");
                let assets_large_image = suggest.image_url().context(XboxSuggestImageUrlSnafu)?.into();
                println!("from_xbox: 5");
                let assets_large_text = name.into();
                println!("from_xbox: 6");
                let assets_small_image = "small-icon".into();
                println!("from_xbox: 7");
                let assets_small_text = "playing on xbox".into();
                println!("from_xbox: 8");
                let time_start = Self::timestamp()?;
                println!("from_xbox: 9");
                let time_end = None;
                println!("from_xbox: 10");
                let button_store = Some((
                    String::from("xbox.com"),
                    suggest.store_url().context(XboxSuggestStoreUrlSnafu)?,
                ));
                println!("from_xbox: 11");
                let button_twitch = Some((String::from("twitch"), Self::twitch_url(name)?));
                println!("from_xbox: 12");
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
