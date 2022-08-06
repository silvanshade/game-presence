use snafu::prelude::*;

pub mod events;
pub mod gui;
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

