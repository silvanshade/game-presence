use snafu::prelude::*;

mod app;
mod core;
mod service;

use crate::core::Core;

#[derive(Debug, Snafu)]
enum Error {
    AppInit { source: crate::app::Error },
    CoreNew { source: crate::core::Error },
    CoreFinish { source: crate::core::Error },
    ModelInit { source: crate::app::model::Error },
    StdIoFlush { source: std::io::Error },
    TauriSpawn { source: tauri::Error },
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tauri::async_runtime::set(tokio::runtime::Handle::current());
    let (tx, rx) = tokio::sync::oneshot::channel();
    let model = crate::app::Model::init().await.context(ModelInitSnafu)?;
    let core = crate::Core::new(rx);
    crate::app::init(model.clone(), tx).context(AppInitSnafu)?;
    core.await
        .context(TauriSpawnSnafu)?
        .context(CoreNewSnafu)?
        .finish()
        .await
        .context(CoreFinishSnafu)?;
    Ok(())
}
