#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use snafu::prelude::*;

pub mod api;
pub mod app;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Snafu)]
enum Error {
    NoneError,
    SerdeJsonDeserializeError {
        source: serde_json::Error,
    },
    TauriError {
        source: tauri::Error,
    },
    #[cfg(feature = "debug")]
    TracingSubscriberError {
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

fn main() -> Result<(), self::Error> {
    #[cfg(feature = "debug")]
    tracing_subscriber::fmt::try_init().context(TracingSubscriberSnafu)?;

    let context = tauri::generate_context!();

    #[allow(unused_mut)]
    let mut app = tauri::Builder::default()
        .manage(crate::app::Model::new())
        .system_tray(self::make_system_tray())
        .on_system_tray_event(self::handle_system_tray_events)
        .build(context)
        .context(TauriSnafu)?;

    // hide app from Dock on macOS
    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    app.run(self::handle_run_events);

    Ok(())
}

fn make_system_tray() -> tauri::SystemTray {
    let system_tray_menu = tauri::SystemTrayMenu::new()
        .add_item(tauri::CustomMenuItem::new("toggle-hide-show", "Hide"))
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(tauri::CustomMenuItem::new("exit-app", "Quit"));
    tauri::SystemTray::new().with_menu(system_tray_menu)
}

fn handle_run_events(app: &tauri::AppHandle, event: tauri::RunEvent) {
    use tauri::Manager;

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
                let window = app.get_window("main").unwrap();
                let new_title = {
                    window.hide().unwrap();
                    "Show"
                };
                app.tray_handle()
                    .get_item("toggle-hide-show")
                    .set_title(new_title)
                    .unwrap();
            }
        },
        _ => {},
    }
}

fn handle_system_tray_events(app: &tauri::AppHandle, event: tauri::SystemTrayEvent) {
    use tauri::Manager;

    match event {
        tauri::SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "exit-app" => {
                app.exit(exitcode::OK);
            },
            "toggle-hide-show" => {
                let window = app.get_window("main").unwrap();
                let new_title = if window.is_visible().unwrap() {
                    window.hide().unwrap();
                    "Show"
                } else {
                    window.show().unwrap();
                    "Hide"
                };
                app.tray_handle()
                    .get_item("toggle-hide-show")
                    .set_title(new_title)
                    .unwrap();
            },
            _ => {},
        },
        _ => {},
    }
}
