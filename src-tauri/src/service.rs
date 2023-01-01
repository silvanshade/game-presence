use snafu::prelude::*;
use tauri::window::PlatformWebview;

// pub mod nintendo;
pub mod playstation;
pub mod steam;
pub mod twitch;
pub mod xbox;

pub struct Webview2Error(webview2_com::Error);

impl std::fmt::Debug for Webview2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::fmt::Display for Webview2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for Webview2Error {
}

impl From<webview2_com::Error> for Webview2Error {
    fn from(value: webview2_com::Error) -> Self {
        Self(value)
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[cfg(windows)]
    ClearState {
        backtrace: snafu::Backtrace,
        source: windows::core::Error,
    },

    StdMpscChannelRecv {
        backtrace: snafu::Backtrace,
        source: std::sync::mpsc::RecvError,
    },
    StdTimeDurationSince {
        backtrace: snafu::Backtrace,
        source: std::time::SystemTimeError,
    },
    StdU64TryIntoI64 {
        backtrace: snafu::Backtrace,
        source: std::num::TryFromIntError,
    },
    TauriWindowWithWebview {
        backtrace: snafu::Backtrace,
        source: tauri::Error,
    },
    TauriWithWebview {
        backtrace: snafu::Backtrace,
        source: tauri::Error,
    },
    #[cfg(target_os = "linux")]
    WebKit2GtkWebsiteDataManagerClear {
        backtrace: snafu::Backtrace,
        source: webkit2gtk::glib::Error,
    },
    #[cfg(target_os = "linux")]
    WebKit2GtkWebviewWebContext { backtrace: snafu::Backtrace },
    #[cfg(target_os = "linux")]
    WebKit2GtkWebContextCookieManager { backtrace: snafu::Backtrace },
    #[cfg(target_os = "windows")]
    WindowsCoreWebView2 {
        backtrace: snafu::Backtrace,
        source: windows::core::Error,
    },
    #[cfg(target_os = "windows")]
    WindowsWebView2CallDevToolsProtocolMethodCompletedHandler {
        backtrace: snafu::Backtrace,
        source: Webview2Error,
    },
    #[cfg(target_os = "windows")]
    WindowsWebView2Navigate {
        backtrace: snafu::Backtrace,
        source: windows::core::Error,
    },
}

trait TauriWindowExt: private::Sealed {
    fn navigate(&self, url: url::Url, clear_data_first: bool) -> Result<(), Error>;
}

impl TauriWindowExt for tauri::Window {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    fn navigate(&self, url: url::Url, clear_data_first: bool) -> Result<(), Error> {
        let (tx, rx) = std::sync::mpsc::channel();
        self.with_webview(move |webview| tx.send(navigate(webview, url, clear_data_first)).unwrap())
            .context(TauriWithWebviewSnafu)?;
        rx.recv().context(StdMpscChannelRecvSnafu)?
    }
}

#[cfg(target_os = "macos")]
#[cfg_attr(feature = "tracing", tracing::instrument(skip(webview)))]
fn navigate(webview: PlatformWebview, url: url::Url, clear_data_first: bool) -> Result<(), Error> {
    use block::ConcreteBlock;
    use objc::{runtime::Object, *};
    use objc_foundation::{INSString, NSString};

    let webview = webview.inner();

    unsafe {
        let handler = {
            let block = ConcreteBlock::new(move || {
                let string = NSString::from_str(url.as_str());
                let url: *mut Object = msg_send![class!(NSURL), URLWithString: string];
                let request: *mut Object = msg_send![class!(NSURLRequest), requestWithURL: url];
                let _navigation: *mut Object = { msg_send![webview, loadRequest: request] };
            });
            block.copy()
        };
        if clear_data_first {
            let configuration: *mut Object = msg_send![webview, configuration];
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

#[cfg(target_os = "linux")]
#[cfg_attr(feature = "tracing", tracing::instrument(skip(webview)))]
fn navigate(webview: PlatformWebview, url: url::Url, clear_data_first: bool) -> Result<(), Error> {
    use webkit2gtk::{CookieManagerExt, WebContextExt, WebViewExt};

    let webview = webview.inner();

    if clear_data_first {
        let context = webview.context().context(WebKit2GtkWebviewWebContextSnafu)?;
        let manager = context
            .cookie_manager()
            .context(WebKit2GtkWebContextCookieManagerSnafu)?;
        #[allow(deprecated)]
        manager.delete_all_cookies();
        context.clear_cache();
    }
    webview.load_uri(url.as_str());

    Ok(())
}

#[cfg(target_os = "windows")]
#[cfg_attr(feature = "tracing", tracing::instrument(skip(webview)))]
fn navigate(webview: PlatformWebview, url: url::Url, clear_data_first: bool) -> Result<(), Error> {
    use std::{rc::Rc, sync::mpsc};
    use webview2_com::CallDevToolsProtocolMethodCompletedHandler;
    use windows::{
        core::{HSTRING, PCWSTR},
        w,
    };

    let url = PCWSTR::from(&HSTRING::from(url.as_str()));

    unsafe {
        let webview = Rc::new(webview.controller().CoreWebView2().context(WindowsCoreWebView2Snafu)?);

        if clear_data_first {
            // clear browser cookies
            let (tx, rx) = mpsc::channel();
            CallDevToolsProtocolMethodCompletedHandler::wait_for_async_operation(
                {
                    let webview = webview.clone();
                    Box::new(move |handler| {
                        webview.CallDevToolsProtocolMethod(w!("Network.clearBrowserCookies"), w!("{}"), &handler)?;
                        Ok(())
                    })
                },
                Box::new(move |hresult, pcwstr| {
                    hresult?;
                    tx.send(pcwstr).unwrap();
                    Ok(())
                }),
            )
            .map_err(Into::into)
            .context(WindowsWebView2CallDevToolsProtocolMethodCompletedHandlerSnafu)?;
            rx.recv().unwrap();

            // clear browser cache
            let (tx, rx) = mpsc::channel();
            CallDevToolsProtocolMethodCompletedHandler::wait_for_async_operation(
                {
                    let webview = webview.clone();
                    Box::new(move |handler| {
                        webview.CallDevToolsProtocolMethod(w!("Network.clearBrowserCache"), w!("{}"), &handler)?;
                        Ok(())
                    })
                },
                Box::new(move |hresult, pcwstr| {
                    hresult?;
                    tx.send(pcwstr).unwrap();
                    Ok(())
                }),
            )
            .map_err(Into::into)
            .context(WindowsWebView2CallDevToolsProtocolMethodCompletedHandlerSnafu)?;
            rx.recv().unwrap();
        }

        // navigate
        webview.Navigate(url).context(WindowsWebView2NavigateSnafu)?;
    }

    Ok(())
}

mod private {
    pub trait Sealed {}
    impl Sealed for tauri::Window {
    }
    impl<'a, T> Sealed for &'a T where T: ?Sized + Sealed
    {
    }
}
