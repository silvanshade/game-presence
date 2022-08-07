use snafu::prelude::*;

#[derive(Debug, Snafu)]
enum Error {
    BuiltWriteBuiltFile {
        source: std::io::Error,
    },
    FileTimeSetATime {
        source: std::io::Error,
    },
    FileTimeSetMTime {
        source: std::io::Error,
    },

    FsDirEntry {
        source: std::io::Error,
    },
    FsMetaData {
        source: std::io::Error,
        tauri_conf_json_path: std::path::PathBuf,
    },
    FsOpenRead {
        source: std::io::Error,
        tauri_conf_json_path: std::path::PathBuf,
    },
    FsOpenWrite {
        source: std::io::Error,
        tauri_conf_json_path: std::path::PathBuf,
    },
    FsReadDir {
        source: std::io::Error,
        assets_path: std::path::PathBuf,
    },
    FsWriteAll {
        source: std::io::Error,
        tauri_conf_json_path: std::path::PathBuf,
        tauri_conf_json: serde_json::Value,
    },
    Regex {
        source: regex::Error,
    },
    SerdeJsonDeserialize {
        source: serde_json::Error,
        tauri_conf_json_path: std::path::PathBuf,
    },
    SerdeJsonToStringPretty {
        source: serde_json::Error,
        tauri_conf_json: serde_json::Value,
    },
    TauriCspPointer {
        csp_pointer: String,
    },
}

fn main() -> Result<(), self::Error> {
    tauri_conf_csp_update()?;
    collect_metadata()?;
    tauri_build::build();
    Ok(())
}

fn collect_metadata() -> Result<(), self::Error> {
    let mut opts = built::Options::default();
    opts.set_dependencies(true);
    let src = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let dst = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("built.rs");
    built::write_built_file_with_opts(&opts, src.as_ref(), &dst).context(BuiltWriteBuiltFileSnafu)?;
    Ok(())
}

fn csp_tauri_schema(file_name: &str) -> String {
    let schema = "tauri://localhost/assets";
    format!("{}/{}", schema, file_name)
}

fn tauri_conf_csp_update() -> Result<(), self::Error> {
    use std::io::Write;

    let assets_path = ["..", "dist", "assets"].join(&std::path::MAIN_SEPARATOR.to_string());
    println!("cargo:rerun-if-changed={}", assets_path);

    let tauri_conf_json_name = "tauri.conf.json";
    let tauri_conf_json_path = std::path::Path::new(tauri_conf_json_name);
    let tauri_conf_json_meta =
        std::fs::metadata(tauri_conf_json_path).context(FsMetaDataSnafu { tauri_conf_json_path })?;

    let tauri_conf_json_atime = filetime::FileTime::from_last_access_time(&tauri_conf_json_meta);
    let tauri_conf_json_mtime = filetime::FileTime::from_last_modification_time(&tauri_conf_json_meta);

    let tauri_conf_json_file = std::fs::OpenOptions::new()
        .read(true)
        .open(tauri_conf_json_path)
        .context(FsOpenReadSnafu { tauri_conf_json_path })?;

    let tauri_conf_json_reader = std::io::BufReader::new(tauri_conf_json_file);
    let mut tauri_conf_json = serde_json::from_reader::<_, serde_json::Value>(tauri_conf_json_reader)
        .context(SerdeJsonDeserializeSnafu { tauri_conf_json_path })?;

    let csp_pointer = r"/tauri/security/csp";
    let csp = tauri_conf_json
        .pointer_mut(csp_pointer)
        .context(TauriCspPointerSnafu { csp_pointer })?;

    let mut images = Vec::<String>::new();
    let mut styles = Vec::<String>::new();

    let images_rx = regex::Regex::new(r"\.[a-z0-9]+\.(?:svg)$").context(RegexSnafu)?;
    let styles_rx = regex::Regex::new(r"\.[a-z0-9]+\.(?:css)$").context(RegexSnafu)?;

    let assets_dir = std::fs::read_dir(assets_path.clone()).context(FsReadDirSnafu { assets_path })?;

    for entry in assets_dir {
        let entry = entry.context(FsDirEntrySnafu)?;
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
        .open(tauri_conf_json_path)
        .context(FsOpenWriteSnafu { tauri_conf_json_path })?;
    tauri_conf_json_file
        .write_all(
            serde_json::to_string_pretty(&tauri_conf_json)
                .context(SerdeJsonToStringPrettySnafu {
                    tauri_conf_json: tauri_conf_json.clone(),
                })?
                .as_bytes(),
        )
        .context(FsWriteAllSnafu {
            tauri_conf_json_path,
            tauri_conf_json,
        })?;

    filetime::set_file_atime(tauri_conf_json_path, tauri_conf_json_atime).context(FileTimeSetATimeSnafu)?;
    filetime::set_file_mtime(tauri_conf_json_path, tauri_conf_json_mtime).context(FileTimeSetMTimeSnafu)?;

    Ok(())
}
