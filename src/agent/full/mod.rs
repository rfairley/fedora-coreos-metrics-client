//! Module to collect data under `full` level

pub(crate) mod container_runtime;
pub(crate) mod hardware;
pub(crate) mod network;

#[cfg(test)]
mod mock_tests;

use crate::agent::minimal;
use failure::Fallible;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub(crate) struct IdentityFull {
    hardware: Option<hardware::HardwareJSON>,
    network: HashMap<String, String>,
    container_rt: container_runtime::ContainerRT,
}

impl IdentityFull {
    pub(crate) fn new() -> Fallible<IdentityFull> {
        // only collect hardware information on bare-metal systems
        let platform = minimal::platform::get_platform(minimal::KERNEL_ARGS_FILE)
            .unwrap_or("metal".to_string());
        let network = network::get_network().unwrap_or(HashMap::new());
        let container_rt = container_runtime::ContainerRT::new();
        match platform.as_str() {
            "metal" => {
                let hw = hardware::HardwareJSON::new()?;
                Ok(IdentityFull {
                    hardware: Some(hw),
                    network,
                    container_rt,
                })
            }
            _ => Ok(IdentityFull {
                hardware: None,
                network,
                container_rt,
            }),
        }
    }
}
