use serde::{Deserialize, Serialize};
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    DirectoriesBaseDirsNew,
    SerdeJsonFromStr { source: serde_json::Error },
    SerdeJsonFromValue { source: serde_json::Error },
    SerdeJsonToVec { source: serde_json::Error },
    StdFsCreateDirAll { source: std::io::Error },
    StdFsMetadata { source: std::io::Error },
    StdFsOpenOptions { source: std::io::Error },
    StdFsReadToString { source: std::io::Error },
    StdFsSyncAll { source: std::io::Error },
    StdIoWriteAll { source: std::io::Error },
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub services: Services,
    pub activity: Activity,
    pub games: Games,
}

impl Config {
    const FILE_NAME: &str = "config.json";

    pub fn init() -> Result<Self, Error> {
        let config = Self::read()?;
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
        std::fs::create_dir_all(base).context(StdFsCreateDirAllSnafu)?;
        Ok(())
    }

    fn file_path() -> Result<std::path::PathBuf, Error> {
        let mut path = Self::file_base()?;
        path.push(Self::FILE_NAME);
        Ok(path)
    }

    pub fn read() -> Result<Self, Error> {
        use std::io::Read;
        Self::file_base_create()?;
        let path = Self::file_path()?;
        let path = path.as_path();
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(path)
            .context(StdFsOpenOptionsSnafu)?;
        let mut json = String::new();
        file.read_to_string(&mut json).context(StdFsReadToStringSnafu)?;
        let value = serde_json::from_str::<Self>(&json);
        let config = match value {
            Err(_) => {
                let config = Self::default();
                config.write()?;
                config
            },
            Ok(config) => config,
        };
        Ok(config)
    }

    pub fn write(&self) -> Result<(), Error> {
        use std::io::Write;
        Self::file_base_create()?;
        let path = Self::file_path()?;
        let path = path.as_path();
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .context(StdFsOpenOptionsSnafu)?;
        let json = serde_json::to_vec_pretty(self).context(SerdeJsonToVecSnafu)?;
        file.write_all(&json).context(StdIoWriteAllSnafu)?;
        file.sync_all().context(StdFsSyncAllSnafu)?;
        Ok(())
    }
}

impl From<crate::app::model::Config> for Config {
    fn from(state: crate::app::model::Config) -> Self {
        let services = state.services.into();
        let activity = state.activity.into();
        let games = state.games.into();
        Self {
            services,
            activity,
            games,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Services {
    pub nintendo: Option<self::service::Nintendo>,
    pub playstation: Option<self::service::Playstation>,
    pub steam: Option<self::service::Steam>,
    pub xbox: Option<self::service::Xbox>,
}

impl From<crate::app::model::config::Services> for self::Services {
    fn from(services: crate::app::model::config::Services) -> Self {
        let nintendo = services.nintendo.map(Into::into);
        let playstation = services.playstation.map(Into::into);
        let steam = services.steam.map(Into::into);
        let xbox = services.xbox.map(Into::into);
        Self {
            nintendo,
            playstation,
            steam,
            xbox,
        }
    }
}

pub mod service {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Nintendo {
        pub enabled: bool,
    }

    impl From<crate::app::model::config::service::Nintendo> for self::Nintendo {
        fn from(nintendo: crate::app::model::config::service::Nintendo) -> Self {
            let enabled = nintendo.enabled;
            Self { enabled }
        }
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Playstation {
        pub enabled: bool,
    }

    impl From<crate::app::model::config::service::Playstation> for self::Playstation {
        fn from(playstation: crate::app::model::config::service::Playstation) -> Self {
            let enabled = playstation.enabled;
            Self { enabled }
        }
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Steam {
        pub enabled: bool,
        pub data: Option<SteamData>,
    }

    impl From<crate::app::model::config::service::Steam> for self::Steam {
        fn from(steam: crate::app::model::config::service::Steam) -> Self {
            let enabled = steam.enabled;
            let data = steam.data.map(Into::into);
            Self { enabled, data }
        }
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SteamData {
        pub id: String,
        pub key: String,
    }

    impl From<crate::app::model::config::service::SteamData> for self::SteamData {
        fn from(data: crate::app::model::config::service::SteamData) -> Self {
            let id = data.id;
            let key = data.key;
            Self { id, key }
        }
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Xbox {
        pub enabled: bool,
    }

    impl From<crate::app::model::config::service::Xbox> for self::Xbox {
        fn from(xbox: crate::app::model::config::service::Xbox) -> Self {
            let enabled = xbox.enabled;
            Self { enabled }
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub discord_display_presence: bool,
    pub twitch_assets_enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitch_access_token: Option<String>,
    pub games_require_whitelisting: bool,
}

impl From<crate::app::model::config::Activity> for self::Activity {
    fn from(activity: crate::app::model::config::Activity) -> Self {
        let discord_display_presence = activity.discord_display_presence;
        let twitch_assets_enabled = activity.twitch_assets_enabled;
        let twitch_access_token = activity.twitch_access_token;
        let games_require_whitelisting = activity.games_require_whitelisting;
        Self {
            discord_display_presence,
            twitch_assets_enabled,
            twitch_access_token,
            games_require_whitelisting,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Games {}

impl From<crate::app::model::config::Games> for self::Games {
    fn from(_games: crate::app::model::config::Games) -> Self {
        Self {}
    }
}
