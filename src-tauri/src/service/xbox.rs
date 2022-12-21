use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    Oauth2AuthUrlNew {
        source: url::ParseError,
    },
    Oauth2CsrfTokenStateSecretMismatch {
        state: String,
        token: CsrfToken,
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
    SerdeUrlEncoded {
        source: serde::de::value::Error,
    },
    StdSyncMpscReceive {
        source: std::sync::mpsc::RecvError,
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
    UrlParse {
        source: url::ParseError,
    },
    UrlQuery,
}

pub mod api {
    pub mod authorize {
        use super::super::{
            Error,
            Oauth2AuthUrlNewSnafu,
            Oauth2ExchangeCodeSnafu,
            Oauth2RedirectUrlNewSnafu,
            Oauth2TokenUrlNewSnafu,
            SerdeUrlEncodedSnafu,
            StdSyncMpscReceiveSnafu,
            TauriSpawnSnafu,
            TauriWindowBuilderNewSnafu,
            TauriWindowCloseSnafu,
            UrlParseSnafu,
            UrlQuerySnafu,
        };
        use serde::Deserialize;
        use snafu::prelude::*;
        use tap::prelude::*;

        const CLIENT_ID: &str = "6d97ccd0-5a71-48c5-9bc3-a203a183da22";

        const REDIRECT_URL: &str = "http://localhost:3000/api/xbox/authorize/redirect";

        const AUTH_SCOPES: [&str; 2] = ["xboxlive.signin", "xboxlive.offline_access"];

        const AUTH_URL: &str = "https://login.microsoftonline.com/common/oauth2/v2.0/authorize";

        const TOKEN_URL: &str = "https://login.microsoftonline.com/common/oauth2/v2.0/token";

        struct AuthCodeData {
            code: oauth2::AuthorizationCode,
            state: oauth2::CsrfToken,
        }

        impl AuthCodeData {
            pub fn new(code: String, state: String) -> Self {
                let code = oauth2::AuthorizationCode::new(code);
                let state = oauth2::CsrfToken::new(state);
                Self { code, state }
            }
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct AuthCodeQuery {
            code: String,
            state: String,
        }

        fn client() -> Result<oauth2::basic::BasicClient, Error> {
            let client_id = CLIENT_ID.conv::<String>().pipe(oauth2::ClientId::new);
            let client_secret = None;
            let auth_url = AUTH_URL
                .conv::<String>()
                .pipe(oauth2::AuthUrl::new)
                .context(Oauth2AuthUrlNewSnafu)?;
            let token_url = TOKEN_URL
                .conv::<String>()
                .pipe(oauth2::TokenUrl::new)
                .context(Oauth2TokenUrlNewSnafu)?
                .conv::<Option<_>>();
            let redirect_uri = "http://localhost:3000/api/xbox/authorize/redirect"
                .conv::<String>()
                .pipe(oauth2::RedirectUrl::new)
                .context(Oauth2RedirectUrlNewSnafu)?;
            let client = oauth2::basic::BasicClient::new(client_id, client_secret, auth_url, token_url)
                .set_redirect_uri(redirect_uri);
            Ok(client)
        }

        async fn flow(app: &tauri::AppHandle<tauri::Wry>) -> Result<(), Error> {
            let client = client()?;
            let (pkce_code_challenge, pkce_code_verifier) = oauth2::PkceCodeChallenge::new_random_sha256();
            let AuthCodeData { code, state } = flow_get_auth_code(app, &client, pkce_code_challenge).await?;
            let token = flow_get_bearer_token(&client, code, pkce_code_verifier).await?;
            Ok(())
        }

        async fn flow_get_auth_code(
            app: &tauri::AppHandle<tauri::Wry>,
            client: &oauth2::basic::BasicClient,
            pkce_code_challenge: oauth2::PkceCodeChallenge,
        ) -> Result<AuthCodeData, Error> {
            let (auth_url, csrf_token) = {
                let scopes = AUTH_SCOPES
                    .into_iter()
                    .map(|scope| scope.conv::<String>().pipe(oauth2::Scope::new));
                client
                    .authorize_url(oauth2::CsrfToken::new_random)
                    .add_scopes(scopes)
                    .set_pkce_challenge(pkce_code_challenge)
                    .url()
            };

            let (tx, rx) = std::sync::mpsc::channel::<String>();
            let window = {
                tauri::WindowBuilder::new(
                    app,
                    "twitch-integration-authorization",
                    tauri::WindowUrl::External(auth_url),
                )
                .on_navigation(move |url: String| {
                    println!("navigating: {}", url);
                    if url.starts_with(REDIRECT_URL) {
                        tx.send(url).expect("failed to send redirect URL back from window");
                        return false;
                    }
                    true
                })
                .build()
                .context(TauriWindowBuilderNewSnafu)?
            };

            let auth_redirect = rx
                .recv()
                .context(StdSyncMpscReceiveSnafu)?
                .as_str()
                .pipe(url::Url::parse)
                .context(UrlParseSnafu)?;

            tauri::async_runtime::spawn(async move { window.close().context(TauriWindowCloseSnafu) })
                .await
                .context(TauriSpawnSnafu)??;

            let AuthCodeQuery { code, state } = auth_redirect
                .query()
                .context(UrlQuerySnafu)?
                .pipe(serde_urlencoded::from_str::<AuthCodeQuery>)
                .context(SerdeUrlEncodedSnafu)?;

            if &state != csrf_token.secret() {
                return Err(Error::Oauth2CsrfTokenStateSecretMismatch {
                    state,
                    token: csrf_token,
                });
            }

            Ok(AuthCodeData::new(code, state))
        }

        async fn flow_get_bearer_token(
            client: &oauth2::basic::BasicClient,
            code: oauth2::AuthorizationCode,
            pkce_code_verifier: oauth2::PkceCodeVerifier,
        ) -> Result<oauth2::basic::BasicTokenResponse, Error> {
            let token = client
                .exchange_code(code)
                .set_pkce_verifier(pkce_code_verifier)
                .request_async(oauth2::reqwest::async_http_client)
                .await
                .context(Oauth2ExchangeCodeSnafu)?;
            Ok(token)
        }

        fn flow_get_xsts_user_token() -> Result<(), Error> {
            Ok(())
        }

        fn flow_get_xsts_xtoken() -> Result<(), Error> {
            Ok(())
        }
    }
}
