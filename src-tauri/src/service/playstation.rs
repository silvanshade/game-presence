use crate::service::PlatformWebviewExt;
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
    TauriWithWebview { source: tauri::Error },
    TokioMpscReceive,
    UrlParse { source: url::ParseError },
    UrlQuery,
}

#[derive(Debug, Deserialize)]
struct ResponseAuthorize {
    code: String,
}

#[derive(Debug, Deserialize)]
struct ResponseToken {
    access_token: String,
    token_type: String,
    refresh_token: String,
    expires_in: u64,
    scope: String,
}

const CLIENT_AUTHORIZATION: &str = "YWM4ZDE2MWEtZDk2Ni00NzI4LWIwZWEtZmZlYzIyZjY5ZWRjOkRFaXhFcVhYQ2RYZHdqMHY=";

const ENDPOINT_AUTHORIZE: &str = "https://ca.account.sony.com/api/authz/v3/oauth/authorize";

const ENDPOINT_TOKEN: &str = "https://ca.account.sony.com/api/authz/v3/oauth/token";

const REDIRECT_URI: &str = "com.playstation.playstationapp://redirect";

pub fn endpoint_authorize_url() -> Result<url::Url, Error> {
    url::Url::parse_with_params(ENDPOINT_AUTHORIZE, &[
        ("response_type", "code"),
        ("app_context", "inapp_ios"),
        ("device_profile", "mobile"),
        (
            "extraQueryParams",
            "%7B%0A%20%20%20%20PlatformPrivacyWs1%20%3D%20minimal%3B%0A%7D",
        ),
        ("token_format", "jwt"),
        ("access_type", "offline"),
        ("scope", "psn%3Amobile.v1%20psn%3Aclientapp"),
        ("service_entity", "urn%3Aservice-entity%3Apsn"),
        ("ui", "pr"),
        ("smcid", "psapp%253Asettings-entrance"),
        ("darkmode", "true"),
        ("redirect_uri", "com.playstation.PlayStationApp%3A%2F%2Fredirect"),
        ("support_scheme", "sneiprls"),
        ("client_id", "ac8d161a-d966-4728-b0ea-ffec22f69edc"),
        ("duid", "0000000d0004008088347AA0C79542D3B656EBB51CE3EBE1"),
        ("device_base_font_size", "10"),
        ("elements_visibility", "no_aclink"),
        ("service_logo", "ps"),
    ])
    .context(UrlParseSnafu)
}

async fn request_authorize(app: &tauri::AppHandle<tauri::Wry>) -> Result<ResponseAuthorize, Error> {
    let (tx_response, mut rx_response) = tokio::sync::mpsc::channel::<String>(2);

    let window = {
        let navigation_handler = move |url: String| {
            if url.starts_with(REDIRECT_URI) {
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
        .with_webview({
            let url = endpoint_authorize_url()?;
            move |webview| {
                let clear_data_first = true;
                webview.navigate(url, clear_data_first).unwrap();
            }
        })
        .context(TauriWithWebviewSnafu)?;

    let response = rx_response.recv().await.context(TokioMpscReceiveSnafu)?;
    println!("response: {}", response);
    tauri::async_runtime::spawn(async move { window.close().context(TauriWindowCloseSnafu) })
        .await
        .context(TauriSpawnSnafu)??;

    let redirect = url::Url::parse(&response).context(UrlParseSnafu)?;
    let query = redirect.query().context(UrlQuerySnafu)?;
    let response_authorize = serde_urlencoded::from_str::<ResponseAuthorize>(query).context(SerdeUrlEncodedSnafu)?;
    Ok(response_authorize)
}

async fn request_token(response_authorize: ResponseAuthorize) -> Result<ResponseToken, Error> {
    let client = reqwest::Client::new();
    let request = client
        .post(ENDPOINT_TOKEN)
        .header("Authorization", format!("Basic {}", CLIENT_AUTHORIZATION))
        .form(
            &[
                ("smcid", "psapp%3Asettings-entrance"),
                ("access_type", "offline"),
                ("code", response_authorize.code.as_str()),
                ("service_logo", "ps"),
                ("ui", "pr"),
                ("elements_visibility", "no_aclink"),
                ("redirect_uri", REDIRECT_URI),
                ("support_scheme", "sneiprls"),
                ("grant_type", "authorization_code"),
                ("darkmode", "true"),
                ("token_format", "jwt"),
                ("device_profile", "mobile"),
                ("app_context", "inapp_ios"),
                ("extraQueryParams", "{ PlatformPrivacyWs1 = minimal; }"),
            ]
            .into_iter()
            .collect::<std::collections::HashMap<&str, &str>>(),
        );
    let response = request.send().await.context(ReqwestRequestSendSnafu)?;
    let response_token = response
        .json::<ResponseToken>()
        .await
        .context(ReqwestRequestJsonSnafu)?;
    Ok(response_token)
}

pub async fn authorization_flow(app: &tauri::AppHandle<tauri::Wry>) -> Result<(), Error> {
    let response_authorize = request_authorize(app).await?;
    let response_token = request_token(response_authorize).await?;
    Ok(())
}
