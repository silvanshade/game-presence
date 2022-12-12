use snafu::prelude::*;

mod api;
mod app;
mod core;
mod service;

#[derive(Debug, Snafu)]
enum Error {
    AppInit { source: crate::app::Error },
    CoreRun { source: crate::core::Error },
    StateInit { source: crate::app::model::state::Error },
    TokioJoin { source: tokio::task::JoinError },
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tauri::async_runtime::set(tokio::runtime::Handle::current());
    let state = crate::app::model::State::init().await.context(StateInitSnafu)?;
    let core = tokio::spawn(crate::core::run(state.clone()));
    crate::app::init(state).context(AppInitSnafu)?;
    core.await.context(TokioJoinSnafu)?.context(CoreRunSnafu)?;
    Ok(())
}
