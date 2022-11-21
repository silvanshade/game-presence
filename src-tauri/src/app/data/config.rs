use serde::{Deserialize, Serialize};
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    DirectoriesBaseDirsNew,
    SerdeJsonFromStr { source: serde_json::Error },
    SerdeJsonFromValue { source: serde_json::Error },
    SerdeJsonToVec { source: serde_json::Error },
    FsCreateDirAll { source: std::io::Error },
    FsMetadata { source: std::io::Error },
    FsOpenOptions { source: std::io::Error },
    FsReadToString { source: std::io::Error },
    FsSyncAll { source: std::io::Error },
    IoWriteAll { source: std::io::Error },
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub profiles: Vec<Profile>,
}

impl Config {
    const FILE_NAME: &str = "config.json";

    pub fn init() -> Result<Self, Error> {
        let config = Self::file_read()?;
        Ok(config)
    }

    fn file_base() -> Result<std::path::PathBuf, Error> {
        let base = directories::BaseDirs::new().context(DirectoriesBaseDirsNewSnafu)?;
        let mut path = base.config_dir().to_path_buf();
        path.push("Game Presence");
        Ok(path)
    }

    fn file_base_create() -> Result<(), Error> {
        let base = Self::file_base()?;
        std::fs::create_dir_all(base).context(FsCreateDirAllSnafu)?;
        Ok(())
    }

    fn file_path() -> Result<std::path::PathBuf, Error> {
        let mut path = Self::file_base()?;
        path.push(Self::FILE_NAME);
        Ok(path)
    }

    pub fn file_read() -> Result<Self, Error> {
        use std::io::Read;
        Self::file_base_create()?;
        let path = Self::file_path()?;
        let path = path.as_path();
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(path)
            .context(FsOpenOptionsSnafu)?;
        let mut json = String::new();
        file.read_to_string(&mut json).context(FsReadToStringSnafu)?;
        let value = serde_json::from_str::<serde_json::Value>(&json).context(SerdeJsonFromStrSnafu);
        let config = match value {
            Err(_) => {
                let config = Self::default();
                config.file_write()?;
                config
            },
            Ok(value) => serde_json::from_value::<Config>(value).context(SerdeJsonFromValueSnafu)?,
        };
        Ok(config)
    }

    pub fn file_write(&self) -> Result<(), Error> {
        use std::io::Write;
        Self::file_base_create()?;
        let path = Self::file_path()?;
        let path = path.as_path();
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .context(FsOpenOptionsSnafu)?;
        let json = serde_json::to_vec(self).context(SerdeJsonToVecSnafu)?;
        file.write_all(&json).context(IoWriteAllSnafu)?;
        file.sync_all().context(FsSyncAllSnafu)?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    pub services: Services,
    pub activity: Activity,
    pub games: Games,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Services {
    pub nintendo: Option<self::service::Nintendo>,
    pub playstation: Option<self::service::Playstation>,
    pub steam: Option<self::service::Steam>,
    pub xbox: Option<self::service::Xbox>,
}

pub mod service {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Nintendo {
        pub enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Playstation {
        pub enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Steam {
        pub enabled: bool,
        pub id: String,
        pub key: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Xbox {
        pub enabled: bool,
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Activity {
    pub discord_display_presence: bool,
    pub twitch_assets_enabled: bool,
    pub games_require_whitelisting: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Games {}
