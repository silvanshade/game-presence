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

const AUTHORIZATION_REDIRECT_URL: &str = "http://localhost:3000/api/twitch/authorize";

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

pub fn authorization_request_url() -> Result<url::Url, Error> {
    use twitch_oauth2::{tokens::ImplicitUserTokenBuilder, ClientId};
    let client_id = ClientId::from_static(GAME_PRESENCE_CLIENT_ID);
    let redirect_url = authorization_redirect_url()?;
    let (url, _csrf_token) = ImplicitUserTokenBuilder::new(client_id, redirect_url)
        .force_verify(true)
        .generate_url();
    Ok(url)
}

pub fn authorization_redirect_url() -> Result<url::Url, Error> {
    url::Url::parse(AUTHORIZATION_REDIRECT_URL).context(UrlParseSnafu)
}

pub async fn authorization_flow<R: tauri::Runtime>(app: &tauri::AppHandle<R>) -> Result<(), Error> {
    let (tx_handler_ready, rx_handler_ready) = tokio::sync::oneshot::channel();

    let access_token_result = {
        let app = app.clone();
        tauri::async_runtime::spawn(authorization_flow_redirect_handler(app, tx_handler_ready))
    };

    rx_handler_ready.await.context(TokioOneShotReceiveSnafu)?;

    let window = {
        let label = "twitch-integration-authorization";
        let url = tauri::WindowUrl::External(authorization_request_url()?);
        tauri::WindowBuilder::new(app, label, url)
            .build()
            .context(TauriWindowBuilderNewSnafu)?
    };

    let (tx_close, rx_close) = tokio::sync::oneshot::channel();
    window.once(AUTHORIZATION_WINDOW_CLOSE_EVENT, |_event| {
        tx_close.send(()).unwrap();
    });

    rx_close.await.context(TokioOneShotReceiveSnafu)?;

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

async fn authorization_flow_redirect_handler<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    tx_handler_ready: tokio::sync::oneshot::Sender<()>,
) -> Result<(), Error> {
    use warp::Filter;

    let (tx_handler_finished, mut rx_handler_finished) = tokio::sync::mpsc::channel::<()>(1);

    let routes = warp::path!("api" / "twitch" / "authorize")
        .and(warp::query())
        .and(warp::fs::file(authorization_html_path()))
        .and_then(move |map: std::collections::HashMap<String, String>, file| {
            let tx_handler_finished = tx_handler_finished.clone();
            async move {
                if map.get("authorization_finished").is_some() {
                    tx_handler_finished.send(()).await.context(TokioMpscSendSnafu)?;
                }
                Ok::<_, warp::reject::Rejection>(file)
            }
        });

    let server = warp::serve(routes)
        .bind_with_graceful_shutdown(([127, 0, 0, 1], 3000), async move {
            tx_handler_ready.send(()).unwrap();
            rx_handler_finished.recv().await;
        })
        .1;

    server.await;

    Ok(())
}
