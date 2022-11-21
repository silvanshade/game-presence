use snafu::prelude::*;

pub mod command;
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

    // create the default builder
    let builder = tauri::Builder::default();

    // configure the system tray
    let builder = builder.system_tray(self::make_system_tray());

    // handle RPC command invocations
    let builder = builder.invoke_handler(handler::invoke());

    // handle system tray events
    let builder = builder.on_system_tray_event(handler::system_tray());

    // build the tauri app
    let app = builder.build(context).context(TauriBuildSnafu)?;

    // run the app
    app.run(handler::run());

    Ok(())
}
