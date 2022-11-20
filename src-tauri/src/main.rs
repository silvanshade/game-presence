use snafu::prelude::*;

mod api;
mod app;

#[derive(Debug, Snafu)]
enum Error {
    AppInit { source: crate::app::Error },
}

fn main() -> Result<(), Error> {
    println!("{:#?}", crate::app::data::build::BuildInfo::collect());
    crate::app::init().context(AppInitSnafu)?;
    Ok(())
}
