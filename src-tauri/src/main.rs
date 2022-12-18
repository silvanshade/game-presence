use snafu::prelude::*;

mod app;
mod core;
mod service;

use crate::core::Core;

#[derive(Debug, Snafu)]
enum Error {
    AppInit { source: crate::app::Error },
    CoreRun { source: crate::core::Error },
    ModelInit { source: crate::app::model::Error },
    StdIoFlush { source: std::io::Error },
    TokioJoin { source: tokio::task::JoinError },
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tauri::async_runtime::set(tokio::runtime::Handle::current());
    let model = crate::app::Model::init().await.context(ModelInitSnafu)?;
    let core = crate::Core::new(model.clone());
    crate::app::init(model).context(AppInitSnafu)?;
    core.finish().await.context(CoreRunSnafu)?;
    Ok(())
}
