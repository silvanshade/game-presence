use snafu::prelude::*;

#[derive(Debug, Snafu)]
enum Error {
    AppInit { source: crate::app::Error },
}

mod api;
mod app;

fn main() -> Result<(), Error> {
    crate::app::init().context(AppInitSnafu)?;
    Ok(())
}
