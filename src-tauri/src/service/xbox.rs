use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    ReqwestSend { source: reqwest::Error },
    ReqwestResponseJson { source: reqwest::Error },
    SerdeJsonFromValue { source: serde_json::Error },
    UrlDropResizeParams,
    UrlParse { source: url::ParseError },
}

pub mod api {
    pub mod autosuggest {
        use super::super::{
            Error,
            ReqwestResponseJsonSnafu,
            ReqwestSendSnafu,
            UrlDropResizeParamsSnafu,
            UrlParseSnafu,
        };
        use serde::Deserialize;
        use snafu::prelude::*;
        use tap::prelude::*;

        pub const ENDPOINT: &str = "https://www.microsoft.com/msstoreapiprod/api/autosuggest";

        fn url(query: &str) -> Result<url::Url, Error> {
            use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
            let encoded_query = utf8_percent_encode(query, NON_ALPHANUMERIC).to_string();
            let params = [
                ("market", "en-us"),
                ("sources", "DCatAll-Products"),
                ("query", encoded_query.as_str()),
            ];
            url::Url::parse_with_params(self::ENDPOINT, params).context(UrlParseSnafu)
        }

        pub async fn request(query: &str) -> Result<Option<self::Suggest>, Error> {
            let client = reqwest::Client::new();
            let request = client.get(self::url(query)?);
            let response = request.send().await.context(ReqwestSendSnafu)?;
            response
                .json::<self::Response>()
                .await
                .context(ReqwestResponseJsonSnafu)?
                .result_sets
                .into_iter()
                .flat_map(|result_set| result_set.suggests)
                .filter(|suggest| suggest.source == "Game")
                .map(|suggest| {
                    use triple_accel::levenshtein::levenshtein_exp;
                    let distance = levenshtein_exp(query.as_bytes(), suggest.title.as_bytes());
                    (distance, suggest)
                })
                .collect::<Vec<_>>()
                .tap_mut(|results| results.sort_by(|lhs, rhs| lhs.0.cmp(&rhs.0)))
                .into_iter()
                .map(|result| result.1)
                .next()
                .pipe(Ok)
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "PascalCase")]
        pub struct Response {
            pub result_sets: Vec<ResultSet>,
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "PascalCase")]
        pub struct ResultSet {
            pub suggests: Vec<Suggest>,
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "PascalCase")]
        pub struct Suggest {
            pub source: String,
            pub title: String,
            pub url: String,
            pub image_url: String,
        }

        impl Suggest {
            pub fn image_url(&self) -> Result<url::Url, Error> {
                let protocol = "https";
                let url = self.image_url.split('?').next().context(UrlDropResizeParamsSnafu)?;
                url::Url::parse(&format!("{}:{}", protocol, url)).context(UrlParseSnafu)
            }

            pub fn store_url(&self) -> Result<url::Url, Error> {
                let protocol = "https";
                let url = &self.url;
                url::Url::parse(&format!("{}:{}", protocol, url)).context(UrlParseSnafu)
            }
        }
    }

    pub mod summary {
        use super::super::{Error, ReqwestResponseJsonSnafu, ReqwestSendSnafu, SerdeJsonFromValueSnafu, UrlParseSnafu};
        use serde::Deserialize;
        use snafu::prelude::*;
        use tap::prelude::*;

        pub const ENDPOINT: &str = "https://xbl.io/api/v2/player/summary";

        fn url() -> Result<url::Url, Error> {
            url::Url::parse(self::ENDPOINT).context(UrlParseSnafu)
        }

        pub async fn request(api_key: &str) -> Result<Option<self::Person>, Error> {
            let client = reqwest::Client::new();
            let request = client.get(self::url()?).header("x-authorization", api_key);
            let response = request.send().await.context(ReqwestSendSnafu)?;
            response
                .json::<self::Response>()
                .await
                .context(ReqwestResponseJsonSnafu)?
                // .tap(|v| println!("value: {:#?}", v))
                // .pipe(serde_json::from_value::<self::Response>)
                // .context(SerdeJsonFromValueSnafu)?
                .people
                .pop()
                .pipe(Ok)
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Response {
            pub people: Vec<Person>,
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Person {
            pub display_pic_raw: String,
            pub gamertag: String,
            pub presence_state: String,
            pub presence_text: String,
        }
    }

    pub async fn autosuggest(query: &str) -> Result<Option<self::autosuggest::Suggest>, super::Error> {
        self::autosuggest::request(query).await
    }

    pub async fn summary(api_key: &str) -> Result<Option<self::summary::Person>, super::Error> {
        self::summary::request(api_key).await
    }
}
