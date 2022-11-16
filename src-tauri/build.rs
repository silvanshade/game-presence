use snafu::prelude::*;

#[derive(Debug, Snafu)]
enum Error {
    TauriBuild { source: Box<dyn std::error::Error> },
}

fn main() -> Result<(), Error> {
    let attributes = tauri_build::Attributes::default();
    tauri_build::try_build(attributes)
        .map_err(Into::into)
        .context(TauriBuildSnafu)?;
    Ok(())
}
