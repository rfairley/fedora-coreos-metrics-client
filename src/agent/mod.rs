//! Agent module for level `minimal` and `full`

pub mod full;
pub mod minimal;

use failure::{bail, Fallible};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub(crate) struct Agent {
    /// Collecting level
    level: String,
    /// Minimal data
    minimal: minimal::IdentityMin,
    /// Full data
    full: Option<full::IdentityFull>,
}

impl Agent {
    pub(crate) fn new(collecting_level: &str) -> Fallible<Agent> {
        match collecting_level {
            "minimal" => {
                return Ok(Agent {
                    level: "minimal".to_string(),
                    minimal: minimal::IdentityMin::new()?,
                    full: None,
                })
            }
            "full" => {
                return Ok(Agent {
                    level: "full".to_string(),
                    minimal: minimal::IdentityMin::new()?,
                    full: Some(full::IdentityFull::new()?),
                })
            }
            _ => bail!("Invalid collecting level: {}", collecting_level),
        }
    }
}

#[test]
fn test_print_minimal() {
    use crate::config::inputs;
    use clap::crate_name;

    let cfg: inputs::ConfigInput =
        inputs::ConfigInput::read_configs(vec!["tests/minimal/".to_string()], crate_name!())
            .unwrap();
    println!("{:?}", Agent::new(&cfg.collecting.level));
}

#[test]
fn test_print_full() {
    use crate::config::inputs;
    use clap::crate_name;

    let cfg: inputs::ConfigInput =
        inputs::ConfigInput::read_configs(vec!["tests/full/".to_string()], crate_name!()).unwrap();
    println!("{:?}", Agent::new(&cfg.collecting.level));
}
