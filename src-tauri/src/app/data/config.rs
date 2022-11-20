use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Config {
    pub profiles: Vec<Profile>,
}

#[derive(Deserialize, Serialize)]
struct Profile {
    pub services: Services,
    pub activity: Activity,
    pub games: Games,
}

#[derive(Deserialize, Serialize)]
struct Services {
    pub nintendo: Option<self::service::Nintendo>,
    pub playstation: Option<self::service::Playstation>,
    pub steam: Option<self::service::Steam>,
    pub xbox: Option<self::service::Xbox>,
}

pub mod service {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    pub struct Nintendo {
        pub enabled: bool,
    }

    #[derive(Deserialize, Serialize)]
    pub struct Playstation {
        pub enabled: bool,
    }

    #[derive(Deserialize, Serialize)]
    pub struct Steam {
        pub enabled: bool,
        pub id: String,
        pub key: String,
    }

    #[derive(Deserialize, Serialize)]
    pub struct Xbox {
        pub enabled: bool,
    }
}

#[derive(Deserialize, Serialize)]
struct Activity {
    pub discord_display_presence: bool,
    pub twitch_assets_enabled: bool,
    pub games_require_whitelisting: bool,
}

#[derive(Deserialize, Serialize)]
struct Games {}
