use serde::{Deserialize, Serialize};
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    IoError { source: tokio::io::Error },
    SerdeJsonError { source: serde_json::Error },
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub steam_id: String,
    pub steam_web_api_user_key: String,
    pub discord_api_client_id: String,
}

pub async fn config_dir_path() -> Result<std::path::PathBuf, Error> {
    let config_dir = directories::ProjectDirs::from(
        crate::app::metadata::project::QUALIFIER,
        crate::app::metadata::project::ORGANIZATION,
        crate::app::metadata::project::APPLICATION,
    )
    .map(|dirs| dirs.config_dir().to_owned())
    .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "configuration directory not found"))
    .context(IoSnafu)?;
    tokio::fs::create_dir_all(&config_dir).await.context(IoSnafu)?;
    Ok(config_dir)
}

pub async fn config_file_path() -> Result<std::path::PathBuf, Error> {
    let config_file_name = "config.json".into();
    let config_file_path = [config_dir_path().await?, config_file_name].iter().collect();
    Ok(config_file_path)
}

pub async fn config_file() -> Result<tokio::fs::File, Error> {
    let config_file_path = config_file_path().await?;
    let config_file = tokio::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(config_file_path)
        .await
        .context(IoSnafu)?;
    Ok(config_file)
}

pub async fn config_data() -> Result<Config, Error> {
    use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

    let mut config_file = config_file().await?;
    let mut config_text = String::new();
    config_file.read_to_string(&mut config_text).await.context(IoSnafu)?;

    if config_text.is_empty() {
        tracing::info!(r#""config.json" is empty; writing defaults to file"#);
        let config_data = Config::default();
        let config_data = serde_json::ser::to_vec(&config_data).context(SerdeJsonSnafu)?;
        config_file.write_all(&config_data).await.context(IoSnafu)?;
        config_file.flush().await.context(IoSnafu)?;
        config_file.seek(std::io::SeekFrom::Start(0)).await.context(IoSnafu)?;
        config_file.read_to_string(&mut config_text).await.context(IoSnafu)?;
    }

    serde_json::from_str(&config_text).context(SerdeJsonSnafu)
}
