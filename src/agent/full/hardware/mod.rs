//! Collect summary of hardware on bare metal machines

pub(crate) mod lsblk;
pub(crate) mod lscpu;
pub(crate) mod lsmem;

use failure::Fallible;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct HardwareJSON {
    disk: lsblk::LsblkJSON,
    cpu: lscpu::LscpuJSON,
    memory: lsmem::LsmemJSON,
}

impl HardwareJSON {
    /// disk_info from: `lsblk --fs --json`
    /// cpu_info from: `lscpu --json`
    /// mem_info from: `lsmem --json` and `lsmem --summary`
    pub(crate) fn new() -> Fallible<HardwareJSON> {
        let lsblk_struct = lsblk::LsblkJSON::new().unwrap_or(lsblk::LsblkJSON {
            blockdevices: Vec::new(),
        });
        let lscpu_struct =
            lscpu::LscpuJSON::new().unwrap_or(lscpu::LscpuJSON { lscpu: Vec::new() });
        let lsmem_struct =
            lsmem::LsmemJSON::new().unwrap_or(lsmem::LsmemJSON { memory: Vec::new() });

        Ok(HardwareJSON {
            disk: lsblk_struct,
            cpu: lscpu_struct,
            memory: lsmem_struct,
        })
    }
}
