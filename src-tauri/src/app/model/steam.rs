use std::sync::Arc;
use tauri::async_runtime::{Mutex, RwLock};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Status {
    Loop,
    Stop,
}

pub struct Steam {
    pub config: Arc<RwLock<crate::app::model::Config>>,
    pub discord: Arc<Mutex<crate::app::model::Discord>>,
    pub status: Arc<RwLock<Status>>,
    pub handle: tauri::async_runtime::JoinHandle<Result<(), crate::api::steam::Error>>,
}

impl Steam {
    pub fn new(
        config: Arc<RwLock<crate::app::model::Config>>,
        discord: Arc<Mutex<crate::app::model::Discord>>,
    ) -> Self {
        let status = Arc::new(RwLock::new(Status::Loop));
        let handle = {
            let task = crate::api::steam::poll_loop(config.clone(), discord.clone(), status.clone());
            tauri::async_runtime::spawn(task)
        };
        Self {
            config,
            discord,
            status,
            handle,
        }
    }
}
