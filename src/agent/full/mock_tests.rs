use crate::agent::full::hardware;
use crate::agent::full::container_runtime;
use hardware::lsblk;
use hardware::lscpu;
use hardware::lsmem;

#[test]
fn test_lscpu() {
    let lscpu_result = lscpu::LscpuJSON::new().unwrap();
    let expected_result: lscpu::LscpuJSON = {
        let mut cmd = std::process::Command::new("lscpu");
        let cmdrun = cmd
            .arg("--json")
            .output()
            .expect("failed to run lscpu --json");

        if !cmdrun.status.success() {
            panic!(
                "lscpu --json failed with error:\n{}",
                String::from_utf8_lossy(&cmdrun.stderr)
            );
        }
        serde_json::from_slice(&cmdrun.stdout).unwrap()
    };

    assert_eq!(lscpu_result, expected_result);
}

#[test]
fn test_lsblk() {
    let lsblk_result = lsblk::LsblkJSON::new().unwrap();
    let expected_result: lsblk::LsblkJSON = {
        let mut cmd = std::process::Command::new("lsblk");
        let cmdrun = cmd
            .arg("--fs")
            .arg("--json")
            .output()
            .expect("failed to run lsblk --fs --json");

        if !cmdrun.status.success() {
            panic!(
                "lsblk --fs --json failed:\n{}",
                String::from_utf8_lossy(&cmdrun.stderr)
            );
        }
        serde_json::from_slice(&cmdrun.stdout).unwrap()
    };

    assert_eq!(lsblk_result, expected_result);
}

#[test]
fn test_lsmem() {
    let lsmem_result = lsmem::LsmemJSON::new().unwrap();
    println!("{:?}", lsmem_result);
}

#[test]
fn test_get_hardware_info() {
    let hw_struct = hardware::HardwareJSON::new();
    println!("{:?}", hw_struct.unwrap());
}

#[test]
fn test_container_runtime() {
    let rt = container_runtime::ContainerRT::new();
    println!("{:?}", rt);
}
