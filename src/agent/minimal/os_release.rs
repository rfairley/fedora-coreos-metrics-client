//! OS version parsing - utility functions

use failure::{format_err, Fallible, ResultExt};
use serde_json;
use std::fs;

/// Read aleph version info from os version json file.
pub(crate) fn read_original_os_version<T>(file_path: T) -> Fallible<String>
where
    T: AsRef<str>,
{
    // open the os release file
    let fpath = file_path.as_ref();
    let file = fs::File::open(fpath)
        .with_context(|e| format_err!("failed to open aleph version file {}: {}", fpath, e))?;

    // parse the content
    let json: serde_json::Value =
        serde_json::from_reader(file).expect("failed to parse aleph version file as JSON");
    let build: String = json
        .get("build")
        .expect("aleph version file does not contain 'build' key")
        .to_string();

    // remove the leading and trailing quotes, \"
    Ok(build[1..build.len() - 1].to_string())
}
