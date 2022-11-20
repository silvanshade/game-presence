use snafu::prelude::*;

#[derive(Debug, Snafu)]
enum Error {
    TauriBuild { source: Box<dyn std::error::Error> },
    WriteBuiltFile { source: std::io::Error },
}

fn built_collect_metadata() -> Result<(), Error> {
    built::write_built_file().context(WriteBuiltFileSnafu)?;
    Ok(())
}

fn tauri_build() -> Result<(), Error> {
    let attributes = tauri_build::Attributes::default();
    tauri_build::try_build(attributes)
        .map_err(Into::into)
        .context(TauriBuildSnafu)?;
    Ok(())
}

fn main() -> Result<(), Error> {
    built_collect_metadata()?;
    tauri_build()?;
    Ok(())
}
