use serde::Deserialize;
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    ReqwestGet { source: reqwest::Error },
    ReqwestResponseJson { source: reqwest::Error },
    SerdeJsonFromValue { source: serde_json::Error },
    UrlDropResizeParams,
    UrlParse { source: url::ParseError },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct XboxStoreAutoSuggest {
    pub result_sets: Vec<XboxStoreAutoSuggestResultSet>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct XboxStoreAutoSuggestResultSet {
    pub suggests: Vec<XboxStoreSuggestResult>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct XboxStoreSuggestResult {
    pub source: String,
    pub title: String,
    pub url: String,
    pub image_url: String,
}

impl XboxStoreSuggestResult {
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

const ENDPOINT_AUTOSUGGEST: &str = "https://www.microsoft.com/msstoreapiprod/api/autosuggest";

fn endpoint_autosuggest_url(query: &str) -> Result<url::Url, Error> {
    use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
    let encoded_query = utf8_percent_encode(query, NON_ALPHANUMERIC).to_string();
    let params = [
        ("market", "en-us"),
        ("sources", "DCatAll-Products"),
        ("query", encoded_query.as_str()),
    ];
    url::Url::parse_with_params(ENDPOINT_AUTOSUGGEST, params).context(UrlParseSnafu)
}

pub async fn request_autosuggest(query: &str) -> Result<Option<XboxStoreSuggestResult>, Error> {
    use tap::Tap;
    let url = endpoint_autosuggest_url(query)?;
    let data = reqwest::get(url).await.context(ReqwestGetSnafu)?;
    let json = data
        .json::<serde_json::Value>()
        .await
        .context(ReqwestResponseJsonSnafu)?;
    let auto_suggest = serde_json::from_value::<XboxStoreAutoSuggest>(json).context(SerdeJsonFromValueSnafu)?;
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
