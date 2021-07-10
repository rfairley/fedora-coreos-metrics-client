//! Cloud instance type parsing - utility functions

use crate::util;
use failure::{bail, format_err, Fallible, ResultExt};
use std::io::Read;
use std::{fs, io};

/// Read instance type from cloud metadata file created by Afterburn
/// reference: https://github.com/coreos/afterburn/pull/278
pub(crate) fn read_instance_type<T>(cmdline_path: T, platform_id: &str) -> Fallible<String>
where
    T: AsRef<str>,
{
    let flag = match platform_id {
        "aliyun" => "AFTERBURN_ALIYUN_INSTANCE_TYPE",
        "aws" => "AFTERBURN_AWS_INSTANCE_TYPE",
        "azure" => "AFTERBURN_AZURE_VMSIZE",
        "gcp" => "AFTERBURN_GCP_MACHINE_TYPE",
        "openstack" => "AFTERBURN_OPENSTACK_INSTANCE_TYPE",
        _ => bail!("platform id not supported"),
    };
    // open the cmdline file
    let fpath = cmdline_path.as_ref();
    let file = fs::File::open(fpath)
        .with_context(|e| format_err!("failed to open metadata file {}: {}", fpath, e))?;

    // read content
    let mut bufrd = io::BufReader::new(file);
    let mut contents = String::new();
    bufrd
        .read_to_string(&mut contents)
        .with_context(|e| format_err!("failed to read metadata file {}: {}", fpath, e))?;

    // lookup flag by key name
    match util::find_flag_value(flag, &contents, "\n") {
        Some(platform) => {
            log::trace!("found platform id: {}", platform);
            Ok(platform)
        }
        None => bail!("could not find flag '{}' in {}", flag, fpath),
    }
}
