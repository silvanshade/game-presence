use snafu::prelude::*;

pub mod data;
mod handler;
mod tray;

#[derive(Debug, Snafu)]
pub enum Error {
    MenuItemSetTitle { source: tauri::Error },
    TauriBuild { source: tauri::Error },
    WindowHide { source: tauri::Error },
    WindowIsVisible { source: tauri::Error },
    WindowShow { source: tauri::Error },
}

fn make_system_tray() -> tauri::SystemTray {
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
    tauri::SystemTray::new().with_menu(system_tray_menu)
}

pub(crate) fn init() -> Result<(), Error> {
    let context = tauri::generate_context!();

    let builder = tauri::Builder::default();

    let builder = builder.invoke_handler(handler::invoke());

    let builder = builder.system_tray(self::make_system_tray());
    let builder = builder.on_system_tray_event(handler::system_tray());

    let app = builder.build(context).context(TauriBuildSnafu)?;

    app.run(handler::run());

    Ok(())
}
