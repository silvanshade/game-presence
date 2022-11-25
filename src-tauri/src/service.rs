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
    fn navigate(&self, uri: &str) -> Result<(), Error>;
}

#[cfg(target_os = "linux")]
impl PlatformWebviewExt for tauri::window::PlatformWebview {
    fn clear_data(&self) -> Result<(), Error> {
        use webkit2gtk::{
            gio,
            glib,
            WebsiteData,
            WebsiteDataManager,
            WebsiteDataManagerExt,
            WebsiteDataManagerExtManual,
            WebsiteDataTypes,
        };

        let manager = WebsiteDataManager::new_ephemeral();

        {
            let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
            let types = WebsiteDataTypes::ALL - WebsiteDataTypes::COOKIES;
            let timespan = glib::TimeSpan::from_seconds(0);
            let cancellable = gio::Cancellable::NONE;
            let callback = move |result: Result<(), glib::Error>| {
                tx.send(result).unwrap();
            };
            manager.clear(types, timespan, cancellable, callback);
            rx.attach(None, |result| match result {
                Ok(()) => glib::Continue(true),
                Err(error) => {
                    println!("error calling WebsiteDataManager::clear: {:#?}", error);
                    glib::Continue(false)
                },
            });
        }

        {
            let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
            let types = WebsiteDataTypes::COOKIES;
            let cancellable = gio::Cancellable::NONE;
            let callback = move |result: Result<Vec<WebsiteData>, glib::Error>| {
                println!("fetch callback");
                match result {
                    Ok(data) => {
                        for datum in data {
                            let types = datum.types();
                            println!("name:  {:#?}", datum.name());
                            println!("size:  {:#?}", datum.size(types));
                            println!("types: {:#?}", types);
                        }
                        tx.send(Ok(())).unwrap();
                    },
                    Err(error) => {
                        tx.send(Err(error)).unwrap();
                    },
                }
            };
            manager.fetch(types, cancellable, callback);
            rx.attach(None, |result| match result {
                Ok(()) => glib::Continue(true),
                Err(error) => {
                    println!("error calling WebsiteDataManager::fetch: {:#?}", error);
                    glib::Continue(false)
                },
            });
        }

        Ok(())
    }

    fn navigate(&self, uri: &str) -> Result<(), Error> {
        use webkit2gtk::WebViewExt;
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
