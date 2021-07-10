//! Module for running `lsblk --fs --json`, and storing the
//! output in the struct LsblkJSON
use failure::{bail, format_err, Fallible, ResultExt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct LsblkJSON {
    pub(crate) blockdevices: Vec<DeviceJSON>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct DeviceJSON {
    name: String,
    fstype: Option<String>,
    label: Option<String>,
    fsavail: Option<String>,
    #[serde(rename = "fsuse%")]
    fsuse_percentage: Option<String>,
    mountpoint: Option<String>,
    children: Option<Box<Vec<DeviceJSON>>>,
}

impl LsblkJSON {
    pub(crate) fn new() -> Fallible<LsblkJSON> {
        let mut cmd = std::process::Command::new("lsblk");
        let cmdrun = cmd
            .arg("--fs")
            .arg("--json")
            .output()
            .with_context(|e| format_err!("failed to run lsblk --fs --json: {}", e))?;

        if !cmdrun.status.success() {
            bail!(
                "lsblk --fs --json failed:\n{}",
                String::from_utf8_lossy(&cmdrun.stderr)
            );
        }
        Ok(serde_json::from_slice(&cmdrun.stdout)?)
    }
}
