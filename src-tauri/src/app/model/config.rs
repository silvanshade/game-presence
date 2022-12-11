use serde::{Deserialize, Serialize};
use snafu::prelude::*;
use std::sync::Arc;
use tokio::sync::{
    watch::{Receiver, Sender},
    Mutex,
    RwLock,
};

#[derive(Debug, Snafu)]
pub enum Error {
    ConfigInit { source: crate::app::data::config::Error },
    ConfigWrite { source: crate::app::data::config::Error },
    TauriSpawnBlocking { source: tauri::Error },
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub services: Services,
    pub activity: Activity,
    pub games: Games,
}

impl Config {
    pub async fn load() -> Result<Self, Error> {
        let config = crate::app::data::Config::init().await.context(ConfigInitSnafu)?;
        config.try_into()
    }

    // pub async fn save(self) -> Result<(), Error> {
    //     let config = Into::<crate::app::data::Config>::into(self);
    //     let handle = tauri::async_runtime::spawn_blocking(move ||
    // config.write().context(ConfigWriteSnafu));     let result =
    // handle.await.context(TauriSpawnBlockingSnafu)?;     result
    // }
}

impl TryFrom<crate::app::data::Config> for Config {
    type Error = Error;

    fn try_from(config: crate::app::data::Config) -> Result<Self, Error> {
        let services = config.services.try_into()?;
        let activity = config.activity.try_into()?;
        let games = config.games.try_into()?;
        Ok(Self {
            services,
            activity,
            games,
        })
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Services {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nintendo: Option<self::service::Nintendo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playstation: Option<self::service::Playstation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steam: Option<self::service::Steam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xbox: Option<self::service::Xbox>,
}

impl TryFrom<crate::app::data::config::Services> for self::Services {
    type Error = Error;

    fn try_from(services: crate::app::data::config::Services) -> Result<Self, Error> {
        let nintendo = services.nintendo.map(TryInto::try_into).transpose()?;
        let playstation = services.playstation.map(TryInto::try_into).transpose()?;
        let steam = services.steam.map(TryInto::try_into).transpose()?;
        let xbox = services.xbox.map(TryInto::try_into).transpose()?;
        Ok(Self {
            nintendo,
            playstation,
            steam,
            xbox,
        })
    }
}

pub mod service {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Nintendo {
        pub disclaimer_acknowledged: bool,
        pub enabled: bool,
        pub data: Option<NintendoData>,
    }

    impl TryFrom<crate::app::data::config::service::Nintendo> for self::Nintendo {
        type Error = super::Error;

        fn try_from(nintendo: crate::app::data::config::service::Nintendo) -> Result<Self, Self::Error> {
            let disclaimer_acknowledged = nintendo.disclaimer_acknowledged;
            let enabled = nintendo.enabled;
            let data = nintendo.data.map(TryInto::try_into).transpose()?;
            Ok(Self {
                disclaimer_acknowledged,
                enabled,
                data,
            })
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NintendoData {
        pub username: Option<String>,
    }

    impl TryFrom<crate::app::data::config::service::NintendoData> for self::NintendoData {
        type Error = super::Error;

        fn try_from(data: crate::app::data::config::service::NintendoData) -> Result<Self, Self::Error> {
            let username = data.username;
            Ok(Self { username })
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Playstation {
        pub enabled: bool,
        pub data: Option<PlaystationData>,
    }

    impl TryFrom<crate::app::data::config::service::Playstation> for self::Playstation {
        type Error = super::Error;

        fn try_from(playstation: crate::app::data::config::service::Playstation) -> Result<Self, Self::Error> {
            let enabled = playstation.enabled;
            let data = playstation.data.map(TryInto::try_into).transpose()?;
            Ok(Self { enabled, data })
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PlaystationData {
        pub username: Option<String>,
    }

    impl TryFrom<crate::app::data::config::service::PlaystationData> for self::PlaystationData {
        type Error = super::Error;

        fn try_from(data: crate::app::data::config::service::PlaystationData) -> Result<Self, Self::Error> {
            let username = data.username;
            Ok(Self { username })
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Steam {
        pub enabled: bool,
        pub data: Option<SteamData>,
    }

    impl TryFrom<crate::app::data::config::service::Steam> for self::Steam {
        type Error = super::Error;

        fn try_from(steam: crate::app::data::config::service::Steam) -> Result<Self, Self::Error> {
            let enabled = steam.enabled;
            let data = steam.data.map(TryInto::try_into).transpose()?;
            Ok(Self { enabled, data })
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SteamData {
        pub id: String,
        pub key: String,
        pub username: String,
    }

    impl TryFrom<crate::app::data::config::service::SteamData> for self::SteamData {
        type Error = super::Error;

        fn try_from(data: crate::app::data::config::service::SteamData) -> Result<Self, Self::Error> {
            let id = data.id;
            let key = data.key;
            let username = data.username;
            Ok(Self { id, key, username })
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Xbox {
        pub enabled: bool,
        pub data: Option<XboxData>,
    }

    impl TryFrom<crate::app::data::config::service::Xbox> for self::Xbox {
        type Error = super::Error;

        fn try_from(xbox: crate::app::data::config::service::Xbox) -> Result<Self, Self::Error> {
            let enabled = xbox.enabled;
            let data = xbox.data.map(TryInto::try_into).transpose()?;
            Ok(Self { enabled, data })
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct XboxData {
        pub username: Option<String>,
    }

    impl TryFrom<crate::app::data::config::service::XboxData> for self::XboxData {
        type Error = super::Error;

        fn try_from(data: crate::app::data::config::service::XboxData) -> Result<Self, Self::Error> {
            let username = data.username;
            Ok(Self { username })
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub discord_display_presence: bool,
    pub twitch_assets_enabled: bool,
    pub twitch_access_token: Option<String>,
    pub games_require_whitelisting: bool,
}

impl TryFrom<crate::app::data::config::Activity> for self::Activity {
    type Error = Error;

    fn try_from(activity: crate::app::data::config::Activity) -> Result<Self, Error> {
        let discord_display_presence = activity.discord_display_presence;
        let twitch_assets_enabled = activity.twitch_assets_enabled;
        let twitch_access_token = activity.twitch_access_token;
        let games_require_whitelisting = activity.games_require_whitelisting;
        Ok(Self {
            discord_display_presence,
            twitch_assets_enabled,
            twitch_access_token,
            games_require_whitelisting,
        })
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Games {}

impl TryFrom<crate::app::data::config::Games> for self::Games {
    type Error = Error;

    fn try_from(_games: crate::app::data::config::Games) -> Result<Self, Error> {
        Ok(Self {})
    }
}

#[derive(Clone)]
pub struct Channels {
    pub tx: Arc<Mutex<Sender<crate::app::ipc::Payload<Config>>>>,
    pub rx: Arc<RwLock<Receiver<crate::app::ipc::Payload<Config>>>>,
}

impl Channels {
    pub fn init() -> Result<Self, Error> {
        let payload = {
            let provenience = crate::app::ipc::Provenience::Backend;
            let data = Config::default();
            crate::app::ipc::Payload { provenience, data }
        };
        let (tx, rx) = tokio::sync::watch::channel(payload);
        let (tx, rx) = (Arc::new(Mutex::new(tx)), Arc::new(RwLock::new(rx)));
        Ok(Self { tx, rx })
    }
}
