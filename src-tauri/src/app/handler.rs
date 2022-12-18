use snafu::prelude::*;
use tauri::Manager;

fn toggle_visibility<R: tauri::Runtime>(app: &tauri::AppHandle<R>) -> Result<(), crate::app::Error> {
    let window = app.get_window("main").expect(r#"failed to get "main" window"#);
    let item = app.tray_handle().get_item(crate::app::tray::visibility::ID);
    if window.is_visible().context(crate::app::TauriWindowIsVisibleSnafu)? {
        item.set_title(crate::app::tray::visibility::show::TITLE)
            .context(crate::app::TauriMenuItemSetTitleSnafu)?;
        window.hide().context(crate::app::TauriWindowHideSnafu)?;
    } else {
        item.set_title(crate::app::tray::visibility::hide::TITLE)
            .context(crate::app::TauriMenuItemSetTitleSnafu)?;
        window.show().context(crate::app::TauriWindowShowSnafu)?;
    }
    Ok(())
}

pub fn invoke() -> impl Fn(tauri::Invoke<tauri::Wry>) {
    tauri::generate_handler![
        crate::app::command::service_playstation_authorization_flow,
        crate::app::command::service_twitch_authorization_flow,
    ]
}

pub fn run() -> impl FnMut(&tauri::AppHandle<tauri::Wry>, tauri::RunEvent) {
    use tauri::{RunEvent, WindowEvent};
    |app, run_event| match run_event {
        RunEvent::Ready => {
            // tauri::async_runtime::spawn(async move {
            //     let query = "atomic heart";
            //     if let Some(result) =
            // crate::service::xbox::request_autosuggest(query).await.unwrap() {
            //         println!("image: {:#?}", result.image_url().unwrap().as_str());
            //         println!("store: {:#?}", result.store_url().unwrap().as_str());
            //     }
            // });
        },
        RunEvent::WindowEvent {
            label,
            event: WindowEvent::CloseRequested { api, .. },
            ..
        } if label == "main" => {
            api.prevent_close();
            self::toggle_visibility(app).unwrap();
        },
        RunEvent::Exit => {
            app.state::<crate::app::Model>().notifiers.exit.notify_waiters();
        },
        _ => {},
    }
}

pub fn system_tray<R: tauri::Runtime>() -> impl Fn(&tauri::AppHandle<R>, tauri::SystemTrayEvent) {
    use tauri::SystemTrayEvent;
    |app, event| match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            crate::app::tray::quit::ID => {
                // NOTE: we manually close all windows (rather than app::exit) so that RunEvent::Exit triggers
                for window in app.windows().values() {
                    window
                        .close()
                        .expect(&format!("failed to close window: {}", window.label()));
                }
            },
            crate::app::tray::visibility::ID => self::toggle_visibility(app).unwrap(),
            _ => {},
        },
        _ => {},
    }
}
