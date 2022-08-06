use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
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

pub fn toggle_hide_show_window(app: &tauri::AppHandle) -> Result<(), self::Error> {
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
