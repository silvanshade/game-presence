use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    TokioOneShotReceive {
        source: tokio::sync::oneshot::error::RecvError,
    },
    UrlParse {
        source: url::ParseError,
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
}

const CLIENT_ID: &str = "0vvuyyk8c79jvwqwc9b4hmbqb3sjdr";

mod endpoint {
    pub const AUTHORIZATION_REQUEST: &str = "https://id.twitch.tv/oauth2/authorize";
}

mod event {
    pub const AUTHORIZATION_WINDOW_CLOSE: &str = "close";

    #[derive(Clone, Copy, Debug)]
    pub struct AuthorizationWindowClose;
}

fn authorization_endpoint() -> Result<url::Url, Error> {
    url::Url::parse(self::endpoint::AUTHORIZATION_REQUEST).context(UrlParseSnafu)
}

pub async fn open_authorization_window<R: tauri::Runtime, M: tauri::Manager<R>>(manager: &M) -> Result<(), Error> {
    let label = "";
    let url = tauri::WindowUrl::External(authorization_endpoint()?);
    let window = tauri::WindowBuilder::new(manager, label, url)
        .build()
        .context(TauriWindowBuilderNewSnafu)?;

    let (tx_close, rx_close) = tokio::sync::oneshot::channel::<self::event::AuthorizationWindowClose>();
    window.once(self::event::AUTHORIZATION_WINDOW_CLOSE, |_event| {
        let message = self::event::AuthorizationWindowClose;
        tx_close
            .send(message)
            .expect(&format!(r#"failed to send message: "{:#?}""#, message));
    });

    println!("after once");

    rx_close.await.context(TokioOneShotReceiveSnafu)?;
    let result = tauri::async_runtime::spawn(async move { window.close().context(TauriWindowCloseSnafu) })
        .await
        .context(TauriSpawnSnafu)?;

    result
}
