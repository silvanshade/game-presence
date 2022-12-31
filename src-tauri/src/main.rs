use snafu::prelude::*;

mod app;
mod core;
mod service;

use crate::core::Core;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Failed to initialize tauri app"))]
    AppInit { source: crate::app::Error },
    #[snafu(display("Failed to initialize service core loops"))]
    CoreInit { source: crate::core::Error },
    #[snafu(display("Failed to wait for service core loops to terminate"))]
    CoreDone { source: crate::core::Error },
    #[snafu(display("Failed to initialize tauri app model for managed state"))]
    ModelInit { source: crate::app::model::Error },
    #[snafu(display("Failed to flush stdio"))]
    StdIoFlush {
        backtrace: snafu::Backtrace,
        source: std::io::Error,
    },
    #[snafu(display("Failed to spawn a future onto the async runtime"))]
    TauriRuntimeSpawn {
        backtrace: snafu::Backtrace,
        source: tauri::Error,
    },
    #[cfg(feature = "debug")]
    TracingSubscriberTryInit {
        backtrace: snafu::Backtrace,
        source: tracing_subscriber::util::TryInitError,
    },
}

#[cfg_attr(feature = "tracing", tracing::instrument)]
#[snafu::report]
#[tokio::main]
async fn main() -> Result<(), Error> {
    #[cfg(feature = "debug")]
    tracing_subscriber::registry()
        // .with(console_subscriber::spawn())
        // .with(tracing_subscriber::fmt::layer())
        .with(tracing_forest::ForestLayer::default())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .try_init()
        .context(TracingSubscriberTryInitSnafu)?;
    tauri::async_runtime::set(tokio::runtime::Handle::current());
    let (tx, rx) = tokio::sync::oneshot::channel();
    let model = crate::app::Model::init().await.context(ModelInitSnafu)?;
    let core = crate::Core::init(rx);
    crate::app::init(model.clone(), tx).context(AppInitSnafu)?;
    core.await
        .context(TauriRuntimeSpawnSnafu)?
        .context(CoreInitSnafu)?
        .terminate()
        .await
        .context(CoreDoneSnafu)?;
    Ok(())
}
