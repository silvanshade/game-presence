use snafu::prelude::*;

mod api;
mod app;
mod core;
mod service;

#[derive(Debug, Snafu)]
enum Error {
    AppInit { source: crate::app::Error },
    CoreRun { source: crate::core::Error },
    ModelInit { source: crate::app::model::Error },
    TokioJoin { source: tokio::task::JoinError },
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tauri::async_runtime::set(tokio::runtime::Handle::current());
    let model = crate::app::model::Model::init().await.context(ModelInitSnafu)?;
    let core = tokio::spawn(crate::core::run(model.clone()));
    crate::app::init(model).context(AppInitSnafu)?;
    core.await.context(TokioJoinSnafu)?.context(CoreRunSnafu)?;
    Ok(())
}
