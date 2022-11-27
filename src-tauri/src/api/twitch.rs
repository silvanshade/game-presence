use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    TauriSpawn {
        source: tauri::Error,
    },
    TauriWindowBuilderNew {
        source: tauri::Error,
    },
    TauriWindowClose {
        source: tauri::Error,
    },
    TokioMpscReceive,
    TokioMpscSend {
        source: tokio::sync::mpsc::error::SendError<()>,
    },
    TokioOneShotReceive {
        source: tokio::sync::oneshot::error::RecvError,
    },
    TwitchAuthorizationFailedWithInvalidQuery {
        query: std::collections::HashMap<String, String>,
    },
    TwitchAuthorizationFailedWithInvalidState {
        state: String,
    },
    TwitchAuthorizationFailedWithMissingQuery {
        url: String,
    },
    UrlParse {
        source: url::ParseError,
    },
}

const GAME_PRESENCE_CLIENT_ID: &str = "0vvuyyk8c79jvwqwc9b4hmbqb3sjdr";

const AUTHORIZATION_REDIRECT_URL: &str = "http://localhost:3000/api/twitch/authorize/redirect";

pub fn authorization_request_url() -> Result<(url::Url, twitch_oauth2::types::CsrfToken), Error> {
    use twitch_oauth2::{tokens::ImplicitUserTokenBuilder, ClientId};
    let client_id = ClientId::from_static(GAME_PRESENCE_CLIENT_ID);
    let redirect_url = authorization_redirect_url()?;
    let result = ImplicitUserTokenBuilder::new(client_id, redirect_url)
        .force_verify(true)
        .generate_url();
    Ok(result)
}

pub fn authorization_redirect_url() -> Result<url::Url, Error> {
    url::Url::parse(AUTHORIZATION_REDIRECT_URL).context(UrlParseSnafu)
}

pub async fn authorization_flow<R: tauri::Runtime>(app: &tauri::AppHandle<R>) -> Result<(), Error> {
    let (url, csrf_token) = authorization_request_url()?;
    let (tx_token, mut rx_token) = tokio::sync::mpsc::channel::<Result<twitch_oauth2::AccessToken, Error>>(2);

    let window = {
        let label = "twitch-integration-authorization";
        let url = tauri::WindowUrl::External(url);
        let navigation_handler = move |url: String| {
            if url.starts_with(AUTHORIZATION_REDIRECT_URL) {
                let result;
                let suffix = &url[AUTHORIZATION_REDIRECT_URL.len() + 1 ..];
                if let Ok(query) = serde_urlencoded::from_str::<std::collections::HashMap<String, String>>(suffix) {
                    if let (Some(access_token), Some(state)) = (query.get("access_token"), query.get("state")) {
                        if csrf_token.secret() == state {
                            let access_token = twitch_oauth2::AccessToken::new(access_token.clone());
                            result = Ok(access_token);
                        } else {
                            let state = state.clone();
                            result = Err(Error::TwitchAuthorizationFailedWithInvalidState { state });
                        }
                    } else {
                        result = Err(Error::TwitchAuthorizationFailedWithInvalidQuery { query });
                    }
                } else {
                    result = Err(Error::TwitchAuthorizationFailedWithMissingQuery { url });
                }
                tx_token.blocking_send(result).unwrap();
                return false;
            }
            true
        };
        tauri::WindowBuilder::new(app, label, url)
            .on_navigation(navigation_handler)
            .build()
            .context(TauriWindowBuilderNewSnafu)?
    };

    let token_result = rx_token.recv().await.context(TokioMpscReceiveSnafu)?;
    tauri::async_runtime::spawn(async move { window.close().context(TauriWindowCloseSnafu) })
        .await
        .context(TauriSpawnSnafu)??;

    match token_result {
        Ok(_access_token) => {
            println!("got token");
            Ok(())
        },
        Err(err) => Err(err),
    }
}
