use snafu::{prelude::*, Backtrace};

pub mod command;
mod handler;
pub mod ipc;
pub mod model;
mod tray;

pub use model::Model;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to build the tauri app"))]
    TauriBuild { backtrace: Backtrace, source: tauri::Error },
    #[snafu(display("Failed to set title {title} for menu item {id}"))]
    TauriMenuItemSetTitle {
        backtrace: Backtrace,
        source: tauri::Error,
        id: String,
        title: String,
    },
    #[snafu(display("Failed to get window {label}"))]
    TauriGetWindow { backtrace: Backtrace, label: String },
    #[snafu(display("Failed to hide window {label}"))]
    TauriWindowHide {
        backtrace: Backtrace,
        source: tauri::Error,
        label: String,
    },
    #[snafu(display("Failed to get visibility for window {label}"))]
    TauriWindowIsVisible {
        backtrace: Backtrace,
        source: tauri::Error,
        label: String,
    },
    #[snafu(display("Failed to show window {label}"))]
    TauriWindowShow {
        backtrace: Backtrace,
        source: tauri::Error,
        label: String,
    },
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

pub(crate) fn init(model: crate::app::Model, tx: tokio::sync::oneshot::Sender<tauri::AppHandle>) -> Result<(), Error> {
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
    let builder = builder.manage(model.clone());

    // configure tauri plugins
    let builder = {
        let schema = crate::app::ipc::schema(model);
        let plugin = tauri_plugin_graphql_ipc::init(schema);
        builder.plugin(plugin)
    };

    // create the main window
    let builder = builder.setup(|app| {
        tauri::WindowBuilder::new(app, "main", tauri::WindowUrl::App("index.html".into()))
            .title("game-presence")
            .inner_size(760f64, 386f64)
            .fullscreen(false)
            .resizable(false)
            .disable_file_drop_handler() // NOTE: needed on windows for vuedraggable to work
            .build()?;
        Ok(())
    });

    // build the tauri app
    let app = builder.build(context).context(TauriBuildSnafu)?;

    tx.send(app.handle()).unwrap();

    // run the app
    app.run(handler::run());

    Ok(())
}
