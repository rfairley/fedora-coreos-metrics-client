//! Telemetry service for FCOS.

extern crate nix;
extern crate slog;
#[macro_use]
extern crate slog_scope;

#[cfg(test)]
extern crate tempfile;

/// agent module
mod agent;
/// Collect config from files.
mod config;
/// rpm-ostree client.
mod rpm_ostree;
/// utility functions
mod util;

use clap::{crate_authors, crate_description, crate_name, crate_version, Arg};
use config::inputs;
use failure::{bail, Fallible, ResultExt};
use log::LevelFilter;
#[cfg(test)]
use mockito;
use nix::unistd::{fork, ForkResult};
use serde_json::json;
use std::thread;
use std::time::Duration;

/// Parse the reporting.enabled and collecting.level keys from config fragments,
/// and check that the keys are set to a valid telemetry setting. If not,
/// or in case of other error, return non-zero.
fn check_config(config: &inputs::ConfigInput) -> Fallible<bool> {
    if config.reporting.enabled.unwrap() {
        println!("Reporting enabled.");

        let collecting_level = &config.collecting.level;
        match collecting_level.as_str() {
            "minimal" | "full" => println!("Collection set at level '{}'.", collecting_level),
            _ => bail!("invalid collection level '{}'", collecting_level),
        }

        Ok(true)
    } else {
        println!("Reporting disabled.");

        Ok(false)
    }
}

fn send_data(agent: &agent::Agent) -> Fallible<()> {
    // TODO: Send data to remote endpoint
    // Currently only prints the Agent struct
    println!("{:?}", json!(agent));

    #[cfg(test)]
    {
        let url = mockito::server_url();
        let client = reqwest::Client::new();
        client.post(url.as_str()).json(agent).send()?;
    };

    Ok(())
}

fn main() -> Fallible<()> {
    match fork() {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("New child has pid: {}", child);
            return Ok(());
        }
        Ok(ForkResult::Child) => (),
        Err(_) => panic!("Fork failed in main()"),
    };

    // continues running in child process
    let matches = clap::app_from_crate!()
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets log verbosity level"),
        )
        .get_matches();

    let log_level = match matches.occurrences_of("v") {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        3 | _ => LevelFilter::Trace,
    };
    env_logger::Builder::from_default_env()
        .format_timestamp(None)
        .format_module_path(false)
        .filter(None, log_level)
        .try_init()?;

    let dirs = vec![
        String::from("/usr/lib"),
        String::from("/run"),
        String::from("/etc"),
    ];
    let config = inputs::ConfigInput::read_configs(dirs, crate_name!())
        .context("failed to read configuration input")?;

    let is_enabled = check_config(&config)?;
    let collecting_level: String = config.collecting.level;

    // Collect the data if enabled
    if is_enabled {
        let collecting_level_copy_daily = collecting_level.clone();
        let collecting_level_copy_monthly = collecting_level.clone();

        // spawn thread for monitoring timestamp and sending report daily
        let daily_thread = thread::spawn(move || -> Fallible<()> {
            const DAILY_TIMESTAMP_FILE: &str = r#"/var/log/fedora-coreos-pinger/timestamp_daily"#;
            const SECS_PER_12_HOURS: Duration = Duration::from_secs(12 * 60 * 60);
            loop {
                let clock = util::Clock::read_timestamp(DAILY_TIMESTAMP_FILE)?;
                if clock.if_need_update("daily")? {
                    println!("Collecting and sending daily report...");
                    let agent = agent::Agent::new(collecting_level_copy_daily.as_str())?;
                    // Send to the remote endpoint
                    send_data(&agent)?;
                    // Update the timestamp
                    clock.write_timestamp(DAILY_TIMESTAMP_FILE)?;
                }
                thread::sleep(SECS_PER_12_HOURS);
            }
        });

        // spawn thread for monitoring timestamp and sending report monthly
        let monthly_thread = thread::spawn(move || -> Fallible<()> {
            const MONTHLY_TIMESTAMP_FILE: &str =
                r#"/var/log/fedora-coreos-pinger/timestamp_monthly"#;
            const SECS_PER_15_DAYS: Duration = Duration::from_secs(15 * 24 * 60 * 60);
            loop {
                let clock = util::Clock::read_timestamp(MONTHLY_TIMESTAMP_FILE)?;
                if clock.if_need_update("monthly")? {
                    println!("Collecting and sending monthly report...");
                    let agent = agent::Agent::new(collecting_level_copy_monthly.as_str())?;
                    // Send to the remote endpoint
                    send_data(&agent)?;
                    // Update the timestamp
                    clock.write_timestamp(MONTHLY_TIMESTAMP_FILE)?;
                }
                thread::sleep(SECS_PER_15_DAYS);
            }
        });

        println!("Waiting for threads...");

        daily_thread
            .join()
            .expect("Thread for daily reporting failed")?;
        monthly_thread
            .join()
            .expect("Thread for monthly reporting failed")?;
    }

    Ok(())
}

#[test]
fn test_send_data() {
    use crate::agent::Agent;
    use crate::config::inputs;
    use clap::crate_name;

    let mock = mockito::mock("POST", "/")
        .match_header("content-type", "application/json")
        .with_status(200)
        .create();

    let cfg: inputs::ConfigInput =
        inputs::ConfigInput::read_configs(vec!["tests/full/".to_string()], crate_name!()).unwrap();
    let agent = Agent::new(&cfg.collecting.level).unwrap();
    send_data(&(agent)).unwrap();

    mock.assert();
}
