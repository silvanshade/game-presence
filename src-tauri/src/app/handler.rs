pub use snafu::prelude::*;

fn toggle_visibility<R: tauri::Runtime>(app: &tauri::AppHandle<R>) -> Result<(), crate::app::Error> {
    use tauri::Manager;
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
    use crate::app::command;
    tauri::generate_handler![
        command::service::authorization_flow_playstation,
        command::service::authorization_flow_twitch
    ]
}

pub fn run() -> impl FnMut(&tauri::AppHandle<tauri::Wry>, tauri::RunEvent) {
    use tauri::{RunEvent, WindowEvent};
    |app, run_event| match run_event {
        RunEvent::Ready => {
            // noop
        },
        RunEvent::WindowEvent {
            label,
            event: WindowEvent::CloseRequested { api, .. },
            ..
        } if label == "main" => {
            api.prevent_close();
            self::toggle_visibility(app).unwrap();
        },
        _ => {},
    }
}

pub fn system_tray<R: tauri::Runtime>() -> impl Fn(&tauri::AppHandle<R>, tauri::SystemTrayEvent) {
    use tauri::SystemTrayEvent;
    |app, event| match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            crate::app::tray::quit::ID => {
                let code = proc_exit::Code::SUCCESS;
                code.process_exit();
            },
            crate::app::tray::visibility::ID => self::toggle_visibility(app).unwrap(),
            _ => {},
        },
        _ => {},
    }
}
