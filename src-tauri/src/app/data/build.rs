use async_graphql::*;
use serde::{Deserialize, Serialize};
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    CollectGitDirty,
    CollectGitHashCommit,
}

pub mod metadata {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct BuildInfo {
    pub built_time_utc: &'static str,
    pub cfg_os: &'static str,
    pub git_commit_hash: &'static str,
    pub git_dirty: &'static str,
    pub pkg_homepage: &'static str,
    pub pkg_license: &'static str,
    pub pkg_version: &'static str,
    pub profile: &'static str,
    pub target: &'static str,
}

impl BuildInfo {
    pub fn collect() -> Result<Self, Error> {
        let built_time_utc = self::metadata::BUILT_TIME_UTC;
        let cfg_os = self::metadata::CFG_OS;
        let git_commit_hash = &self::metadata::GIT_COMMIT_HASH.context(CollectGitHashCommitSnafu)?[.. 7];
        let git_dirty = if self::metadata::GIT_DIRTY.context(CollectGitDirtySnafu)? {
            "dirty"
        } else {
            "clean"
        };
        let pkg_homepage = self::metadata::PKG_HOMEPAGE;
        let pkg_license = self::metadata::PKG_LICENSE;
        let pkg_version = self::metadata::PKG_VERSION;
        let profile = self::metadata::PROFILE;
        let target = self::metadata::TARGET;
        let this = Self {
            built_time_utc,
            cfg_os,
            git_dirty,
            git_commit_hash,
            pkg_homepage,
            pkg_license,
            pkg_version,
            profile,
            target,
        };
        Ok(this)
    }
}
