use serde::{Deserialize, Serialize};
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    Io { source: tokio::io::Error },
    SerdeJsonDeserialize { source: serde_json::Error },
    SerdeJsonSerialize { source: serde_json::Error },
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub discord_client_id: Option<String>,
    pub steam_user_id: Option<String>,
    pub steam_user_key: Option<String>,
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

    async fn file_open(read: bool, write: bool, create: bool, truncate: bool) -> Result<tokio::fs::File, Error> {
        let config_dirs_path = Self::path_find().await?;
        tokio::fs::create_dir_all(&config_dirs_path).await.context(IoSnafu)?;
        let config_file_name = Self::FILE_NAME.into();
        let config_file_path = [config_dirs_path, config_file_name]
            .iter()
            .collect::<std::path::PathBuf>();
        let config_file = tokio::fs::OpenOptions::new()
            .read(read)
            .write(write)
            .create(create)
            .truncate(truncate)
            .open(config_file_path)
            .await
            .context(IoSnafu)?;
        Ok(config_file)
    }

    async fn file_read(dst: &mut String) -> Result<usize, Error> {
        use tokio::io::AsyncReadExt;
        let read = true;
        let write = false;
        let create = false;
        let truncate = false;
        Self::file_open(read, write, create, truncate)
            .await?
            .read_to_string(dst)
            .await
            .context(IoSnafu)
    }

    async fn file_write(&self) -> Result<(), Error> {
        use tokio::io::AsyncWriteExt;
        let read = false;
        let write = true;
        let create = true;
        let truncate = true;
        let mut config_file = Self::file_open(read, write, create, truncate).await?;
        let config_data = serde_json::ser::to_string_pretty(&self).context(SerdeJsonSerializeSnafu)?;
        let config_data = config_data.as_bytes();
        config_file.write_all(config_data).await.context(IoSnafu)?;
        config_file.flush().await.context(IoSnafu)?;
        Ok(())
    }

    pub async fn load() -> Result<Self, Error> {
        let mut config_text = String::new();
        let mut config_data = Self::default();

        Self::file_read(&mut config_text).await?;

        if config_text.is_empty() {
            #[cfg(feature = "debug")]
            tracing::info!(r#""{}" is empty; writing defaults to file"#, Self::FILE_NAME);
            config_data.file_write().await?;
            Self::file_read(&mut config_text).await?;
        }

        let config_read = serde_json::from_str::<serde_json::Map<_, _>>(&config_text);

        if let Err(error) = &config_read {
            if !error.is_eof() {
                #[cfg(feature = "debug")]
                tracing::warn!(
                    ?error,
                    r#"error deserializing "{}"; writing defaults to file"#,
                    Self::FILE_NAME
                );
                config_data.file_write().await?;
                Self::file_read(&mut config_text).await?;
            }
        }

        if let Ok(mut config_json) = config_read {
            if let Some(value) = config_json.remove("discordClientId") {
                config_data.discord_client_id = serde_json::from_value(value).context(SerdeJsonDeserializeSnafu)?;
            }
            if let Some(value) = config_json.remove("steamUserId") {
                config_data.steam_user_id = serde_json::from_value(value).context(SerdeJsonDeserializeSnafu)?;
            }
            if let Some(value) = config_json.remove("steamUserKey") {
                config_data.steam_user_key = serde_json::from_value(value).context(SerdeJsonDeserializeSnafu)?;
            }

            if !config_json.is_empty() {
                #[cfg(feature = "debug")]
                tracing::warn!(r#""config.json" includes spurious data; overwriting"#);
                config_data.file_write().await?;
            }
        }

        Ok(config_data)
    }
}
