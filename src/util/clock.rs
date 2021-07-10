//! Clock for tracking report intervals
//! currently implemented with two levels of intervals: daily and monthly
use bincode::{deserialize_from, serialize_into};
use failure::{bail, Fallible};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::time::SystemTime;

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Clock {
    timestamp: SystemTime,
}

impl Clock {
    /// reads previous timestamp from file
    /// located in `/var/lib/fedora-coreos-pinger`
    pub(crate) fn read_timestamp(path: &str) -> Fallible<Clock> {
        let f = open_file(path)?;
        let mut reader = BufReader::new(f);
        let clock: Clock = deserialize_from(&mut reader)?;

        Ok(clock)
    }

    /// writes current timestamp into file
    /// located in `/var/lib/fedora-coreos-pinger`
    pub(crate) fn write_timestamp(&self, path: &str) -> Fallible<()> {
        let mut f = BufWriter::new(File::create(path)?);
        serialize_into(&mut f, self)?;
        Ok(())
    }

    /// checks if the timestamp needs an update
    /// mode = 'daily' | 'monthly'
    pub(crate) fn if_need_update(&self, mode: &str) -> Fallible<bool> {
        let secs_per_day = 24 * 60 * 60;
        let secs_per_month = 31 * secs_per_day;

        let now = SystemTime::now();
        let elapsed_seconds = now.duration_since(self.timestamp)?.as_secs();

        match mode {
            "daily" => {
                let elapsed_days = elapsed_seconds / secs_per_day;
                if elapsed_days >= 1 {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            "monthly" => {
                let elapsed_months = elapsed_seconds / secs_per_month;
                if elapsed_months >= 1 {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            _ => bail!("Clock mode is not supported"),
        }
    }
}

/// open a file in read-only mode and create the file if it doesn't exist
/// with current timestamp and then returns the File
fn open_file(path: &str) -> Fallible<File> {
    if !Path::new(path).exists() {
        let file = File::create(path)?;
        let clock = Clock {
            timestamp: SystemTime::now(),
        };
        let mut writer = BufWriter::new(file);
        serialize_into(&mut writer, &clock)?;
    }
    Ok(File::open(path)?)
}
