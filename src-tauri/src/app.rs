#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    MenuItemSetTitle { source: tauri::Error },
    TauriBuild { source: tauri::Error },
    WindowHide { source: tauri::Error },
    WindowIsVisible { source: tauri::Error },
    WindowShow { source: tauri::Error },
}

mod menu {
    pub mod tray {
        pub mod quit {
            pub const ID: &str = "quit";
            pub const TITLE: &str = "quit";
        }

        pub mod visibility {
            pub const ID: &str = "visibility";
            pub mod hide {
                pub const TITLE: &str = "hide";
            }
            pub mod show {
                pub const TITLE: &str = "show";
            }
        }
    }
}

mod handler {
    pub fn invoke<R: tauri::Runtime>() -> impl Fn(tauri::Invoke<R>) {
        tauri::generate_handler![]
    }

    pub fn run<R: tauri::Runtime>() -> impl FnMut(&tauri::AppHandle<R>, tauri::RunEvent) {
        use tauri::{RunEvent, WindowEvent};
        |app, run_event| match run_event {
            RunEvent::WindowEvent {
                label,
                event: WindowEvent::CloseRequested { api, .. },
                ..
            } if label == "main" => {
                api.prevent_close();
                crate::app::window::main::toggle_visibility(app).unwrap();
            },
            _ => {},
        }
    }

    pub fn system_tray<R: tauri::Runtime>() -> impl Fn(&tauri::AppHandle<R>, tauri::SystemTrayEvent) {
        use tauri::SystemTrayEvent;
        |app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                crate::app::menu::tray::quit::ID => {
                    let code = proc_exit::Code::SUCCESS;
                    code.process_exit();
                },
                crate::app::menu::tray::visibility::ID => crate::app::window::main::toggle_visibility(app).unwrap(),
                _ => {},
            },
            _ => {},
        }
    }
}

mod window {
    pub mod main {
        use snafu::prelude::*;
        pub fn toggle_visibility<R: tauri::Runtime>(app: &tauri::AppHandle<R>) -> Result<(), crate::app::Error> {
            use tauri::Manager;
            let window = app.get_window("main").expect(r#"failed to get "main" window"#);
            let item = app.tray_handle().get_item(crate::app::menu::tray::visibility::ID);
            if window.is_visible().context(crate::app::WindowIsVisibleSnafu)? {
                item.set_title(crate::app::menu::tray::visibility::show::TITLE)
                    .context(crate::app::MenuItemSetTitleSnafu)?;
                window.hide().context(crate::app::WindowHideSnafu)?;
            } else {
                item.set_title(crate::app::menu::tray::visibility::hide::TITLE)
                    .context(crate::app::MenuItemSetTitleSnafu)?;
                window.show().context(crate::app::WindowShowSnafu)?;
            }
            Ok(())
        }
    }
}

pub(crate) fn init() -> Result<(), Error> {
    let context = tauri::generate_context!();

    let system_tray_menu = tauri::SystemTrayMenu::new()
        .add_item(tauri::CustomMenuItem::new(
            crate::app::menu::tray::visibility::ID,
            crate::app::menu::tray::visibility::hide::TITLE,
        ))
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(tauri::CustomMenuItem::new(
            crate::app::menu::tray::quit::ID,
            crate::app::menu::tray::quit::TITLE,
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
