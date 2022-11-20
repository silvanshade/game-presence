use snafu::prelude::*;
use tracing::field::display;

#[derive(Debug, Snafu)]
enum Error {
    AppInit { source: crate::app::Error },
}

mod api;
mod app;

fn display_build_info() {
    use crate::app::data::build::metadata;
    println!("CI_PLATFORM: {}", metadata::CI_PLATFORM.unwrap_or("N/A"));
    println!("PKG_VERSION: {}", metadata::PKG_VERSION);
    println!("PKG_VERSION_MAJOR: {}", metadata::PKG_VERSION_MAJOR);
    println!("PKG_VERSION_MINOR: {}", metadata::PKG_VERSION_MINOR);
    println!("PKG_VERSION_PATCH: {}", metadata::PKG_VERSION_PATCH);
    println!("PKG_VERSION_PRE: {}", metadata::PKG_VERSION_PRE);
    println!("PKG_AUTHORS: {}", metadata::PKG_AUTHORS);
    println!("PKG_NAME: {}", metadata::PKG_NAME);
    println!("PKG_DESCRIPTION: {}", metadata::PKG_DESCRIPTION);
    println!("PKG_HOMEPAGE: {}", metadata::PKG_HOMEPAGE);
    println!("PKG_LICENSE: {}", metadata::PKG_LICENSE);
    println!("PKG_REPOSITORY: {}", metadata::PKG_REPOSITORY);
    println!("TARGET: {}", metadata::TARGET);
    println!("HOST: {}", metadata::HOST);
    println!("PROFILE: {}", metadata::PROFILE);
    println!("RUSTC: {}", metadata::RUSTC);
    println!("RUSTDOC: {}", metadata::RUSTDOC);
    println!("OPT_LEVEL: {}", metadata::OPT_LEVEL);
    println!("NUM_JOBS: {}", metadata::NUM_JOBS);
    println!("DEBUG: {}", metadata::DEBUG);
    println!("FEATURES: {:#?}", metadata::FEATURES);
    println!("FEATURES_STR: {}", metadata::FEATURES_STR);
    println!("RUSTC_VERSION: {}", metadata::RUSTC_VERSION);
    println!("RUSTDOC_VERSION: {}", metadata::RUSTDOC_VERSION);
    println!("GIT_VERSION: {:#?}", metadata::GIT_VERSION);
    println!("GIT_DIRTY: {:#?}", metadata::GIT_DIRTY);
    println!("GIT_HEAD_REF: {:#?}", metadata::GIT_HEAD_REF);
    println!("GIT_COMMIT_HASH: {:#?}", metadata::GIT_COMMIT_HASH);
    println!("BUILT_TIME_UTC: {:#?}", metadata::BUILT_TIME_UTC);
    println!("CFG_TARGET_ARCH: {:#?}", metadata::CFG_TARGET_ARCH);
    println!("CFG_ENDIAN: {:#?}", metadata::CFG_ENDIAN);
    println!("CFG_ENV: {:#?}", metadata::CFG_ENV);
    println!("CFG_FAMILY: {:#?}", metadata::CFG_FAMILY);
    println!("CFG_OS: {:#?}", metadata::CFG_OS);
    println!("CFG_POINTER_WIDTH: {:#?}", metadata::CFG_POINTER_WIDTH);
}

fn main() -> Result<(), Error> {
    display_build_info();
    crate::app::init().context(AppInitSnafu)?;
    Ok(())
}
