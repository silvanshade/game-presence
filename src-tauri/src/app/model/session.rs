use crate::service::xbox;
use std::sync::Arc;
use tokio::sync::RwLock;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Default)]
pub struct Session {
    pub xbox: Arc<RwLock<Option<xbox::XstsToken>>>,
}
