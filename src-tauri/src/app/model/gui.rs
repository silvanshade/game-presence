use serde::{Deserialize, Serialize};
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Gui {
    pub services: Services,
    pub activity: Activity,
    pub games: Games,
}

impl TryFrom<crate::app::model::Config> for Gui {
    type Error = Error;

    fn try_from(config: crate::app::model::Config) -> Result<Self, Self::Error> {
        let services = config.services.try_into()?;
        let activity = config.activity.try_into()?;
        let games = config.games.try_into()?;
        Ok(Gui {
            services,
            activity,
            games,
        })
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

impl TryFrom<crate::app::model::config::Services> for self::Services {
    type Error = Error;

    fn try_from(services: crate::app::model::config::Services) -> Result<Self, Self::Error> {
        let nintendo = services.nintendo.try_into()?;
        let playstation = services.playstation.try_into()?;
        let steam = services.steam.try_into()?;
        let twitch = services.twitch.try_into()?;
        let xbox = services.xbox.try_into()?;
        Ok(self::Services {
            nintendo,
            playstation,
            steam,
            twitch,
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
        pub assets_priorities: Vec<crate::app::model::config::AssetSourceEntry>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<self::nintendo::Data>,
    }

    impl Default for self::Nintendo {
        fn default() -> Self {
            let disclaimer_acknowledged = bool::default();
            let enabled = bool::default();
            let assets_priorities = vec![crate::app::model::config::AssetSourceEntry::default()];
            let data = Option::default();
            Self {
                disclaimer_acknowledged,
                enabled,
                assets_priorities,
                data,
            }
        }
    }

    impl TryFrom<crate::app::model::config::service::Nintendo> for self::Nintendo {
        type Error = super::Error;

        fn try_from(nintendo: crate::app::model::config::service::Nintendo) -> Result<Self, Self::Error> {
            let disclaimer_acknowledged = nintendo.disclaimer_acknowledged;
            let enabled = nintendo.enabled;
            let assets_priorities = nintendo.assets_priorities;
            let data = nintendo.data.map(TryInto::try_into).transpose()?;
            Ok(self::Nintendo {
                disclaimer_acknowledged,
                enabled,
                assets_priorities,
                data,
            })
        }
    }

    pub mod nintendo {
        use serde::{Deserialize, Serialize};

        #[derive(Clone, Debug, Deserialize, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Data {}

        impl TryFrom<crate::app::model::config::service::nintendo::Data> for self::Data {
            type Error = super::super::Error;

            fn try_from(_data: crate::app::model::config::service::nintendo::Data) -> Result<Self, Self::Error> {
                Ok(self::Data {})
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Playstation {
        pub enabled: bool,
        pub assets_priorities: Vec<crate::app::model::config::AssetSourceEntry>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<self::playstation::Data>,
    }

    impl Default for self::Playstation {
        fn default() -> Self {
            let enabled = bool::default();
            let assets_priorities = vec![crate::app::model::config::AssetSourceEntry::default()];
            let data = Option::default();
            Self {
                enabled,
                assets_priorities,
                data,
            }
        }
    }

    impl TryFrom<crate::app::model::config::service::Playstation> for self::Playstation {
        type Error = super::Error;

        fn try_from(playstation: crate::app::model::config::service::Playstation) -> Result<Self, Self::Error> {
            let enabled = playstation.enabled;
            let assets_priorities = playstation.assets_priorities;
            let data = playstation.data.map(TryInto::try_into).transpose()?;
            Ok(self::Playstation {
                enabled,
                assets_priorities,
                data,
            })
        }
    }

    pub mod playstation {
        use serde::{Deserialize, Serialize};

        #[derive(Clone, Debug, Deserialize, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Data {}

        impl TryFrom<crate::app::model::config::service::playstation::Data> for self::Data {
            type Error = super::super::Error;

            fn try_from(_data: crate::app::model::config::service::playstation::Data) -> Result<Self, Self::Error> {
                Ok(self::Data {})
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Steam {
        pub enabled: bool,
        pub assets_priorities: Vec<crate::app::model::config::AssetSourceEntry>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<self::steam::Data>,
    }

    impl Default for self::Steam {
        fn default() -> Self {
            let enabled = bool::default();
            let assets_priorities = vec![crate::app::model::config::AssetSourceEntry::default()];
            let data = Option::default();
            Self {
                enabled,
                assets_priorities,
                data,
            }
        }
    }

    impl TryFrom<crate::app::model::config::service::Steam> for self::Steam {
        type Error = super::Error;

        fn try_from(steam: crate::app::model::config::service::Steam) -> Result<Self, Self::Error> {
            let enabled = steam.enabled;
            let assets_priorities = steam.assets_priorities;
            let data = steam.data.map(TryInto::try_into).transpose()?;
            Ok(self::Steam {
                enabled,
                assets_priorities,
                data,
            })
        }
    }

    pub mod steam {
        use serde::{Deserialize, Serialize};

        #[derive(Clone, Debug, Deserialize, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Data {}

        impl TryFrom<crate::app::model::config::service::steam::Data> for self::Data {
            type Error = super::super::Error;

            fn try_from(_data: crate::app::model::config::service::steam::Data) -> Result<Self, Self::Error> {
                Ok(self::Data {})
            }
        }
    }

    #[derive(Clone, Debug, Default, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Twitch {
        pub enabled: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<self::twitch::Data>,
    }

    impl TryFrom<crate::app::model::config::service::Twitch> for self::Twitch {
        type Error = super::Error;

        fn try_from(twitch: crate::app::model::config::service::Twitch) -> Result<Self, Self::Error> {
            let enabled = twitch.enabled;
            let data = twitch.data.map(TryInto::try_into).transpose()?;
            Ok(self::Twitch { enabled, data })
        }
    }

    pub mod twitch {
        use serde::{Deserialize, Serialize};

        #[derive(Clone, Debug, Deserialize, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Data {}

        impl TryFrom<crate::app::model::config::service::twitch::Data> for self::Data {
            type Error = super::super::Error;

            fn try_from(_data: crate::app::model::config::service::twitch::Data) -> Result<Self, Self::Error> {
                Ok(self::Data {})
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Xbox {
        pub enabled: bool,
        pub assets_priorities: Vec<crate::app::model::config::AssetSourceEntry>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<self::xbox::Data>,
    }

    impl Default for self::Xbox {
        fn default() -> Self {
            let enabled = bool::default();
            let assets_priorities = vec![crate::app::model::config::AssetSourceEntry::default()];
            let data = Option::default();
            Self {
                enabled,
                assets_priorities,
                data,
            }
        }
    }

    impl TryFrom<crate::app::model::config::service::Xbox> for self::Xbox {
        type Error = super::Error;

        fn try_from(xbox: crate::app::model::config::service::Xbox) -> Result<Self, Self::Error> {
            let enabled = xbox.enabled;
            let assets_priorities = xbox.assets_priorities;
            let data = xbox.data.map(TryInto::try_into).transpose()?;
            Ok(self::Xbox {
                enabled,
                assets_priorities,
                data,
            })
        }
    }

    pub mod xbox {
        use serde::{Deserialize, Serialize};

        #[derive(Clone, Debug, Deserialize, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Data {}

        impl TryFrom<crate::app::model::config::service::xbox::Data> for self::Data {
            type Error = super::super::Error;

            fn try_from(_data: crate::app::model::config::service::xbox::Data) -> Result<Self, Self::Error> {
                Ok(self::Data {})
            }
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub polling_active: bool,
    pub discord_display_presence: bool,
    pub games_require_whitelisting: bool,
    pub service_priorities: Vec<crate::app::model::config::ServicePrioritiesEntry>,
}

impl TryFrom<crate::app::model::config::Activity> for self::Activity {
    type Error = Error;

    fn try_from(activity: crate::app::model::config::Activity) -> Result<Self, Self::Error> {
        let polling_active = activity.polling_active;
        let discord_display_presence = activity.discord_display_presence;
        let games_require_whitelisting = activity.games_require_whitelisting;
        let service_priorities = activity.service_priorities;
        Ok(self::Activity {
            polling_active,
            discord_display_presence,
            games_require_whitelisting,
            service_priorities,
        })
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Games {}

impl TryFrom<crate::app::model::config::Games> for self::Games {
    type Error = Error;

    fn try_from(_games: crate::app::model::config::Games) -> Result<Self, Self::Error> {
        Ok(self::Games {})
    }
}
