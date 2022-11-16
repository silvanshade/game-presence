#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub(crate) enum Error {
    TauriBuild { source: tauri::Error },
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

mod handler {
    pub(super) fn invoke<R: tauri::Runtime>() -> impl Fn(tauri::Invoke<R>) {
        tauri::generate_handler![super::greet]
    }

    pub(super) fn run<R: tauri::Runtime>() -> impl FnMut(&tauri::AppHandle<R>, tauri::RunEvent) {
        |_app, _event| {}
    }
}

pub(crate) fn init() -> Result<(), Error> {
    let context = tauri::generate_context!();
    let app = tauri::Builder::default()
        .invoke_handler(handler::invoke())
        .build(context)
        .context(TauriBuildSnafu)?;
    app.run(handler::run());
    Ok(())
}
