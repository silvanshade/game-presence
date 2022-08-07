//! Build-time metadata for the server.

use serde::{Deserialize, Serialize};

mod built_info {
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub mod project {
    pub const QUALIFIER: &str = "io";
    pub const ORGANIZATION: &str = "silvanshade";
    pub const APPLICATION: &str = "steam-presence";
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuiltInfo {
    ci_platform: Option<String>,
    pkg_version: String,
    pkg_version_major: String,
    pkg_version_minor: String,
    pkg_version_patch: String,
    pkg_version_pre: String,
    pkg_authors: String,
    pkg_name: String,
    pkg_description: String,
    pkg_homepage: String,
    pkg_license: String,
    pkg_repository: String,
    target: String,
    host: String,
    profile: String,
    rustc: String,
    rustdoc: String,
    opt_level: String,
    num_jobs: u32,
    debug: bool,
    features: Vec<String>,
    features_str: String,
    rustc_version: String,
    rustdoc_version: String,
    git_version: Option<String>,
    git_dirty: Option<bool>,
    git_head_ref: Option<String>,
    git_commit_hash: Option<String>,
    dependencies: Vec<(String, String)>,
    dependencies_str: String,
    built_time_utc: String,
    cfg_target_arch: String,
    cfg_endian: String,
    cfg_env: String,
    cfg_family: String,
    cfg_os: String,
    cfg_pointer_width: String,
}

impl Default for BuiltInfo {
    fn default() -> Self {
        let ci_platform = self::built_info::CI_PLATFORM.map(Into::into);
        let pkg_version = self::built_info::PKG_VERSION.into();
        let pkg_version_major = self::built_info::PKG_VERSION_MAJOR.into();
        let pkg_version_minor = self::built_info::PKG_VERSION_MINOR.into();
        let pkg_version_patch = self::built_info::PKG_VERSION_PATCH.into();
        let pkg_version_pre = self::built_info::PKG_VERSION_PRE.into();
        let pkg_authors = self::built_info::PKG_AUTHORS.into();
        let pkg_name = self::built_info::PKG_NAME.into();
        let pkg_description = self::built_info::PKG_DESCRIPTION.into();
        let pkg_homepage = self::built_info::PKG_HOMEPAGE.into();
        let pkg_license = self::built_info::PKG_LICENSE.into();
        let pkg_repository = self::built_info::PKG_REPOSITORY.into();
        let target = self::built_info::TARGET.into();
        let host = self::built_info::HOST.into();
        let profile = self::built_info::PROFILE.into();
        let rustc = self::built_info::RUSTC.into();
        let rustdoc = self::built_info::RUSTDOC.into();
        let opt_level = self::built_info::OPT_LEVEL.into();
        let num_jobs = self::built_info::NUM_JOBS;
        let debug = self::built_info::DEBUG;
        let features = self::built_info::FEATURES.into_iter().map(Into::into).collect();
        let features_str = self::built_info::FEATURES_STR.into();
        let rustc_version = self::built_info::RUSTC_VERSION.into();
        let rustdoc_version = self::built_info::RUSTDOC_VERSION.into();
        let git_version = self::built_info::GIT_VERSION.map(Into::into);
        let git_dirty = self::built_info::GIT_DIRTY.map(Into::into);
        let git_head_ref = self::built_info::GIT_HEAD_REF.map(Into::into);
        let git_commit_hash = self::built_info::GIT_COMMIT_HASH.map(Into::into);
        let dependencies = self::built_info::DEPENDENCIES
            .into_iter()
            .map(|(dep, ver)| (dep.into(), ver.into()))
            .collect();
        let dependencies_str = self::built_info::DEPENDENCIES_STR.into();
        let built_time_utc = self::built_info::BUILT_TIME_UTC.into();
        let cfg_target_arch = self::built_info::CFG_TARGET_ARCH.into();
        let cfg_endian = self::built_info::CFG_ENDIAN.into();
        let cfg_env = self::built_info::CFG_ENV.into();
        let cfg_family = self::built_info::CFG_FAMILY.into();
        let cfg_os = self::built_info::CFG_OS.into();
        let cfg_pointer_width = self::built_info::CFG_POINTER_WIDTH.into();
        BuiltInfo {
            ci_platform,
            pkg_version,
            pkg_version_major,
            pkg_version_minor,
            pkg_version_patch,
            pkg_version_pre,
            pkg_authors,
            pkg_name,
            pkg_description,
            pkg_homepage,
            pkg_license,
            pkg_repository,
            target,
            host,
            profile,
            rustc,
            rustdoc,
            opt_level,
            num_jobs,
            debug,
            features,
            features_str,
            rustc_version,
            rustdoc_version,
            git_version,
            git_dirty,
            git_head_ref,
            git_commit_hash,
            dependencies,
            dependencies_str,
            built_time_utc,
            cfg_target_arch,
            cfg_endian,
            cfg_env,
            cfg_family,
            cfg_os,
            cfg_pointer_width,
        }
    }
}
