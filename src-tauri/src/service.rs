use snafu::prelude::*;
use tauri::window::PlatformWebview;

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
        source: windows::core::Error,
    },
    #[cfg(target_os = "windows")]
    WindowsWebView2Navigate {
        source: windows::core::Error,
    },
}

trait PlatformWebviewExt {
    fn navigate(&self, url: url::Url, clear_data_first: bool) -> Result<(), Error>;
}

#[cfg(target_os = "macos")]
impl PlatformWebviewExt for PlatformWebview {
    fn navigate(&self, url: url::Url, clear_data_first: bool) -> Result<(), Error> {
        use block::ConcreteBlock;
        use objc::{runtime::Object, *};
        use objc_foundation::{INSString, NSString};

        let web_view = self.inner();

        unsafe {
            let handler = {
                let block = ConcreteBlock::new(move || {
                    let string = NSString::from_str(url.as_str());
                    let url: *mut Object = msg_send![class!(NSURL), URLWithString: string];
                    let request: *mut Object = msg_send![class!(NSURLRequest), requestWithURL: url];
                    let _navigation: *mut Object = { msg_send![web_view, loadRequest: request] };
                });
                block.copy()
            };

            if clear_data_first {
                let configuration: *mut Object = msg_send![web_view, configuration];
                let data_store: *mut Object = msg_send![configuration, websiteDataStore];
                let data_types: *mut Object = msg_send![class!(WKWebsiteDataStore), allWebsiteDataTypes];
                let date: *mut Object = msg_send![class!(NSDate), distantPast];
                let _: () =
                    msg_send![data_store, removeDataOfTypes: data_types modifiedSince: date completionHandler: handler];
            } else {
                handler.call(());
            }
        }

        Ok(())
    }
}

#[cfg(target_os = "linux")]
impl PlatformWebviewExt for PlatformWebview {
    fn navigate(&self, url: url::Url, clear_data_first: bool) -> Result<(), Error> {
        use webkit2gtk::{CookieManagerExt, WebContextExt, WebViewExt};

        let web_view = self.inner();

        if clear_data_first {
            let context = web_view.context().context(WebKit2GtkWebviewWebContextSnafu)?;
            let manager = context
                .cookie_manager()
                .context(WebKit2GtkWebContextCookieManagerSnafu)?;
            #[allow(deprecated)]
            manager.delete_all_cookies();
            context.clear_cache();
        }
        web_view.load_uri(url.as_str());

        Ok(())
    }
}

#[cfg(target_os = "windows")]
impl PlatformWebviewExt for PlatformWebview {
    fn navigate(&self, url: url::Url, clear_data_first: bool) -> Result<(), Error> {
        use webview2_com::CallDevToolsProtocolMethodCompletedHandler;
        use windows::{
            core::{HSTRING, PCWSTR},
            w,
        };

        let url = PCWSTR::from(&HSTRING::from(url.as_str()));

        unsafe {
            let web_view = self.controller().CoreWebView2().context(WindowsCoreWebView2Snafu)?;

            if clear_data_first {
                web_view
                    .CallDevToolsProtocolMethod(
                        w!("Network.clearBrowserCookies"),
                        w!("{}"),
                        &CallDevToolsProtocolMethodCompletedHandler::create(Box::new({
                            let web_view = web_view.clone();
                            move |hresult, _pcwstr| {
                                hresult?;
                                web_view.CallDevToolsProtocolMethod(
                                    w!("Network.clearBrowserCache"),
                                    w!("{}"),
                                    &CallDevToolsProtocolMethodCompletedHandler::create(Box::new({
                                        let web_view = web_view.clone();
                                        move |hresult, _pcwstr| {
                                            hresult?;
                                            web_view.Navigate(url)?;
                                            Ok(())
                                        }
                                    })),
                                )?;
                                Ok(())
                            }
                        })),
                    )
                    .context(WindowsWebView2CallDevToolsProtocolMethodSnafu)?;
            } else {
                web_view.Navigate(url).context(WindowsWebView2NavigateSnafu)?;
            }
        }

        Ok(())
    }
}
