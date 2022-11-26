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
    #[cfg(target_os = "linux")]
    WebKit2GtkWebviewWebContext,
    #[cfg(target_os = "linux")]
    WebKit2GtkWebContextCookieManager,
    #[cfg(target_os = "windows")]
    WindowsCoreWebView2 {
        source: windows::core::Error,
    },
    #[cfg(target_os = "windows")]
    WindowsWebView2CallDevToolsProtocolMethod {
        method: String,
        source: windows::core::Error,
    },
    WindowsWebView2Navigate {
        source: windows::core::Error,
    },
}

trait PlatformWebviewExt {
    fn clear_data(&self) -> Result<(), Error>;
    fn load_url(&self, url: url::Url) -> Result<(), Error>;
    fn navigate(&self, url: url::Url, clear_data: bool) -> Result<(), Error> {
        if clear_data {
            self.clear_data()?;
        }
        self.load_url(url)?;
        Ok(())
    }
}

#[cfg(target_os = "linux")]
impl PlatformWebviewExt for tauri::window::PlatformWebview {
    fn clear_data(&self) -> Result<(), Error> {
        use webkit2gtk::{CookieManagerExt, WebContextExt};

        let inner = self.inner();

        let context = inner.context().context(WebKit2GtkWebviewWebContextSnafu)?;
        let manager = context
            .cookie_manager()
            .context(WebKit2GtkWebContextCookieManagerSnafu)?;

        #[allow(deprecated)]
        manager.delete_all_cookies();
        context.clear_cache();

        Ok(())
    }

    fn load_url(&self, url: url::Url) -> Result<(), Error> {
        use webkit2gtk::WebViewExt;
        self.inner().load_uri(url.as_str());
        Ok(())
    }
}

#[cfg(target_os = "windows")]
impl PlatformWebviewExt for tauri::window::PlatformWebview {
    fn clear_data(&self) -> Result<(), Error> {
        use windows::w;
        let controller = self.controller();
        unsafe {
            let web_view = controller.CoreWebView2().context(WindowsCoreWebView2Snafu)?;
            web_view
                .CallDevToolsProtocolMethod(w!("Network.clearBrowserCookies"), w!("{}"), None)
                .context(WindowsWebView2CallDevToolsProtocolMethodSnafu {
                    method: String::from("Network.clearBrowserCookies"),
                })?;
            web_view
                .CallDevToolsProtocolMethod(w!("Network.clearBrowserCache"), w!("{}"), None)
                .context(WindowsWebView2CallDevToolsProtocolMethodSnafu {
                    method: String::from("Network.clearBrowserCache"),
                })?;
        }
        Ok(())
    }

    fn load_url(&self, url: url::Url) -> Result<(), Error> {
        use windows::core::{HSTRING, PCWSTR};
        let url = PCWSTR::from(&HSTRING::from(url.as_str()));
        let controller = self.controller();
        unsafe {
            let web_view = controller.CoreWebView2().context(WindowsCoreWebView2Snafu)?;
            web_view.Navigate(url).context(WindowsWebView2NavigateSnafu)
        }
    }
}
