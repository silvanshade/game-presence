use tokio::sync::RwLock;

mod config;
pub use self::config::Config;

#[derive(Debug)]
pub struct Model {
    pub config: RwLock<self::Config>,
}

impl Model {
    pub fn new() -> Self {
        let config = Default::default();
        Self { config }
    }
}

// fn make_state() -> Result<crate::app::Model, self::Error> {
//     let config = tauri::async_runtime::block_on(crate::app::Config::load()).context(ConfigSnafu)?;
//     let data = crate::app::Model::new(config);
//     Ok(data)
// }
