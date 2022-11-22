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
    use snafu::prelude::*;

    const AUTHORIZATION_REDIRECT: &str = "http://localhost/api/twitch/auth/redirect";

    pub fn authorization_redirect() -> Result<url::Url, super::Error> {
        let client_id = twitch_oauth2::types::ClientId::from_static(super::CLIENT_ID);
        let redirect_url = url::Url::parse(super::endpoint::AUTHORIZATION_REDIRECT).context(super::UrlParseSnafu)?;
        let (url, _csrf_token) =
            twitch_oauth2::tokens::ImplicitUserTokenBuilder::new(client_id, redirect_url).generate_url();
        Ok(url)
    }
}

mod event {
    pub const AUTHORIZATION_WINDOW_CLOSE: &str = "close";

    #[derive(Clone, Copy, Debug)]
    pub struct AuthorizationWindowClose;
}

pub mod window {
    pub mod authorization {
        use crate::api::twitch::{
            endpoint,
            event,
            Error,
            TauriSpawnSnafu,
            TauriWindowBuilderNewSnafu,
            TauriWindowCloseSnafu,
            TokioOneShotReceiveSnafu,
        };
        use snafu::prelude::*;

        pub const LABEL: &str = "Twitch Integration Authorization";

        pub async fn open<R: tauri::Runtime, M: tauri::Manager<R>>(manager: &M) -> Result<(), self::Error> {
            let label = LABEL;
            let url = tauri::WindowUrl::External(self::endpoint::authorization_redirect()?);
            let window = tauri::WindowBuilder::new(manager, label, url)
                .build()
                .context(TauriWindowBuilderNewSnafu)?;

            let (tx_close, rx_close) = tokio::sync::oneshot::channel();
            window.once(self::event::AUTHORIZATION_WINDOW_CLOSE, |_event| {
                let message = self::event::AuthorizationWindowClose;
                tx_close
                    .send(message)
                    .expect(&format!(r#"failed to send message: "{:#?}""#, message));
            });

            rx_close.await.context(TokioOneShotReceiveSnafu)?;
            let result = tauri::async_runtime::spawn(async move { window.close().context(TauriWindowCloseSnafu) })
                .await
                .context(TauriSpawnSnafu)?;

            result
        }
    }
}
