//! Kernel cmdline parsing - utility functions
//!
//! NOTE(lucab): this is not a complete/correct cmdline parser, as it implements
//!  just enough logic to extract the platform ID value. In particular, it does not
//!  handle separator quoting/escaping, list of values, and merging of repeated
//!  flags. Logic is taken from Afterburn, please backport any bugfix there too:
//!  https://github.com/coreos/afterburn/blob/v4.1.0/src/util/cmdline.rs

use crate::util;
use failure::Fallible;

/// Platform key.
#[cfg(not(feature = "cl-legacy"))]
const CMDLINE_PLATFORM_FLAG: &str = "ignition.platform.id";
/// Platform key (CL and RHCOS legacy name: "OEM").
#[cfg(feature = "cl-legacy")]
const CMDLINE_PLATFORM_FLAG: &str = "coreos.oem.id";

/// Read platform value from cmdline file.
pub(crate) fn get_platform<T>(cmdline_path: T) -> Fallible<String>
where
    T: AsRef<str>,
{
    let fpath = cmdline_path.as_ref();
    util::get_value_by_flag(CMDLINE_PLATFORM_FLAG, fpath, " ")
}
