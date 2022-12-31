use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Gui {
    pub activity: Activity,
    pub services: Services,
    pub games: Games,
}

impl Gui {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn synchronize_with_config(&mut self, config: &crate::app::model::Config) {
        self.services.synchronize_with_config(config);
        self.activity.synchronize_with_config(config);
        self.games.synchronize_with_config(config);
    }
}

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub polling_active: bool,
    pub discord_display_presence: bool,
    pub games_require_whitelisting: bool,
    pub service_priorities: Vec<crate::app::model::config::ServicePrioritiesEntry>,
}

impl Activity {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn synchronize_with_config(&mut self, config: &crate::app::model::Config) {
        let activity = &config.activity;
        self.polling_active = activity.polling_active;
        self.discord_display_presence = activity.discord_display_presence;
        self.games_require_whitelisting = activity.games_require_whitelisting;
        self.service_priorities = activity.service_priorities.clone();
    }
}

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Services {
    pub nintendo: self::service::Nintendo,
    pub playstation: self::service::Playstation,
    pub steam: self::service::Steam,
    pub twitch: self::service::Twitch,
    pub xbox: self::service::Xbox,
}

impl Services {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn synchronize_with_config(&mut self, config: &crate::app::model::Config) {
        self.nintendo.synchronize_with_config(config);
        self.playstation.synchronize_with_config(config);
        self.steam.synchronize_with_config(config);
        self.twitch.synchronize_with_config(config);
        self.xbox.synchronize_with_config(config);
    }
}

pub mod service {
    use serde::{Deserialize, Serialize};

    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Clone, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Nintendo {
        pub disclaimer_acknowledged: bool,
        pub enabled: bool,
        pub assets_priorities: Vec<crate::app::model::config::AssetSourceEntry>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<self::nintendo::Data>,
    }

    impl Nintendo {
        #[cfg_attr(feature = "tracing", tracing::instrument)]
        pub fn synchronize_with_config(&mut self, config: &crate::app::model::Config) {
            let nintendo = &config.services.nintendo;
            self.disclaimer_acknowledged = nintendo.disclaimer_acknowledged;
            self.enabled = nintendo.enabled;
            self.assets_priorities = nintendo.assets_priorities.clone();
        }
    }

    impl Default for self::Nintendo {
        #[cfg_attr(feature = "tracing", tracing::instrument)]
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

    pub mod nintendo {
        use serde::{Deserialize, Serialize};

        #[cfg_attr(feature = "debug", derive(Debug))]
        #[derive(Clone, Deserialize, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Data {
            pub presence: Option<crate::app::model::Presence>,
        }
    }

    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Clone, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Playstation {
        pub enabled: bool,
        pub assets_priorities: Vec<crate::app::model::config::AssetSourceEntry>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<self::playstation::Data>,
    }

    impl Playstation {
        #[cfg_attr(feature = "tracing", tracing::instrument)]
        pub fn synchronize_with_config(&mut self, config: &crate::app::model::Config) {
            let playstation = &config.services.playstation;
            self.enabled = playstation.enabled;
            self.assets_priorities = playstation.assets_priorities.clone();
        }
    }

    impl Default for self::Playstation {
        #[cfg_attr(feature = "tracing", tracing::instrument)]
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

    pub mod playstation {
        use serde::{Deserialize, Serialize};

        #[cfg_attr(feature = "debug", derive(Debug))]
        #[derive(Clone, Deserialize, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Data {
            presence: Option<crate::app::model::Presence>,
        }
    }

    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Clone, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Steam {
        pub enabled: bool,
        pub assets_priorities: Vec<crate::app::model::config::AssetSourceEntry>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<self::steam::Data>,
    }

    impl Steam {
        #[cfg_attr(feature = "tracing", tracing::instrument)]
        pub fn synchronize_with_config(&mut self, config: &crate::app::model::Config) {
            let steam = &config.services.steam;
            self.enabled = steam.enabled;
            self.assets_priorities = steam.assets_priorities.clone();
            if let Some(that_data) = &steam.data {
                if let Some(this_data) = &mut self.data {
                    this_data.api_key = that_data.api_key.clone();
                } else {
                    self.data = Some(that_data.clone().into());
                }
            }
        }
    }

    impl Default for self::Steam {
        #[cfg_attr(feature = "tracing", tracing::instrument)]
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

    pub mod steam {
        use serde::{Deserialize, Serialize};

        #[cfg_attr(feature = "debug", derive(Debug))]
        #[derive(Clone, Deserialize, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Data {
            pub api_key: String,
            presence: Option<crate::app::model::Presence>,
        }

        impl From<crate::app::model::config::service::steam::Data> for self::Data {
            fn from(data: crate::app::model::config::service::steam::Data) -> Self {
                let api_key = data.api_key.clone();
                let presence = None;
                Self { api_key, presence }
            }
        }
    }

    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Clone, Default, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Twitch {
        pub enabled: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<self::twitch::Data>,
    }

    impl Twitch {
        #[cfg_attr(feature = "tracing", tracing::instrument)]
        pub fn synchronize_with_config(&mut self, config: &crate::app::model::Config) {
            let twitch = &config.services.twitch;
            self.enabled = twitch.enabled;
        }
    }

    pub mod twitch {
        use serde::{Deserialize, Serialize};

        #[cfg_attr(feature = "debug", derive(Debug))]
        #[derive(Clone, Deserialize, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Data {}
    }

    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Clone, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Xbox {
        pub enabled: bool,
        pub assets_priorities: Vec<crate::app::model::config::AssetSourceEntry>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<self::xbox::Data>,
    }

    impl Xbox {
        #[cfg_attr(feature = "tracing", tracing::instrument)]
        pub fn synchronize_with_config(&mut self, config: &crate::app::model::Config) {
            let xbox = &config.services.xbox;
            self.enabled = xbox.enabled;
            self.assets_priorities = xbox.assets_priorities.clone();
        }
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

    pub mod xbox {
        use serde::{Deserialize, Serialize};

        #[cfg_attr(feature = "debug", derive(Debug))]
        #[derive(Clone, Deserialize, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Data {
            pub gamertag: String,
            pub presence: Option<crate::app::model::Presence>,
        }
    }
}

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Games {}

impl Games {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn synchronize_with_config(&mut self, _config: &crate::app::model::Config) {
    }
}
