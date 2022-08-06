//! Build-time metadata for the server.

mod built_info {
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub mod project {
    pub const QUALIFIER: &str = "io";
    pub const ORGANIZATION: &str = "silvanshade";
    pub const APPLICATION: &str = "steam-presence";
}
