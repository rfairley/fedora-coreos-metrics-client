//! Module to obtain network information from `nmcli device show`,
//! parse the output, and store the output in a HashMap
use failure::{bail, format_err, Fallible, ResultExt};
use std::collections::HashMap;

// Parse key-value output from `nmcli device show`
// separated by whitespaces followed by newline
fn parse_nmcli_output(content: &str) -> HashMap<String, String> {
    // split the contents into elements and keep key-value tuples only.
    let mut hashmap = HashMap::new();
    let iter = content.split("\n");
    for e in iter {
        let kv: Vec<&str> = e.splitn(2, " ").collect();
        if kv.len() < 2 {
            continue;
        }
        let key: String = kv[0].trim().trim_end_matches(":").to_string();
        let value: String = kv[1].trim().to_string();
        hashmap.insert(key, value);
    }
    hashmap
}

pub(crate) fn get_network() -> Fallible<HashMap<String, String>> {
    let mut cmd = std::process::Command::new("nmcli");
    let cmdrun = cmd
        .arg("device")
        .arg("show")
        .output()
        .with_context(|e| format_err!("failed to run nmclli device show: {}", e))?;

    if !cmdrun.status.success() {
        bail!(
            "nmcli device show failed:\n{}",
            String::from_utf8_lossy(&cmdrun.stderr)
        );
    }
    Ok(parse_nmcli_output(&String::from_utf8_lossy(&cmdrun.stdout)))
}
