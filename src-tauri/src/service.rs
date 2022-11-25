use snafu::prelude::*;

pub mod nintendo;
pub mod playstation;
pub mod steam;
pub mod xbox;

#[derive(Debug, Snafu)]
pub enum Error {
    #[cfg(windows)]
    ClearState {
        source: windows::core::Error,
    },
    TauriWindowWithWebview {
        source: tauri::Error,
    },
}

trait PlatformWebviewExt {
    type Error;
    fn clear_state(&self) -> Result<(), Self::Error>;
}

#[cfg(windows)]
impl PlatformWebviewExt for tauri::window::PlatformWebview {
    type Error = windows::core::Error;

    fn clear_state(&self) -> Result<(), Self::Error> {
        use windows::w;
        let controller = self.controller();
        unsafe {
            let web_view = controller.CoreWebView2()?;
            web_view.CallDevToolsProtocolMethod(w!("Network.clearBrowserCache"), w!("{}"), None)?;
            web_view.CallDevToolsProtocolMethod(w!("Network.clearBrowserCookies"), w!("{}"), None)?;
        }
        Ok(())
    }
}

trait WindowExt {
    fn clear_webview_state(&self) -> Result<(), Error>;
}

impl WindowExt for tauri::Window {
    fn clear_webview_state(&self) -> Result<(), Error> {
        self.with_webview(|webview| webview.clear_state().unwrap())
            .context(TauriWindowWithWebviewSnafu)
    }
}
