use crate::service::PlatformWebviewExt;
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    RandTryFillBytes { source: rand::Error },

    UrlParse { source: url::ParseError },
    TokioMpscReceive,
    TauriSpawn { source: tauri::Error },
    TauriWindowBuilderNew { source: tauri::Error },
    TauriWindowClose { source: tauri::Error },
    TauriWithWebview { source: tauri::Error },
}

struct AuthorizeChallenge {
    state: String,
    session_token_code_challenge: String,
    session_token_code_verifier: String,
}

impl AuthorizeChallenge {
    pub fn new() -> Result<Self, Error> {
        use rand::RngCore;
        use ring::digest::{Context, SHA256};

        let mut rng = rand::thread_rng();

        let state = {
            let mut bytes = [0u8; 36];
            rng.try_fill_bytes(&mut bytes).context(RandTryFillBytesSnafu)?;
            base64::encode_config(bytes, base64::URL_SAFE_NO_PAD)
        };
        let session_token_code_verifier = {
            let mut bytes = [0u8; 32];
            rng.try_fill_bytes(&mut bytes).context(RandTryFillBytesSnafu)?;
            base64::encode_config(bytes, base64::URL_SAFE_NO_PAD)
        };
        let session_token_code_challenge = {
            let mut context = Context::new(&SHA256);
            context.update(session_token_code_verifier.as_bytes());
            let digest = context.finish();
            base64::encode_config(digest, base64::URL_SAFE_NO_PAD)
        };
        Ok(Self {
            state,
            session_token_code_challenge,
            session_token_code_verifier,
        })
    }
}

const ENDPOINT_AUTHORIZE: &str = "https://accounts.nintendo.com/connect/1.0.0/authorize";

// #[derive(Debug, Deserialize, Serialize)]
// pub struct RequestAuthorize {
//     state: String,
//     redirect_uri: String,
//     scope: String,
//     response_type: String,
//     session_token_code_challenge: String,
//     session_token_code_challenge_method: String,
//     theme: String,
// }

// impl RequestAuthorize {
//     pub fn new() -> Result<Self, Error> {
//         let AuthorizeChallenge {
//             state,
//             session_token_code_challenge,
//             ..
//         } = AuthorizeChallenge::new()?;
//         let redirect_uri = String::from("npf71b963c1b7b6d119://auth&client_id=71b963c1b7b6d119");
//         let scope = String::default();
//         let response_type = String::from("session_token_code");
//         let session_token_code_challenge_method = String::from("S256");
//         let theme = String::from("login_form");
//         Ok(Self {
//             state,
//             redirect_uri,
//             scope,
//             response_type,
//             session_token_code_challenge,
//             session_token_code_challenge_method,
//             theme,
//         })
//     }
// }

pub async fn authorization_flow(app: &tauri::AppHandle<tauri::Wry>) -> Result<(), Error> {
    const REDIRECT_URI_PREFIX: &str = "npf71b963c1b7b6d119://auth";

    let AuthorizeChallenge {
        state,
        session_token_code_challenge,
        ..
    } = AuthorizeChallenge::new()?;

    let endpoint_authorize_url = {
        let mut buffer = String::from(ENDPOINT_AUTHORIZE);
        buffer.push_str(&format!("?state={}", state));
        buffer.push_str(&format!(
            "&redirect_uri={}",
            format!("{}&client_id=71b963c1b7b6d119", REDIRECT_URI_PREFIX)
        ));
        buffer.push_str(&format!(
            "&scope={}",
            "openid%20user%20user.birthday%20user.mii%20user.screenName"
        ));
        buffer.push_str(&format!("&response_type={}", "session_token_code"));
        buffer.push_str(&format!(
            "&session_token_code_challenge={}",
            session_token_code_challenge
        ));
        buffer.push_str(&format!("&session_token_code_challenge_method={}", "S256"));
        buffer.push_str(&format!("&theme={}", "login_form"));
        url::Url::parse(&buffer).context(UrlParseSnafu)?
    };

    // let endpoint_authorize_url = url::Url::parse_with_params(ENDPOINT_AUTHORIZE, [
    //     ("state", state.as_str()),
    //     (
    //         "redirect_uri",
    //         format!("{}&client_id=71b963c1b7b6d119", REDIRECT_URI_PREFIX).as_str(),
    //     ),
    //     ("scope", "openid%20user%20user.birthday%20user.mii%20user.screenName"),
    //     ("response_type", "session_token_code"),
    //     ("session_token_code_challenge", session_token_code_challenge.as_str()),
    //     ("session_token_code_challenge_method", "S256"),
    //     ("theme", "login_form"),
    // ])
    // .context(UrlParseSnafu)?;
    println!("{}", endpoint_authorize_url.as_str());

    let (tx_response, mut rx_response) = tokio::sync::mpsc::channel::<String>(2);

    let window = {
        let navigation_handler = move |url: String| {
            println!("url: {}", url);
            if url.starts_with(REDIRECT_URI_PREFIX) {
                tx_response.blocking_send(url).unwrap();
                return false;
            }
            true
        };
        tauri::WindowBuilder::new(
            app,
            "nintendo-service-authorization",
            tauri::WindowUrl::App("/html/empty".into()),
        )
        .on_navigation(navigation_handler)
        .build()
        .context(TauriWindowBuilderNewSnafu)?
    };
    window
        .with_webview({
            move |webview| {
                let clear_data_first = false;
                webview.navigate(endpoint_authorize_url, clear_data_first).unwrap();
            }
        })
        .context(TauriWithWebviewSnafu)?;

    let response = rx_response.recv().await.context(TokioMpscReceiveSnafu)?;
    println!("response: {}", response);
    tauri::async_runtime::spawn(async move { window.close().context(TauriWindowCloseSnafu) })
        .await
        .context(TauriSpawnSnafu)??;

    println!("response: {}", response);

    Ok(())
}
