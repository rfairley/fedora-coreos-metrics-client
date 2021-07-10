// Copyright 2018 CoreOS, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Kernel cmdline parsing - utility functions
//!
//! NOTE(lucab): this is not a complete/correct cmdline parser, as it implements
//!  just enough logic to extract the OEM ID value. In particular, it doesn't
//!  handle separator quoting/escaping, list of values, and merging of repeated
//!  flags.

use failure::{bail, format_err, Fallible, ResultExt};
use std::io::Read;
use std::{fs, io};

// Get value of `flag` from key-value pairs in the file `fpath`
pub fn get_value_by_flag(flag: &str, fpath: &str, delimiter: &str) -> Fallible<String> {
    // open the cmdline file
    let file = fs::File::open(fpath)
        .with_context(|e| format_err!("Failed to open file {}: {}", fpath, e))?;

    // read the contents
    let mut bufrd = io::BufReader::new(file);
    let mut contents = String::new();
    bufrd
        .read_to_string(&mut contents)
        .with_context(|e| format_err!("Failed to read file {}: {}", fpath, e))?;

    match find_flag_value(flag, &contents, delimiter) {
        Some(platform) => {
            trace!("found '{}' flag: {}", flag, platform);
            Ok(platform)
        }
        None => bail!("Couldn't find flag '{}' in file ({})", flag, fpath),
    }
}

// Find flag value in cmdline string.
pub fn find_flag_value(flagname: &str, cmdline: &str, delimiter: &str) -> Option<String> {
    // split the contents into elements and keep key-value tuples only.
    let params: Vec<(&str, &str)> = cmdline
        .split(delimiter)
        .filter_map(|s| {
            let kv: Vec<&str> = s.splitn(2, '=').collect();
            match kv.len() {
                2 => Some((kv[0], kv[1])),
                _ => None,
            }
        })
        .collect();

    // find the oem flag
    for (key, val) in params {
        if key != flagname {
            continue;
        }
        let bare_val = val.trim();
        if !bare_val.is_empty() {
            return Some(bare_val.to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_flag() {
        let flagname = "coreos.oem.id";
        let tests = vec![
            ("", None),
            ("foo=bar", None),
            ("coreos.oem.id", None),
            ("coreos.oem.id=", None),
            ("coreos.oem.id=\t", None),
            ("coreos.oem.id=ec2", Some("ec2".to_string())),
            ("coreos.oem.id=\tec2", Some("ec2".to_string())),
            ("coreos.oem.id=ec2\n", Some("ec2".to_string())),
            ("foo=bar coreos.oem.id=ec2", Some("ec2".to_string())),
            ("coreos.oem.id=ec2 foo=bar", Some("ec2".to_string())),
        ];
        for (tcase, tres) in tests {
            let res = find_flag_value(flagname, tcase, " ");
            assert_eq!(res, tres, "failed testcase: '{}'", tcase);
        }
    }
}
