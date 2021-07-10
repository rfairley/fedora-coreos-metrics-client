//! Collect container runtime information
//!
//! Currently four container runtimes are considered: docker, podman, systemd-nspawn, crio
//! Following commands are called to check whether the runtime is running and
//! the number of containers run by the specific runtime.
//!
//! Podman:
//!   - pgrep podman
//!   - pgrep conmon
//! Docker:
//!   - pgrep dockerd
//!   - pgrep containerd-shim
//! Systemd-nspawn:
//!   - pgrep systemd-nspawn
//! Crio:
//!   - pgrep crio
//!   - pgrep crictl
//!
//! Note: none of the commands require root access

use failure::{self, bail, Fallible};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::process;

/// wrapper struct for single container runtime
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ContainerRTInfo {
    is_running: bool,
    num_containers: i32,
}

/// struct for storing all container runtime info
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ContainerRT {
    /// is_running: pgrep podman
    /// num_containers: pgrep conmon | wc -l
    podman: ContainerRTInfo,
    /// is_running: pgrep dockerd
    /// num_containers: pgrep containerd-shim | wc -l
    docker: ContainerRTInfo,
    /// is_running and num_containers: pgrep systemd-nspawn | wc -l
    systemd_nspawn: ContainerRTInfo,
    /// is_running: pgrep crio
    /// num_containers: pgrep crictl | wc -l
    crio: ContainerRTInfo,
}

/// function to run `${command} ${args}`
fn run_command(command: &str, args: &Vec<&str>) -> Result<process::Output, std::io::Error> {
    process::Command::new(command).args(args).output()
}

/// spawn a child process and pass the output of previous command through pipe
fn spawn_child(
    input: &str,
    command: &str,
    args: Vec<&str>,
) -> Result<process::Output, std::io::Error> {
    let mut child = process::Command::new(command)
        .args(args)
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .spawn()?;
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    let output = child.wait_with_output()?;
    Ok(output)
}

impl ContainerRT {
    pub(crate) fn new() -> ContainerRT {
        ContainerRT {
            podman: ContainerRTInfo {
                is_running: Self::rt_is_running("podman").unwrap_or(false),
                num_containers: Self::rt_count_running("podman").unwrap_or(0),
            },
            docker: ContainerRTInfo {
                is_running: Self::rt_is_running("docker").unwrap_or(false),
                num_containers: Self::rt_count_running("docker").unwrap_or(0),
            },
            systemd_nspawn: ContainerRTInfo {
                is_running: Self::rt_is_running("systemd_nspawn").unwrap_or(false),
                num_containers: Self::rt_count_running("systemd_nspawn").unwrap_or(0),
            },
            crio: ContainerRTInfo {
                is_running: Self::rt_is_running("crio").unwrap_or(false),
                num_containers: Self::rt_count_running("crio").unwrap_or(0),
            },
        }
    }

    /// checks if the runtime is running
    fn rt_is_running(container_rt: &str) -> Fallible<bool> {
        let command = "pgrep";
        match container_rt {
            "podman" => {
                let options = vec!["podman"];
                let output = run_command(command, &options)?;
                Ok(output.status.success())
            }
            "docker" => {
                let options = vec!["dockerd"];
                let output = run_command(command, &options)?;
                Ok(output.status.success())
            }
            "systemd_nspawn" => {
                let options = vec!["systemd-nspawn"];
                let output = run_command(command, &options)?;
                Ok(output.status.success())
            }
            "crio" => {
                let options = vec!["crio"];
                let output = run_command(command, &options)?;
                Ok(output.status.success())
            }
            _ => Ok(false),
        }
    }

    /// counts the number of running containers
    fn rt_count_running(container_rt: &str) -> Fallible<i32> {
        let command = "pgrep";
        let options = match container_rt {
            "podman" => vec!["conmon"],
            "docker" => vec!["containerd-shim"],
            "systemd-nspawn" => vec!["systemd-nspawn"],
            "crio" => vec!["crictl"],
            _ => bail!("container runtime {} is not supported", container_rt),
        };

        // run `${command} ${args}`
        let mut output = run_command(command, &options)?;
        if !output.status.success() {
            return Ok(0);
        }
        let mut std_out = String::from_utf8(output.stdout)?;

        // count lines of previous output
        output = spawn_child(std_out.as_str(), "wc", vec!["-l"])?;
        if !output.status.success() {
            return Ok(0);
        }
        std_out = String::from_utf8(output.stdout)?
            .trim()
            .trim_end_matches("\n")
            .to_string();
        let count: i32 = std_out.parse()?;

        Ok(count)
    }
}
