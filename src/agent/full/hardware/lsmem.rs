//! Module for running `lsmem --json`, and storing the output
//! in the struct LsmemJSON
use failure::{bail, format_err, Fallible, ResultExt};
use serde::de::{self, Unexpected};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct LsmemJSON {
    pub(crate) memory: Vec<MemoryJSON>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct MemoryJSON {
    size: String,
    state: String,
    #[serde(deserialize_with = "deserialize_bool_or_string")]
    removable: bool,
    block: String,
}

impl LsmemJSON {
    pub(crate) fn new() -> Fallible<LsmemJSON> {
        let mut cmd = std::process::Command::new("lsmem");
        let cmdrun = cmd
            .arg("--json")
            .output()
            .with_context(|e| format_err!("failed to run lsmem --json: {}", e))?;

        if !cmdrun.status.success() {
            bail!(
                "lsmem --json failed:\n{}",
                String::from_utf8_lossy(&cmdrun.stderr)
            );
        }
        Ok(serde_json::from_slice(&cmdrun.stdout)?)
    }
}

struct DeserializeBoolOrString;

impl<'de> de::Visitor<'de> for DeserializeBoolOrString {
    type Value = bool;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a bool or a string")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "yes" {
            Ok(true)
        } else if v == "no" {
            Ok(false)
        } else {
            Err(E::invalid_value(Unexpected::Str(v), &self))
        }
    }
}

/// In some version of lsmem, the field `removable` is 'yes'/'no' instead of 'true'/'false',
/// causing failure of deserialization by serde, hence adds this customized deserializing function
fn deserialize_bool_or_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(DeserializeBoolOrString)
}
