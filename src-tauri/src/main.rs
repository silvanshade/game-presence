use snafu::prelude::*;

mod api;
mod app;
mod service;

#[derive(Debug, Snafu)]
enum Error {
    AppInit { source: crate::app::Error },
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tauri::async_runtime::set(tokio::runtime::Handle::current());
    crate::app::init().context(AppInitSnafu)?;
    Ok(())
}
