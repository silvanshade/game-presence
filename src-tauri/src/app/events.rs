pub fn handle_run(app: &tauri::AppHandle, event: tauri::RunEvent) {
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
                crate::app::gui::toggle_hide_show_window(app).expect("failed to toggle hide/show for window");
            }
        },
        _ => {},
    }
}

pub fn handle_system_tray(app: &tauri::AppHandle, event: tauri::SystemTrayEvent) {
    if let tauri::SystemTrayEvent::MenuItemClick { id, .. } = event {
        match id.as_str() {
            "exit-app" => {
                app.exit(exitcode::OK);
            },
            "toggle-hide-show" => {
                crate::app::gui::toggle_hide_show_window(app).expect("failed to toggle hide/show for window");
            },
            _ => {},
        }
    }
}
