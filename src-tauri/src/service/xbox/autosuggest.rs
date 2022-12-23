use super::{
    Error,
    ReqwestRequestGetSnafu,
    ReqwestResponseJsonSnafu,
    SerdeJsonFromValueSnafu,
    UrlDropResizeParamsSnafu,
    UrlParseSnafu,
};
use serde::Deserialize;
use snafu::prelude::*;
use tap::prelude::*;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StoreAutoSuggest {
    pub result_sets: Vec<StoreAutoSuggestResultSet>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StoreAutoSuggestResultSet {
    pub suggests: Vec<StoreSuggestResult>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StoreSuggestResult {
    pub source: String,
    pub title: String,
    pub url: String,
    pub image_url: String,
}

impl StoreSuggestResult {
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

fn endpoint(query: &str) -> Result<url::Url, Error> {
    use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
    let encoded_query = utf8_percent_encode(query, NON_ALPHANUMERIC).to_string();
    let params = [
        ("market", "en-us"),
        ("sources", "DCatAll-Products"),
        ("query", encoded_query.as_str()),
    ];
    url::Url::parse_with_params(ENDPOINT_AUTOSUGGEST, params).context(UrlParseSnafu)
}

pub async fn request(query: &str) -> Result<Option<StoreSuggestResult>, Error> {
    let url = endpoint(query)?;
    reqwest::get(url)
        .await
        .context(ReqwestRequestGetSnafu)?
        .json::<StoreAutoSuggest>()
        .await
        .context(ReqwestResponseJsonSnafu)?
        .result_sets
        .into_iter()
        .flat_map(|result_set| result_set.suggests)
        .filter_map(|suggest| {
            if suggest.source != "Game" {
                None
            } else {
                let query = query.as_bytes();
                let title = suggest.title.as_bytes();
                Some((triple_accel::levenshtein_exp(query, title), suggest))
            }
        })
        .collect::<Vec<_>>()
        .tap_mut(|results| results.sort_by(|lhs, rhs| lhs.0.cmp(&rhs.0)))
        .into_iter()
        .map(|suggest| suggest.1)
        .next()
        .pipe(Ok)
}
