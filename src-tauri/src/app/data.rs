use tokio::sync::RwLock;

#[derive(Debug)]
pub struct Data {
    pub config: RwLock<crate::app::Config>,
}

impl Data {
    pub fn new(config: crate::app::Config) -> Self {
        let config = RwLock::new(config);
        Self { config }
    }
}
