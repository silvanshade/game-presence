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
        use webkit2gtk::{gio, glib, WebsiteDataManager, WebsiteDataManagerExtManual, WebsiteDataTypes};

        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        let types = WebsiteDataTypes::ALL - WebsiteDataTypes::COOKIES;
        let timespan = glib::TimeSpan::from_seconds(0);
        let cancellable = gio::Cancellable::NONE;
        let callback = move |result: Result<(), glib::Error>| {
            tx.send(result).unwrap();
        };
        let manager = WebsiteDataManager::new_ephemeral();
        manager.clear(types, timespan, cancellable, callback);

        rx.attach(None, |result| match result {
            Ok(()) => {
                println!("clear");
                glib::Continue(true)
            },
            Err(error) => {
                println!("error: {:#?}", error);
                glib::Continue(false)
            },
        });

        Ok(())
    }

    fn navigate(&self, uri: &str, clear_data: bool) -> Result<(), Error> {
        use webkit2gtk::WebViewExt;
        if clear_data {
            self.clear_data()?;
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
