use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    MenuItemSetTitle { source: tauri::Error },
    TauriBuild { source: tauri::Error },
    WindowHide { source: tauri::Error },
    WindowIsVisible { source: tauri::Error },
    WindowShow { source: tauri::Error },
}

mod config;
mod handler;
mod tray;

pub(crate) fn init() -> Result<(), Error> {
    let context = tauri::generate_context!();

    let system_tray_menu = tauri::SystemTrayMenu::new()
        .add_item(tauri::CustomMenuItem::new(
            crate::app::tray::visibility::ID,
            crate::app::tray::visibility::hide::TITLE,
        ))
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(tauri::CustomMenuItem::new(
            crate::app::tray::quit::ID,
            crate::app::tray::quit::TITLE,
        ));

    let system_tray = tauri::SystemTray::new().with_menu(system_tray_menu);

    let app = tauri::Builder::default()
        .system_tray(system_tray)
        .invoke_handler(handler::invoke())
        .on_system_tray_event(handler::system_tray())
        .build(context)
        .context(TauriBuildSnafu)?;

    app.run(handler::run());

    Ok(())
}
