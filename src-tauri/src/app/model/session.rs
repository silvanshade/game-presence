use crate::service::xbox::api::authorize::XboxXstsToken;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Default)]
pub struct Session {
    pub xbox: Arc<RwLock<Option<XboxXstsToken>>>,
}
