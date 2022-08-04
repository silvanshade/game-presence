fn main() -> Result<(), Box<dyn std::error::Error>> {
    tauri_conf_csp_update()?;
    tauri_build::build();
    Ok(())
}

fn csp_tauri_schema(file_name: &str) -> String {
    let schema = "tauri://localhost/assets";
    format!("{}/{}", schema, file_name)
}

fn tauri_conf_csp_update() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;

    let assets_path = ["..", "dist", "assets"].join(&std::path::MAIN_SEPARATOR.to_string());
    println!("cargo:rerun-if-changed={}", assets_path);

    let tauri_conf_json_name = "tauri.conf.json";
    let tauri_conf_json_path = std::path::Path::new(tauri_conf_json_name);
    let tauri_conf_json_meta = std::fs::metadata(tauri_conf_json_path)?;

    let tauri_conf_json_atime = filetime::FileTime::from_last_access_time(&tauri_conf_json_meta);
    let tauri_conf_json_mtime = filetime::FileTime::from_last_modification_time(&tauri_conf_json_meta);

    let tauri_conf_json_file = std::fs::OpenOptions::new().read(true).open(tauri_conf_json_path)?;

    let tauri_conf_json_reader = std::io::BufReader::new(tauri_conf_json_file);
    let mut tauri_conf_json = serde_json::from_reader::<_, serde_json::Value>(tauri_conf_json_reader)?;

    let csp_pointer = r"/tauri/security/csp";
    let csp = tauri_conf_json.pointer_mut(csp_pointer).expect(&format!(
        r#"property for json pointer "{}" does not exist in "{}""#,
        csp_pointer, tauri_conf_json_name,
    ));

    let mut images = Vec::<String>::new();
    let mut styles = Vec::<String>::new();

    let images_rx = regex::Regex::new(r"\.[a-z0-9]+\.(?:svg)$")?;
    let styles_rx = regex::Regex::new(r"\.[a-z0-9]+\.(?:css)$")?;

    let assets_dir = std::fs::read_dir(assets_path)?;

    for entry in assets_dir {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name = file_name.to_str();
        match file_name {
            Some(file_name) if images_rx.is_match(file_name) => images.push(csp_tauri_schema(file_name)),
            Some(file_name) if styles_rx.is_match(file_name) => styles.push(csp_tauri_schema(file_name)),
            _ => {},
        }
    }

    let img_src = format!("img-src {}", images.join(" "));
    let style_src = format!("style-src {}", styles.join(" "));

    *csp = [img_src, style_src].join("; ").into();

    let mut tauri_conf_json_file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(tauri_conf_json_path)?;
    tauri_conf_json_file.write_all(serde_json::to_string_pretty(&tauri_conf_json)?.as_bytes())?;

    filetime::set_file_atime(tauri_conf_json_path, tauri_conf_json_atime)?;
    filetime::set_file_mtime(tauri_conf_json_path, tauri_conf_json_mtime)?;

    Ok(())
}
