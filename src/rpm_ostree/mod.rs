//! Use rpm-ostree commands to extract system info

mod cli_status;

pub use cli_status::{basearch, booted, updates_stream};

use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Release {
    /// OS version.
    pub version: String,
    /// Image base checksum.
    pub checksum: String,
}
