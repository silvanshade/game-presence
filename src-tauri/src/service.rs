use snafu::prelude::*;
use webkit2gtk::WebViewExt;

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

    StdMpscChannelRecv {
        source: std::sync::mpsc::RecvError,
    },
    StdTimeDurationSince {
        source: std::time::SystemTimeError,
    },
    StdU64TryIntoI64 {
        source: std::num::TryFromIntError,
    },
    TauriWindowWithWebview {
        source: tauri::Error,
    },
    #[cfg(target_os = "linux")]
    WebKit2GtkWebsiteDataManagerClear {
        source: webkit2gtk::glib::Error,
    },
    #[cfg(target_os = "windows")]
    WindowsCoreWebView2 {
        source: windows::core::Error,
    },
    #[cfg(target_os = "windows")]
    WindowsWebView2CallDevToolsProtocolMethod {
        source: windows::core::Error,
    },
}

trait PlatformWebviewExt {
    fn clear_data(&self) -> Result<(), Error>;
    fn navigate(&self, uri: &str, clear_data: bool) -> Result<(), Error>;
}

#[cfg(target_os = "linux")]
impl PlatformWebviewExt for tauri::window::PlatformWebview {
    fn clear_data(&self) -> Result<(), Error> {
        use webkit2gtk::{CookieManagerExt, WebContextExt};

        let inner = self.inner();

        let context = inner.context().unwrap();
        let manager = context.cookie_manager().unwrap();

        #[allow(deprecated)]
        manager.delete_all_cookies();
        context.clear_cache();

        Ok(())
    }

    fn navigate(&self, uri: &str, clear_data: bool) -> Result<(), Error> {
        if clear_data {
            self.clear_data();
        }
        self.inner().load_uri(uri);
        Ok(())
    }
}

#[cfg(target_os = "windows")]
impl PlatformWebviewExt for tauri::window::PlatformWebview {
    fn clear_data(&self) -> Result<(), Error> {
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
