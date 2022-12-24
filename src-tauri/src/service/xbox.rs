mod authorize;
mod autosuggest;
mod presence;

pub use self::{authorize::XstsToken, autosuggest::StoreSuggestResult, presence::PresenceRecord};

use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    ModelUpdateGui {
        source: crate::app::model::Error,
    },
    Oauth2AuthUrlNew {
        source: url::ParseError,
    },
    Oauth2CsrfTokenStateSecretMismatch {
        state: String,
        csrf_token: oauth2::CsrfToken,
    },
    Oauth2ExchangeCode {
        source: oauth2::basic::BasicRequestTokenError<oauth2::reqwest::Error<reqwest::Error>>,
    },
    Oauth2RedirectUrlNew {
        source: url::ParseError,
    },
    Oauth2TokenUrlNew {
        source: url::ParseError,
    },
    ReqwestRequestGet {
        source: reqwest::Error,
    },
    ReqwestResponseJson {
        source: reqwest::Error,
    },
    ReqwestRequestSend {
        source: reqwest::Error,
    },
    SerdeJsonFromValue {
        source: serde_json::Error,
    },
    SerdeUrlEncoded {
        source: serde::de::value::Error,
    },
    StdSyncMpscReceive {
        source: std::sync::mpsc::RecvError,
    },
    TauriWindowNavigate {
        source: crate::service::Error,
    },
    TauriSpawn {
        source: tauri::Error,
    },
    TauriWindowBuilderNew {
        source: tauri::Error,
    },
    TauriWindowClose {
        source: tauri::Error,
    },
    UrlDropResizeParams,
    UrlParse {
        source: url::ParseError,
    },
    UrlQuery,
    XboxTokenXui,
}

pub async fn authorize(app: &tauri::AppHandle, reauthorize: bool) -> Result<(), Error> {
    self::authorize::flow(app, reauthorize).await
}

pub async fn autosuggest(query: &str) -> Result<Option<StoreSuggestResult>, Error> {
    println!("autosuggest::query: {:#?}", query);
    self::autosuggest::request(query).await
}

pub async fn presence(xsts: &XstsToken) -> Result<PresenceRecord, Error> {
    self::presence::request(xsts).await
}
