use snafu::prelude::*;

mod api;
mod app;
mod service;

#[derive(Debug, Snafu)]
enum Error {
    AppInit { source: crate::app::Error },
    StateInit { source: crate::app::model::state::Error },
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tauri::async_runtime::set(tokio::runtime::Handle::current());
    let state = crate::app::model::State::init().await.context(StateInitSnafu)?;
    crate::app::init(state).context(AppInitSnafu)?;
    Ok(())
}
