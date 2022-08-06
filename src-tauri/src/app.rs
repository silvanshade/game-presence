use snafu::prelude::*;

pub mod frontend;
pub mod metadata;
mod model;

pub use self::model::Model;

#[derive(Debug, Snafu)]
enum Error {
    TauriAppGetWindow { label: String },
    TauriWindowIsVisible { source: tauri::Error },
    TauriWindowHide { source: tauri::Error },
    TauriWindowShow { source: tauri::Error },
    TauriSystemTrayMenuItemSetTitle { source: tauri::Error },
}

pub fn make_system_tray() -> tauri::SystemTray {
    let system_tray_menu = tauri::SystemTrayMenu::new()
        .add_item(tauri::CustomMenuItem::new("toggle-hide-show", "Hide"))
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(tauri::CustomMenuItem::new("exit-app", "Quit"));
    tauri::SystemTray::new().with_menu(system_tray_menu)
}

pub fn handle_run_events(app: &tauri::AppHandle, event: tauri::RunEvent) {
    match event {
        tauri::RunEvent::Exit => {
            // client.close().unwrap();
        },
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        },
        tauri::RunEvent::WindowEvent {
            label,
            event: tauri::WindowEvent::CloseRequested { api, .. },
            ..
        } => {
            if label == "main" {
                api.prevent_close();
                self::toggle_hide_show_window(app).expect("failed to toggle hide/show for window");
            }
        },
        _ => {},
    }
}

pub fn handle_system_tray_events(app: &tauri::AppHandle, event: tauri::SystemTrayEvent) {
    if let tauri::SystemTrayEvent::MenuItemClick { id, .. } = event {
        match id.as_str() {
            "exit-app" => {
                app.exit(exitcode::OK);
            },
            "toggle-hide-show" => {
                self::toggle_hide_show_window(app).expect("failed to toggle hide/show for window");
            },
            _ => {},
        }
    }
}

fn toggle_hide_show_window(app: &tauri::AppHandle) -> Result<(), self::Error> {
    use tauri::Manager;

    let label = "main";
    let window = app.get_window(label).context(TauriAppGetWindowSnafu { label })?;
    let new_title = if window.is_visible().context(TauriWindowIsVisibleSnafu)? {
        window.hide().context(TauriWindowHideSnafu)?;
        "Show"
    } else {
        window.show().context(TauriWindowShowSnafu)?;
        "Hide"
    };
    app.tray_handle()
        .get_item("toggle-hide-show")
        .set_title(new_title)
        .context(TauriSystemTrayMenuItemSetTitleSnafu)?;

    Ok(())
}
