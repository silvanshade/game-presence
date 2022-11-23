use serde::Deserialize;
use snafu::prelude::*;
use tauri::Manager;
use url::form_urlencoded::Target;

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
    TokioMpscSend {
        source: tokio::sync::mpsc::error::SendError<()>,
    },
    TokioOneShotReceive {
        source: tokio::sync::oneshot::error::RecvError,
    },
    TwitchAuthorizationFailed,
    UrlParse {
        source: url::ParseError,
    },
}

impl warp::reject::Reject for Error {
}

const GAME_PRESENCE_CLIENT_ID: &str = "0vvuyyk8c79jvwqwc9b4hmbqb3sjdr";

const AUTHORIZATION_REDIRECT_URL: &str = "http://localhost:3000/api/twitch/authorize/redirect";

const AUTHORIZATION_WINDOW_CLOSE_EVENT: &str = "authorization-complete";

const AUTHORIZATION_HANDLER_FINISHED_EVENT: &str = "authorization-handler-finished";

pub fn authorization_html_path() -> std::path::PathBuf {
    [
        env!("CARGO_MANIFEST_DIR"),
        "..",
        "src",
        "assets",
        "html",
        "api",
        "twitch",
        "authorize.html",
    ]
    .iter()
    .collect::<std::path::PathBuf>()
}

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
    let (tx_handler_ready, rx_handler_ready) = tokio::sync::oneshot::channel();
    let (tx_window_close, mut rx_window_close) = tokio::sync::mpsc::channel::<()>(2);

    let access_token_result = tauri::async_runtime::spawn(authorization_flow_redirect_handler(
        csrf_token,
        tx_handler_ready,
        tx_window_close,
    ));

    rx_handler_ready.await.context(TokioOneShotReceiveSnafu)?;

    let window = tauri::WindowBuilder::new(app, "twitch-integration-authorization", tauri::WindowUrl::External(url))
        .build()
        .context(TauriWindowBuilderNewSnafu)?;

    rx_window_close.recv().await;
    tauri::async_runtime::spawn(async move { window.close().context(TauriWindowCloseSnafu) })
        .await
        .context(TauriSpawnSnafu)??;

    match access_token_result.await.context(TauriSpawnSnafu)? {
        Ok(access_token) => {
            todo!()
        },
        Err(err) => Err(err),
    }
}

async fn authorization_flow_redirect_handler(
    csrf_token: twitch_oauth2::CsrfToken,
    tx_handler_ready: tokio::sync::oneshot::Sender<()>,
    tx_window_close: tokio::sync::mpsc::Sender<()>,
) -> Result<twitch_oauth2::AccessToken, Error> {
    use warp::Filter;

    let result = std::sync::Arc::new(tokio::sync::RwLock::new(None::<String>));

    let (tx_handler_finished, mut rx_handler_finished) = tokio::sync::mpsc::channel::<()>(2);

    let routes = {
        let authorize = warp::path!("api" / "twitch" / "authorize" / ..);
        let redirect = warp::path!("redirect").and(warp::fs::file(authorization_html_path()));
        let token = warp::path!("token").and(warp::query()).and_then({
            let result = result.clone();
            move |data: std::collections::HashMap<String, String>| {
                let csrf_token = csrf_token.clone();
                let tx_handler_finished = tx_handler_finished.clone();
                let tx_window_close = tx_window_close.clone();
                let result = result.clone();
                async move {
                    tx_handler_finished.send(()).await.context(TokioMpscSendSnafu)?;
                    if let (Some(access_token), Some(state)) = (data.get("access_token"), data.get("state")) {
                        if csrf_token.secret() == state {
                            *result.write().await = Some(String::from(access_token));
                            tx_handler_finished.send(()).await.unwrap();
                            tx_window_close.send(()).await.unwrap();
                            return Ok::<_, warp::reject::Rejection>(warp::reply());
                        }
                    }
                    Err(warp::reject::custom(Error::TwitchAuthorizationFailed))
                }
            }
        });
        authorize.and(redirect.or(token))
    };

    let server = warp::serve(routes)
        .bind_with_graceful_shutdown(([127, 0, 0, 1], 3000), async move {
            tx_handler_ready.send(()).unwrap();
            rx_handler_finished.recv().await;
        })
        .1;

    server.await;

    let result = result.read().await.clone();

    match result {
        None => Err(Error::TwitchAuthorizationFailed),
        Some(access_token) => Ok(twitch_oauth2::AccessToken::new(access_token)),
    }
}
