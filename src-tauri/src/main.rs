#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use discord_rich_presence::{activity::Activity, DiscordIpc, DiscordIpcClient};
use tauri::Manager;

async fn fetch_status() -> anyhow::Result<serde_json::Value> {
    let url = "http://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002";

    // TODO: create a personal Web API Key at https://steamcommunity.com/dev/apikey
    let key = "<FIXME>";
    // TODO: see "Steam ID" at top left of page at https://store.steampowered.com/account/
    let steamids = "<FIXME>";

    let client = reqwest::Client::new();

    let request = client.get(url).query(&[("key", key), ("steamids", steamids)]).build()?;
    println!("request: {:#?}", request);

    let response = client.execute(request).await?;
    println!("response: {:#?}", response);

    let json: serde_json::Value = response.json().await?;
    println!("json: {:#?}", json);

    Ok(json)
}

#[tauri::command]
async fn fetch_status_command() -> Result<(), String> {
    fetch_status().await.map_err(|err| err.to_string())?;
    Ok(())  
  }

fn main() -> anyhow::Result<()> {
    let system_tray_menu = tauri::SystemTrayMenu::new()
        .add_item(tauri::CustomMenuItem::new("toggle-hide-show", "Hide"))
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(tauri::CustomMenuItem::new("exit-app", "Quit"));

    let system_tray = tauri::SystemTray::new().with_menu(system_tray_menu);

    #[allow(unused_mut)]
    let mut app = tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            tauri::SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "exit-app" => {
                    app.exit(exitcode::OK);
                },
                "toggle-hide-show" => {
                    let window = app.get_window("main").unwrap();
                    let new_title = if window.is_visible().unwrap() {
                        window.hide().unwrap();
                        "Show"
                    } else {
                        window.show().unwrap();
                        "Hide"
                    };
                    app.tray_handle()
                        .get_item("toggle-hide-show")
                        .set_title(new_title)
                        .unwrap();
                },
                _ => {},
            },
            _ => {},
        })
        .invoke_handler(tauri::generate_handler![fetch_status_command])
        .build(tauri::generate_context!())?;

    // hide app from Dock on macOS
    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    let mut client = {
        // get a client id by registering an application at https://discord.com/developers/applications
        let client_id = "<FIXME>";
        DiscordIpcClient::new(client_id)
    }
    .map_err(|err| anyhow::Error::msg(err.to_string()))?;

    app.run(move |app_handle, e| match e {
        tauri::RunEvent::Exit => {
            client.close().unwrap();
        },
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        },
        tauri::RunEvent::WindowEvent {
            label,
            event: tauri::WindowEvent::CloseRequested { api, .. },
            ..
        } => {
            if label == "main" {
                api.prevent_close();
                let window = app_handle.get_window("main").unwrap();
                let new_title = {
                    window.hide().unwrap();
                    "Show"
                };
                app_handle
                    .tray_handle()
                    .get_item("toggle-hide-show")
                    .set_title(new_title)
                    .unwrap();
            }
        },
        tauri::RunEvent::Ready => {
            let json = tauri::async_runtime::block_on(fetch_status()).unwrap();
            let mut state = String::from("Not playing anything");
            if let Ok(gameextrainfo) = json
                .get("response")
                .and_then(|json| json.get("players"))
                .and_then(|json| json.get(0))
                .and_then(|json| json.get("gameextrainfo"))
                .ok_or_else(|| serde::de::Error::custom("foo"))
                .and_then(|json| serde_json::from_value::<String>(json.clone()))
            {
                state = format!("Playing {}", gameextrainfo);
            }
            client.connect().unwrap();
            client.set_activity(Activity::new().state(&state)).unwrap();
        },
        _ => {},
    });

    Ok(())
}
