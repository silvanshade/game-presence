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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub activity: Activity,
    pub services: Services,
    pub games: Games,
}

impl Config {
    const FILE_NAME: &str = "config.json";

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

    pub async fn load() -> Result<Self, Error> {
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
                config.save().await?;
                config
            },
            Ok(config) => config,
        };
        Ok(config)
    }

    pub async fn save(&self) -> Result<(), Error> {
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

impl Config {
    pub fn synchronize_with_gui(&mut self, gui: &crate::app::model::Gui) {
        self.services.synchronize_with_gui(gui);
        self.activity.synchronize_with_gui(gui);
        self.games.synchronize_with_gui(gui);
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub polling_active: bool,
    pub discord_display_presence: bool,
    pub games_require_whitelisting: bool,
    pub service_priorities: Vec<ServicePrioritiesEntry>,
}

impl Activity {
    pub fn synchronize_with_gui(&mut self, gui: &crate::app::model::Gui) {
        let activity = &gui.activity;
        self.polling_active = activity.polling_active;
        self.discord_display_presence = activity.discord_display_presence;
        self.games_require_whitelisting = activity.games_require_whitelisting;
        self.service_priorities = activity.service_priorities.clone();
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Services {
    pub nintendo: self::service::Nintendo,
    pub playstation: self::service::Playstation,
    pub steam: self::service::Steam,
    pub twitch: self::service::Twitch,
    pub xbox: self::service::Xbox,
}

impl Services {
    fn synchronize_with_gui(&mut self, gui: &crate::app::model::Gui) {
        self.nintendo.synchronize_with_gui(gui);
        self.playstation.synchronize_with_gui(gui);
        self.steam.synchronize_with_gui(gui);
        self.twitch.synchronize_with_gui(gui);
        self.xbox.synchronize_with_gui(gui);
    }
}

pub mod service {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Nintendo {
        pub disclaimer_acknowledged: bool,
        pub enabled: bool,
        pub assets_priorities: Vec<super::AssetSourceEntry>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<self::nintendo::Data>,
    }

    impl Nintendo {
        pub fn synchronize_with_gui(&mut self, gui: &crate::app::model::Gui) {
            let nintendo = &gui.services.nintendo;
            self.disclaimer_acknowledged = nintendo.disclaimer_acknowledged;
            self.enabled = nintendo.enabled;
            self.assets_priorities = nintendo.assets_priorities.clone();
        }
    }

    impl Default for self::Nintendo {
        fn default() -> Self {
            let disclaimer_acknowledged = bool::default();
            let enabled = bool::default();
            let assets_priorities = vec![super::AssetSourceEntry::default()];
            let data = Option::default();
            Self {
                disclaimer_acknowledged,
                enabled,
                assets_priorities,
                data,
            }
        }
    }

    pub mod nintendo {
        use serde::{Deserialize, Serialize};

        #[derive(Clone, Debug, Deserialize, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Data {}
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Playstation {
        pub enabled: bool,
        pub assets_priorities: Vec<super::AssetSourceEntry>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<self::playstation::Data>,
    }

    impl Playstation {
        pub fn synchronize_with_gui(&mut self, gui: &crate::app::model::Gui) {
            let playstation = &gui.services.playstation;
            self.enabled = playstation.enabled;
            self.assets_priorities = playstation.assets_priorities.clone();
        }
    }

    impl Default for self::Playstation {
        fn default() -> Self {
            let enabled = bool::default();
            let assets_priorities = vec![super::AssetSourceEntry::default()];
            let data = Option::default();
            Self {
                enabled,
                assets_priorities,
                data,
            }
        }
    }

    pub mod playstation {
        use serde::{Deserialize, Serialize};

        #[derive(Clone, Debug, Deserialize, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Data {}
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Steam {
        pub enabled: bool,
        pub assets_priorities: Vec<super::AssetSourceEntry>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<self::steam::Data>,
    }

    impl Steam {
        pub fn synchronize_with_gui(&mut self, gui: &crate::app::model::Gui) {
            let steam = &gui.services.steam;
            self.enabled = steam.enabled;
            self.assets_priorities = steam.assets_priorities.clone();
        }
    }

    impl Default for self::Steam {
        fn default() -> Self {
            let enabled = bool::default();
            let assets_priorities = vec![super::AssetSourceEntry::default()];
            let data = Option::default();
            Self {
                enabled,
                assets_priorities,
                data,
            }
        }
    }

    pub mod steam {
        use serde::{Deserialize, Serialize};

        #[derive(Clone, Debug, Deserialize, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Data {
            pub api_key: String,
        }
    }

    #[derive(Clone, Debug, Default, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Twitch {
        pub enabled: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<self::twitch::Data>,
    }

    impl Twitch {
        pub fn synchronize_with_gui(&mut self, gui: &crate::app::model::Gui) {
            let twitch = &gui.services.twitch;
            self.enabled = twitch.enabled;
        }
    }

    pub mod twitch {
        use serde::{Deserialize, Serialize};

        #[derive(Clone, Debug, Deserialize, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Data {}
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Xbox {
        pub enabled: bool,
        pub assets_priorities: Vec<super::AssetSourceEntry>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<self::xbox::Data>,
    }

    impl Xbox {
        pub fn synchronize_with_gui(&mut self, gui: &crate::app::model::Gui) {
            let xbox = &gui.services.xbox;
            self.enabled = xbox.enabled;
            self.assets_priorities = xbox.assets_priorities.clone();
        }
    }

    impl Default for self::Xbox {
        fn default() -> Self {
            let enabled = bool::default();
            let assets_priorities = vec![super::AssetSourceEntry::default()];
            let data = Option::default();
            Self {
                enabled,
                assets_priorities,
                data,
            }
        }
    }

    pub mod xbox {
        use serde::{Deserialize, Serialize};

        #[derive(Clone, Debug, Deserialize, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Data {}
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AssetSourceEntry {
    #[default]
    Native,
    Twitch,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ServicePrioritiesEntry {
    Nintendo,
    Playstation,
    Steam,
    Xbox,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Games {}

impl Games {
    fn synchronize_with_gui(&mut self, _gui: &crate::app::model::Gui) {
    }
}
