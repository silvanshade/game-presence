use serde::{Deserialize, Serialize};
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    IoError { source: tokio::io::Error },
    SerdeJsonDeserializeError { source: serde_json::Error },
    SerdeJsonSerializeError { source: serde_json::Error },
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub discord_api_client_id: Option<String>,
    pub steam_id: Option<String>,
    pub steam_web_api_user_key: Option<String>,
}

impl Config {
    const FILE_NAME: &'static str = "config.json";

    async fn path_find() -> Result<std::path::PathBuf, Error> {
        let config_dir = directories::ProjectDirs::from(
            crate::app::metadata::project::QUALIFIER,
            crate::app::metadata::project::ORGANIZATION,
            crate::app::metadata::project::APPLICATION,
        )
        .map(|dirs| dirs.config_dir().to_owned())
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "configuration directory not found"))
        .context(IoSnafu)?;
        Ok(config_dir)
    }

    async fn file_open() -> Result<tokio::fs::File, Error> {
        let config_dirs_path = Self::path_find().await?;
        tokio::fs::create_dir_all(&config_dirs_path).await.context(IoSnafu)?;
        let config_file_name = Self::FILE_NAME.into();
        let config_file_path = [config_dirs_path, config_file_name]
            .iter()
            .collect::<std::path::PathBuf>();
        let config_file = tokio::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(config_file_path)
            .await
            .context(IoSnafu)?;
        Ok(config_file)
    }

    async fn file_write(&self) -> Result<(), Error> {
        use tokio::io::{AsyncSeekExt, AsyncWriteExt};
        let mut config_file = Self::file_open().await?;
        let config_data = serde_json::ser::to_string_pretty(&self).context(SerdeJsonSerializeSnafu)?;
        let config_data = config_data.as_bytes();
        config_file.seek(std::io::SeekFrom::Start(0)).await.context(IoSnafu)?;
        config_file.write_all(&config_data).await.context(IoSnafu)?;
        config_file.flush().await.context(IoSnafu)?;
        config_file.seek(std::io::SeekFrom::Start(0)).await.context(IoSnafu)?;
        Ok(())
    }

    pub async fn load() -> Result<Self, Error> {
        use tokio::io::AsyncReadExt;

        let mut config_file = Self::file_open().await?;
        let mut config_text = String::new();
        let mut config_data = Self::default();

        config_file.read_to_string(&mut config_text).await.context(IoSnafu)?;

        if config_text.is_empty() {
            #[cfg(feature = "debug")]
            tracing::info!(r#""{}" is empty; writing defaults to file"#, Self::FILE_NAME);
            Self::default().file_write().await?;
            config_file.read_to_string(&mut config_text).await.context(IoSnafu)?;
        }

        let mut config_json =
            serde_json::from_str::<serde_json::Map<_, _>>(&config_text).context(SerdeJsonDeserializeSnafu)?;

        if let Some(value) = config_json.remove("discordApiClientId") {
            config_data.discord_api_client_id = serde_json::from_value(value).context(SerdeJsonDeserializeSnafu)?;
        }
        if let Some(value) = config_json.remove("steamId") {
            config_data.steam_id = serde_json::from_value(value).context(SerdeJsonDeserializeSnafu)?;
        }
        if let Some(value) = config_json.remove("steamWebApiUserKey") {
            config_data.steam_web_api_user_key = serde_json::from_value(value).context(SerdeJsonDeserializeSnafu)?;
        }

        if !config_json.is_empty() {
            #[cfg(feature = "debug")]
            tracing::warn!(r#""config.json" includes spurious data; overwriting"#);
            config_data.file_write().await?;
        }

        Ok(config_data)
    }
}
