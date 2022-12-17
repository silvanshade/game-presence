use snafu::prelude::*;

pub mod command;
mod handler;
pub mod ipc;
pub mod model;
mod tray;

pub use model::Model;

#[derive(Debug, Snafu)]
pub enum Error {
    TauriBuild { source: tauri::Error },
    TauriMenuItemSetTitle { source: tauri::Error },
    TauriGetWindow,
    TauriWindowHide { source: tauri::Error },
    TauriWindowIsVisible { source: tauri::Error },
    TauriWindowShow { source: tauri::Error },
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

pub(crate) fn init(state: crate::app::Model) -> Result<(), Error> {
    let context = tauri::generate_context!();

    // create the default builder
    let builder = tauri::Builder::default();

    // configure the system tray
    let builder = builder.system_tray(self::make_system_tray());

    // handle RPC command invocations
    let builder = builder.invoke_handler(handler::invoke());

    // handle system tray events
    let builder = builder.on_system_tray_event(handler::system_tray());

    // configure the app state
    let builder = builder.manage(state.clone());

    // configure tauri plugins
    let builder = {
        let schema = crate::app::ipc::schema(state);
        let plugin = tauri_plugin_graphql_ipc::init(schema);
        builder.plugin(plugin)
    };

    // create the main window
    let builder = builder.setup(|app| {
        tauri::WindowBuilder::new(app, "main", tauri::WindowUrl::App("index.html".into()))
            .title("game-presence")
            .inner_size(762f64, 400f64)
            .fullscreen(false)
            .resizable(false)
            .disable_file_drop_handler() // NOTE: needed on windows for vuedraggable to work
            .build()?;
        Ok(())
    });

    // build the tauri app
    let app = builder.build(context).context(TauriBuildSnafu)?;

    // run the app
    app.run(handler::run());

    Ok(())
}
