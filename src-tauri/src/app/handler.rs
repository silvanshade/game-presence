use crate::app::{
    Error,
    TauriGetWindowSnafu,
    TauriMenuItemSetTitleSnafu,
    TauriWindowHideSnafu,
    TauriWindowIsVisibleSnafu,
    TauriWindowShowSnafu,
};
use snafu::prelude::*;
use tauri::Manager;

#[cfg_attr(feature = "tracing", tracing::instrument)]
fn toggle_visibility<R: tauri::Runtime>(app: &tauri::AppHandle<R>) -> Result<(), Error> {
    let label = "main";
    let window = app.get_window(label).context(TauriGetWindowSnafu { label })?;
    let id = crate::app::tray::visibility::ID;
    let item = app.tray_handle().get_item(id);
    if window
        .is_visible()
        .context(TauriWindowIsVisibleSnafu { label: window.label() })?
    {
        let title = crate::app::tray::visibility::show::TITLE;
        item.set_title(title)
            .context(TauriMenuItemSetTitleSnafu { id, title })?;
        window.hide().context(TauriWindowHideSnafu { label: window.label() })?;
    } else {
        let title = crate::app::tray::visibility::hide::TITLE;
        item.set_title(title)
            .context(TauriMenuItemSetTitleSnafu { id, title })?;
        window.show().context(TauriWindowShowSnafu { label: window.label() })?;
    }
    Ok(())
}

#[cfg_attr(feature = "tracing", tracing::instrument)]
pub fn invoke() -> impl Fn(tauri::Invoke<tauri::Wry>) {
    tauri::generate_handler![
        crate::app::command::service_playstation_authorization_flow,
        crate::app::command::service_twitch_authorization_flow,
        crate::app::command::service_xbox_authorization_flow,
        crate::app::command::nintendo_auth_ready,
        crate::app::command::playstation_auth_ready,
        crate::app::command::steam_auth_ready,
        crate::app::command::xbox_auth_ready,
    ]
}

#[cfg_attr(feature = "tracing", tracing::instrument)]
pub fn run() -> impl FnMut(&tauri::AppHandle, tauri::RunEvent) {
    use tauri::{RunEvent, WindowEvent};
    |app, run_event| match run_event {
        RunEvent::Ready => {},
        RunEvent::WindowEvent {
            label,
            event: WindowEvent::CloseRequested { api, .. },
            ..
        } if label == "main" => {
            api.prevent_close();
            self::toggle_visibility(app).unwrap();
        },
        RunEvent::Exit => {
            app.try_state::<crate::app::Model>()
                .expect("model should have already been managed by state")
                .notifiers
                .exit
                .notify_waiters();
        },
        _ => {},
    }
}

#[cfg_attr(feature = "tracing", tracing::instrument)]
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
