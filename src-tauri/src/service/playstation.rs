// NOTE: for PSN API details see https://github.com/Tustin/PlayStationDiscord

use crate::service::TauriWindowExt;
use serde::Deserialize;
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    ReqwestRequestJson { source: reqwest::Error },
    ReqwestRequestSend { source: reqwest::Error },
    SerdeUrlEncoded { source: serde::de::value::Error },
    TauriSpawn { source: tauri::Error },
    TauriWindowBuilderNew { source: tauri::Error },
    TauriWindowClose { source: tauri::Error },
    TauriWindowNavigate { source: crate::service::Error },
    TokioMpscReceive,
    UrlParse { source: url::ParseError },
    UrlQuery,
}

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Deserialize)]
struct ResponseAuthorize {
    code: String,
}

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Deserialize)]
struct ResponseToken {
    // access_token: String,
    // token_type: String,
    // refresh_token: String,
    // expires_in: u64,
    // scope: String,
}

const APP_CLIENT_ID: &str = "YWM4ZDE2MWEtZDk2Ni00NzI4LWIwZWEtZmZlYzIyZjY5ZWRjOkRFaXhFcVhYQ2RYZHdqMHY=";

const APP_REDIRECT_URI: &str = "com.playstation.PlayStationApp://redirect";

const ENDPOINT_AUTHORIZE: &str = "https://ca.account.sony.com/api/authz/v3/oauth/authorize";

const ENDPOINT_TOKEN: &str = "https://ca.account.sony.com/api/authz/v3/oauth/token";

mod params {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn common<'a, 'b>() -> impl Iterator<Item = (&'a str, &'b str)> {
        [
            ("access_type", "offline"),
            ("app_context", "inapp_ios"),
            ("device_profile", "mobile"),
            ("smcid", "psapp:settings-entrance"),
            ("support_scheme", "sneiprls"),
            ("token_format", "jwt"),
            ("ui", "pr"),
            ("extraQueryParams", "{ PlatformPrivacyWs1 = minimal; }"),
            ("redirect_uri", super::APP_REDIRECT_URI),
        ]
        .into_iter()
    }
}

#[cfg_attr(feature = "tracing", tracing::instrument)]
pub fn endpoint_authorize_url() -> Result<url::Url, Error> {
    url::Url::parse_with_params(
        ENDPOINT_AUTHORIZE,
        params::common().chain(
            [
                ("response_type", "code"),
                ("scope", "psn:mobile.v1 psn:clientapp"),
                ("client_id", "ac8d161a-d966-4728-b0ea-ffec22f69edc"),
                ("duid", "0000000d0004008088347AA0C79542D3B656EBB51CE3EBE1"),
            ]
            .into_iter(),
        ),
    )
    .context(UrlParseSnafu)
}

#[cfg_attr(feature = "tracing", tracing::instrument)]
async fn request_authorize(app: &tauri::AppHandle, reauthorize: bool) -> Result<ResponseAuthorize, Error> {
    let (tx_response, mut rx_response) = tokio::sync::mpsc::channel::<url::Url>(2);

    let window = {
        let navigation_handler = move |url: url::Url| {
            let str = url.as_str();
            if str.starts_with(&APP_REDIRECT_URI.to_ascii_lowercase()) {
                tx_response.blocking_send(url).unwrap();
                return false;
            }
            true
        };
        tauri::WindowBuilder::new(
            app,
            "playstation-service-authorization",
            tauri::WindowUrl::App("/html/empty".into()),
        )
        .on_navigation(navigation_handler)
        .build()
        .context(TauriWindowBuilderNewSnafu)?
    };
    window
        .navigate(endpoint_authorize_url()?, reauthorize)
        .context(TauriWindowNavigateSnafu)?;

    let redirect = rx_response.recv().await.context(TokioMpscReceiveSnafu)?;
    println!("redirect: {}", redirect);
    tauri::async_runtime::spawn(async move { window.close().context(TauriWindowCloseSnafu) })
        .await
        .context(TauriSpawnSnafu)??;

    let query = redirect.query().context(UrlQuerySnafu)?;
    let response_authorize = serde_urlencoded::from_str::<ResponseAuthorize>(query).context(SerdeUrlEncodedSnafu)?;
    Ok(response_authorize)
}

#[cfg_attr(feature = "tracing", tracing::instrument)]
async fn request_token(response_authorize: ResponseAuthorize) -> Result<ResponseToken, Error> {
    let client = reqwest::Client::new();
    let request = client
        .post(ENDPOINT_TOKEN)
        .header("Authorization", format!("Basic {}", APP_CLIENT_ID))
        .form(
            &params::common()
                .chain(
                    [
                        ("grant_type", "authorization_code"),
                        ("code", response_authorize.code.as_str()),
                        ("redirect_uri", APP_REDIRECT_URI),
                    ]
                    .into_iter(),
                )
                .collect::<std::collections::HashMap<&str, &str>>(),
        );
    let response = request.send().await.context(ReqwestRequestSendSnafu)?;
    let response_token = response
        .json::<ResponseToken>()
        .await
        .context(ReqwestRequestJsonSnafu)?;
    Ok(response_token)
}

#[cfg_attr(feature = "tracing", tracing::instrument)]
pub async fn authorization_flow(app: &tauri::AppHandle, reauthorize: bool) -> Result<(), Error> {
    let response_authorize = request_authorize(app, reauthorize).await?;
    let _response_token = request_token(response_authorize).await?;
    Ok(())
}
