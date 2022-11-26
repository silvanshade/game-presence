use crate::service::PlatformWebviewExt;
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    TauriSpawn { source: tauri::Error },
    TauriWindowBuilderNew { source: tauri::Error },
    TauriWindowClose { source: tauri::Error },
    TauriWithWebview { source: tauri::Error },
    TokioMpscReceive,
    // TokioMpscSend {
    //     source: tokio::sync::mpsc::error::SendError<()>,
    // },
    // TokioOneShotReceive {
    //     source: tokio::sync::oneshot::error::RecvError,
    // },
    // TwitchAuthorizationFailedWithInvalidQuery {
    //     query: std::collections::HashMap<String, String>,
    // },
    // TwitchAuthorizationFailedWithInvalidState {
    //     state: String,
    // },
    // TwitchAuthorizationFailedWithMissingQuery {
    //     url: String,
    // },
    UrlParse { source: url::ParseError },
}

const AUTHORIZATION_REQUEST_URL: &str = "https://ca.account.sony.com/api/authz/v3/oauth/authorize?response_type=code&app_context=inapp_ios&device_profile=mobile&extraQueryParams=%7B%0A%20%20%20%20PlatformPrivacyWs1%20%3D%20minimal%3B%0A%7D&token_format=jwt&access_type=offline&scope=psn%3Amobile.v1%20psn%3Aclientapp&service_entity=urn%3Aservice-entity%3Apsn&ui=pr&smcid=psapp%253Asettings-entrance&darkmode=true&redirect_uri=com.playstation.PlayStationApp%3A%2F%2Fredirect&support_scheme=sneiprls&client_id=ac8d161a-d966-4728-b0ea-ffec22f69edc&duid=0000000d0004008088347AA0C79542D3B656EBB51CE3EBE1&device_base_font_size=10&elements_visibility=no_aclink&service_logo=ps";

const AUTHORIZATION_REDIRECT_URL: &str = "com.playstation.playstationapp://redirect/";

pub fn authorization_request_url() -> Result<url::Url, Error> {
    url::Url::parse(AUTHORIZATION_REQUEST_URL).context(UrlParseSnafu)
}

pub fn authorization_redirect_url() -> Result<url::Url, Error> {
    url::Url::parse(AUTHORIZATION_REQUEST_URL).context(UrlParseSnafu)
}

pub async fn authorization_flow(app: &tauri::AppHandle<tauri::Wry>) -> Result<(), Error> {
    let (tx_token, mut rx_token) = tokio::sync::mpsc::channel::<Result<String, Error>>(2);

    let window = {
        let navigation_handler = move |url: String| {
            if url.starts_with(AUTHORIZATION_REDIRECT_URL) {
                let result = Ok(url);
                // let suffix = &url[AUTHORIZATION_REDIRECT_URL.len() + 1 ..];
                // if let Ok(query) = serde_urlencoded::from_str::<std::collections::HashMap<String, String>>(suffix) {
                //     if let (Some(access_token), Some(state)) = (query.get("access_token"), query.get("state")) {
                //         if csrf_token.secret() == state {
                //             let access_token = twitch_oauth2::AccessToken::new(access_token.clone());
                //             result = Ok(access_token);
                //         } else {
                //             let state = state.clone();
                //             result = Err(Error::TwitchAuthorizationFailedWithInvalidState { state });
                //         }
                //     } else {
                //         result = Err(Error::TwitchAuthorizationFailedWithInvalidQuery { query });
                //     }
                // } else {
                //     result = Err(Error::TwitchAuthorizationFailedWithMissingQuery { url });
                // }
                tx_token.blocking_send(result).unwrap();
                return false;
            }
            true
        };
        tauri::WindowBuilder::new(
            app,
            "twitch-integration-authorization",
            tauri::WindowUrl::App("/html/empty".into()),
        )
        .on_navigation(navigation_handler)
        .build()
        .context(TauriWindowBuilderNewSnafu)?
    };
    window
        .with_webview({
            // let url = authorization_request_url()?;
            let url = url::Url::parse("https://google.com").unwrap();
            move |webview| {
                let clear_data = true;
                webview.navigate(url, clear_data).unwrap();
            }
        })
        .context(TauriWithWebviewSnafu)?;

    let token_result = rx_token.recv().await.context(TokioMpscReceiveSnafu)?;
    tauri::async_runtime::spawn(async move { window.close().context(TauriWindowCloseSnafu) })
        .await
        .context(TauriSpawnSnafu)??;

    match token_result {
        Ok(access_token) => {
            println!("got token: {}", access_token);
            Ok(())
        },
        Err(err) => Err(err),
    }
}
