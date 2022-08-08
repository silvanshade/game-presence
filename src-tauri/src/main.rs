#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use snafu::prelude::*;

pub mod api;
pub mod app;

#[derive(Debug, Snafu)]
enum Error {
    Model {
        source: crate::app::model::Error,
    },
    Tauri {
        source: tauri::Error,
    },
    #[cfg(feature = "debug")]
    TracingSubscriber {
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

fn main() -> Result<(), self::Error> {
    #[cfg(feature = "debug")]
    tracing_subscriber::fmt::try_init().context(TracingSubscriberSnafu)?;

    #[cfg(feature = "debug")]
    tracing::info!("tracing");

    let context = tauri::generate_context!();

    #[allow(unused_mut)]
    let mut app = tauri::Builder::default()
        .manage(crate::app::Model::new().context(ModelSnafu)?)
        .system_tray(crate::app::gui::make_system_tray())
        .on_system_tray_event(crate::app::handler::system_tray)
        .invoke_handler(tauri::generate_handler![
            crate::app::commands::model_config_load,
            crate::app::commands::model_discord_connect,
            crate::app::commands::get_built_info,
            crate::app::commands::get_settings,
            crate::app::commands::set_settings,
        ])
        .build(context)
        .context(TauriSnafu)?;

    // hide app from Dock on macOS
    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    app.run(crate::app::handler::run);

    Ok(())
}
