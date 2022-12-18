use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    ReqwestGet { source: reqwest::Error },
    ReqwestResponseJson { source: reqwest::Error },
    SerdeJsonFromValue { source: serde_json::Error },
    UrlDropResizeParams,
    UrlParse { source: url::ParseError },
}

pub mod api {
    pub mod autosuggest {
        use super::super::{
            Error,
            ReqwestGetSnafu,
            ReqwestResponseJsonSnafu,
            SerdeJsonFromValueSnafu,
            UrlDropResizeParamsSnafu,
            UrlParseSnafu,
        };
        use serde::Deserialize;
        use snafu::prelude::*;

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
            use tap::Tap;
            let url = self::url(query)?;
            let data = reqwest::get(url).await.context(ReqwestGetSnafu)?;
            let json = data
                .json::<serde_json::Value>()
                .await
                .context(ReqwestResponseJsonSnafu)?;
            let auto_suggest = serde_json::from_value::<self::Response>(json).context(SerdeJsonFromValueSnafu)?;
            let result = auto_suggest
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
                .next();
            Ok(result)
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
}

const ENDPOINT_PRESENCE: &str = "https://xbl.io/api/v2/player/summary";
