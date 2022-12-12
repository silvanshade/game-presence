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
    TokioFsOpenOptions { source: std::io::Error },
    TokioIoReadToString { source: std::io::Error },
    StdFsSyncAll { source: std::io::Error },
    TokioIoWriteAll { source: std::io::Error },
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

    pub async fn init() -> Result<Self, Error> {
        let config = Self::read().await?;
        Ok(config)
    }

    fn file_base() -> Result<std::path::PathBuf, Error> {
        let base = directories::BaseDirs::new().context(DirectoriesBaseDirsNewSnafu)?;
        let mut path = base.config_dir().to_path_buf();
        path.push("game-presence");
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

    pub async fn read() -> Result<Self, Error> {
        use tokio::io::AsyncReadExt;
        Self::file_base_create()?;
        let path = Self::file_path()?;
        let path = path.as_path();
        let mut file = tokio::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(path)
            .await
            .context(TokioFsOpenOptionsSnafu)?;
        let mut json = String::new();
        file.read_to_string(&mut json).await.context(TokioIoReadToStringSnafu)?;
        let value = serde_json::from_str::<Self>(&json);
        let config = match value {
            Err(_) => {
                let config = Self::default();
                config.write().await?;
                config
            },
            Ok(config) => config,
        };
        Ok(config)
    }

    pub async fn write(&self) -> Result<(), Error> {
        use tokio::io::AsyncWriteExt;
        Self::file_base_create()?;
        let path = Self::file_path()?;
        let path = path.as_path();
        let mut file = tokio::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .await
            .context(TokioFsOpenOptionsSnafu)?;
        let json = serde_json::to_vec_pretty(self).context(SerdeJsonToVecSnafu)?;
        file.write_all(&json).await.context(TokioIoWriteAllSnafu)?;
        file.sync_all().await.context(StdFsSyncAllSnafu)?;
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
    pub nintendo: self::service::Nintendo,
    pub playstation: self::service::Playstation,
    pub steam: self::service::Steam,
    pub twitch: self::service::Twitch,
    pub xbox: self::service::Xbox,
}

impl From<crate::app::model::config::Services> for self::Services {
    fn from(services: crate::app::model::config::Services) -> Self {
        let nintendo = services.nintendo.into();
        let playstation = services.playstation.into();
        let steam = services.steam.into();
        let twitch = services.twitch.into();
        let xbox = services.xbox.into();
        Self {
            nintendo,
            playstation,
            steam,
            twitch,
            xbox,
        }
    }
}

pub mod service {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Nintendo {
        pub disclaimer_acknowledged: bool,
        pub enabled: bool,
        pub game_asset_sources: Vec<super::AssetSource>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<self::nintendo::Data>,
    }

    impl Default for self::Nintendo {
        fn default() -> Self {
            let disclaimer_acknowledged = bool::default();
            let enabled = bool::default();
            let game_asset_sources = vec![super::AssetSource::default()];
            let data = Option::default();
            Self {
                disclaimer_acknowledged,
                enabled,
                game_asset_sources,
                data,
            }
        }
    }

    impl From<crate::app::model::config::service::Nintendo> for self::Nintendo {
        fn from(nintendo: crate::app::model::config::service::Nintendo) -> Self {
            let disclaimer_acknowledged = nintendo.disclaimer_acknowledged;
            let enabled = nintendo.enabled;
            let game_asset_sources = nintendo.game_asset_sources;
            let data = nintendo.data.map(Into::into);
            Self {
                disclaimer_acknowledged,
                enabled,
                game_asset_sources,
                data,
            }
        }
    }

    pub mod nintendo {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Data {
            pub username: Option<String>,
        }

        impl From<crate::app::model::config::service::nintendo::Data> for self::Data {
            fn from(data: crate::app::model::config::service::nintendo::Data) -> Self {
                let username = data.username;
                Self { username }
            }
        }
    }

    #[derive(Debug, Default, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Playstation {
        pub enabled: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<self::playstation::Data>,
    }

    impl From<crate::app::model::config::service::Playstation> for self::Playstation {
        fn from(playstation: crate::app::model::config::service::Playstation) -> Self {
            let enabled = playstation.enabled;
            let data = playstation.data.map(Into::into);
            Self { enabled, data }
        }
    }

    pub mod playstation {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Data {
            pub username: Option<String>,
        }

        impl From<crate::app::model::config::service::playstation::Data> for self::Data {
            fn from(data: crate::app::model::config::service::playstation::Data) -> Self {
                let username = data.username;
                Self { username }
            }
        }
    }

    #[derive(Debug, Default, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Steam {
        pub enabled: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<self::steam::Data>,
    }

    impl From<crate::app::model::config::service::Steam> for self::Steam {
        fn from(steam: crate::app::model::config::service::Steam) -> Self {
            let enabled = steam.enabled;
            let data = steam.data.map(Into::into);
            Self { enabled, data }
        }
    }

    pub mod steam {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Data {
            pub id: String,
            pub key: String,
            pub username: String,
        }

        impl From<crate::app::model::config::service::steam::Data> for self::Data {
            fn from(data: crate::app::model::config::service::steam::Data) -> Self {
                let id = data.id;
                let key = data.key;
                let username = data.username;
                Self { id, key, username }
            }
        }
    }

    #[derive(Debug, Default, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Twitch {
        pub enabled: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<self::twitch::Data>,
    }

    impl From<crate::app::model::config::service::Twitch> for self::Twitch {
        fn from(twitch: crate::app::model::config::service::Twitch) -> Self {
            let enabled = twitch.enabled;
            let data = twitch.data.map(Into::into);
            Self { enabled, data }
        }
    }

    pub mod twitch {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Data {
            pub username: String,
        }

        impl From<crate::app::model::config::service::twitch::Data> for self::Data {
            fn from(data: crate::app::model::config::service::twitch::Data) -> Self {
                let username = data.username;
                Self { username }
            }
        }
    }

    #[derive(Debug, Default, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Xbox {
        pub enabled: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<self::xbox::Data>,
    }

    impl From<crate::app::model::config::service::Xbox> for self::Xbox {
        fn from(xbox: crate::app::model::config::service::Xbox) -> Self {
            let enabled = xbox.enabled;
            let data = xbox.data.map(Into::into);
            Self { enabled, data }
        }
    }

    pub mod xbox {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Data {
            pub username: Option<String>,
        }

        impl From<crate::app::model::config::service::xbox::Data> for self::Data {
            fn from(data: crate::app::model::config::service::xbox::Data) -> Self {
                let username = data.username;
                Self { username }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub enum AssetSource {
    #[default]
    Native,
    Twitch,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub polling_active: bool,
    pub discord_display_presence: bool,
    pub games_require_whitelisting: bool,
}

impl From<crate::app::model::config::Activity> for self::Activity {
    fn from(activity: crate::app::model::config::Activity) -> Self {
        let polling_active = activity.polling_active;
        let discord_display_presence = activity.discord_display_presence;
        let games_require_whitelisting = activity.games_require_whitelisting;
        Self {
            polling_active,
            discord_display_presence,
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
