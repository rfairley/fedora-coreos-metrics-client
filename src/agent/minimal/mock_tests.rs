use crate::agent::minimal;
use std::io::Write;
use tempfile;

fn mock_cmdline() -> String {
    r#"BOOT_IMAGE=(hd0,gpt1)/ostree/fedora-coreos-ea8c6e88611854b872ef062fa8cab93d8f8e49f6d74c12f99420e14293019eee/vmlinuz-5.2.15-200.fc30.x86_64 root=/dev/disk/by-label/root r
ootflags=defaults,prjquota rw ignition.firstboot rd.neednet=1 ip=dhcp mitigations=auto,nosmt console=tty0 console=ttyS0,115200n8 ignition.platform.id=aws ostree=/ostree
/boot.1/fedora-coreos/ea8c6e88611854b872ef062fa8cab93d8f8e49f6d74c12f99420e14293019eee/0"#.to_string()
}

fn mock_original_os_version() -> String {
    r#"{
	"build": "30.20190923.dev.2-2",
	"ref": "fedora/x86_64/coreos/testing-devel",
	"ostree-commit": "93244e2568e83f26fe6ab40bb85788dc066d5d18fce2d0c4a773b6ea193b13c5",
	"imgid": "fedora-coreos-30.20190923.dev.2-2-qemu.qcow2"
}"#
    .to_string()
}

fn mock_metadata() -> String {
    r#"AFTERBURN_AWS_INSTANCE_TYPE=m4.large
AFTERBURN_AWS_REGION=us-east-1
AFTERBURN_AWS_AVAILABILITY_ZONE=us-east-1c
AFTERBURN_AWS_IPV4_LOCAL=123.45.67.8
AFTERBURN_AWS_PUBLIC_HOSTNAME=ec2-123-45-678-901.compute-1.amazonaws.com
AFTERBURN_AWS_IPV4_PUBLIC=123.45.678.901
AFTERBURN_AWS_HOSTNAME=ip-123-45-67-8.ec2.internal
AFTERBURN_AWS_INSTANCE_ID=i-0e12a3451176181c4"#
        .to_string()
}

#[test]
fn test_minimal_with_file() {
    let cmdline = mock_cmdline();
    let original_os_version = mock_original_os_version();
    let metadata = mock_metadata();

    let mut karg_file =
        tempfile::NamedTempFile::new().expect("Unable to create temporary karg file");
    let mut aleph_version_file =
        tempfile::NamedTempFile::new().expect("Unable to create temporary aleph version file");
    let mut metadata_file =
        tempfile::NamedTempFile::new().expect("Unable to create temporary metadata file");

    writeln!(karg_file, "{}", cmdline).expect("Unable to write to temporary karg file");
    writeln!(aleph_version_file, "{}", original_os_version)
        .expect("Unable to write to temporary aleph version file");
    writeln!(metadata_file, "{}", metadata).expect("Unable to write to temporary metadata file");

    let minimal_id = minimal::IdentityMin::collect_minimal_data(
        karg_file.path().to_str().unwrap(),
        aleph_version_file.path().to_str().unwrap(),
        metadata_file.path().to_str().unwrap(),
    )
    .expect("Failed to collect test data");

    let expected_result = minimal::IdentityMin {
        platform: "aws".to_string(),
        original_os_version: "30.20190923.dev.2-2".to_string(),
        current_os_version: "30.20190924.dev.0".to_string(),
        instance_type: Some("m4.large".to_string()),
    };

    assert_eq!(minimal_id, expected_result);

    karg_file
        .close()
        .expect("Unable to close the temporary karg file");
    aleph_version_file
        .close()
        .expect("Unable to close the temporary aleph version file");
    metadata_file
        .close()
        .expect("Unable to close the temporary metadata file");
}
