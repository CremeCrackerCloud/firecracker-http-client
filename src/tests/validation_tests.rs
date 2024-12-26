use std::fs::{self, File};
use std::os::unix::fs::PermissionsExt;
use tempfile::tempdir;
use validator::Validate;

use crate::{
    validate_unix_path, validate_existing_path, validate_writable_path,
    Drive, NetworkInterface, Logger, Metrics, Vsock, SnapshotCreateParams, SnapshotLoadParams
};

#[test]
fn test_unix_path_validation() {
    // Valid paths
    assert!(validate_unix_path("/absolute/path").is_ok());
    assert!(validate_unix_path("/single").is_ok());
    assert!(validate_unix_path("/path/with/trailing/slash/").is_ok());
    assert!(validate_unix_path("/path/with-dash").is_ok());
    assert!(validate_unix_path("/path/with_underscore").is_ok());
    assert!(validate_unix_path("/path/with.dot").is_ok());
    assert!(validate_unix_path("/path/with~tilde").is_ok());
    assert!(validate_unix_path("/path/with spaces").is_ok());

    // Invalid paths
    assert!(validate_unix_path("").is_err()); // Empty path
    assert!(validate_unix_path("relative/path").is_err()); // Relative path
    assert!(validate_unix_path("./path").is_err()); // Relative with dot
    assert!(validate_unix_path("../path").is_err()); // Parent directory reference
    assert!(validate_unix_path("/path/with/../parent").is_err()); // Parent reference in middle
    assert!(validate_unix_path("/path/with/\0/null").is_err()); // Contains null character
    assert!(validate_unix_path("\0").is_err()); // Just null character
}

#[test]
fn test_existing_path_validation() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create some test files and directories
    let existing_file = temp_path.join("existing.txt");
    File::create(&existing_file).unwrap();
    
    let existing_dir = temp_path.join("existing_dir");
    fs::create_dir(&existing_dir).unwrap();

    // Valid cases
    assert!(validate_existing_path(existing_file.to_str().unwrap()).is_ok());
    assert!(validate_existing_path(existing_dir.to_str().unwrap()).is_ok());
    assert!(validate_existing_path(temp_path.to_str().unwrap()).is_ok());

    // Invalid cases
    let non_existent = temp_path.join("non_existent.txt");
    assert!(validate_existing_path(non_existent.to_str().unwrap()).is_err());
    
    let non_existent_dir = temp_path.join("non_existent_dir");
    assert!(validate_existing_path(non_existent_dir.to_str().unwrap()).is_err());
    
    assert!(validate_existing_path("").is_err());
    assert!(validate_existing_path("relative/path").is_err());
}

#[test]
fn test_writable_path_validation() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create test files with different permissions
    let writable_file = temp_path.join("writable.txt");
    File::create(&writable_file).unwrap();
    
    let readonly_file = temp_path.join("readonly.txt");
    File::create(&readonly_file).unwrap();
    let mut perms = fs::metadata(&readonly_file).unwrap().permissions();
    perms.set_mode(0o444); // Read-only
    fs::set_permissions(&readonly_file, perms).unwrap();

    let writable_dir = temp_path.join("writable_dir");
    fs::create_dir(&writable_dir).unwrap();
    
    let readonly_dir = temp_path.join("readonly_dir");
    fs::create_dir(&readonly_dir).unwrap();
    let mut perms = fs::metadata(&readonly_dir).unwrap().permissions();
    perms.set_mode(0o555); // Read-only directory
    fs::set_permissions(&readonly_dir, perms).unwrap();

    // Valid cases
    assert!(validate_writable_path(writable_file.to_str().unwrap()).is_ok());
    assert!(validate_writable_path(writable_dir.to_str().unwrap()).is_ok());
    
    // New file in writable directory
    let new_file = writable_dir.join("new_file.txt");
    assert!(validate_writable_path(new_file.to_str().unwrap()).is_ok());

    // Invalid cases
    assert!(validate_writable_path(readonly_file.to_str().unwrap()).is_err());
    assert!(validate_writable_path(readonly_dir.to_str().unwrap()).is_err());
    
    // New file in readonly directory
    let new_file_readonly = readonly_dir.join("new_file.txt");
    assert!(validate_writable_path(new_file_readonly.to_str().unwrap()).is_err());
    
    // Non-existent parent
    let non_existent_parent = temp_path.join("non_existent_dir/file.txt");
    assert!(validate_writable_path(non_existent_parent.to_str().unwrap()).is_err());
    
    assert!(validate_writable_path("").is_err());
    assert!(validate_writable_path("relative/path").is_err());
}

#[test]
fn test_drive_validation() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    let existing_file = temp_path.join("disk.img");
    File::create(&existing_file).unwrap();

    // Valid cases
    let valid_drive = Drive {
        drive_id: "rootfs".to_string(),
        path_on_host: existing_file.to_str().unwrap().to_string(),
        is_root_device: true,
        is_read_only: false,
        partuuid: Some("00000000-1234-1234-1234-123456789abc".to_string()),
        rate_limiter: None,
    };
    assert!(valid_drive.validate().is_ok());

    // Invalid cases
    let invalid_drive_empty_id = Drive {
        drive_id: "".to_string(),
        ..valid_drive.clone()
    };
    assert!(invalid_drive_empty_id.validate().is_err());

    let invalid_drive_non_existent = Drive {
        path_on_host: "/non/existent/path".to_string(),
        ..valid_drive.clone()
    };
    assert!(invalid_drive_non_existent.validate().is_err());

    let invalid_drive_partuuid = Drive {
        partuuid: Some("invalid-uuid".to_string()),
        ..valid_drive.clone()
    };
    assert!(invalid_drive_partuuid.validate().is_err());
}

#[test]
fn test_network_interface_validation() {
    // Valid cases
    let valid_interface = NetworkInterface {
        iface_id: "eth0".to_string(),
        host_dev_name: "/dev/tap0".to_string(),
        guest_mac: Some("12:34:56:78:9A:BC".to_string()),
        rx_rate_limiter: None,
        tx_rate_limiter: None,
    };
    assert!(valid_interface.validate().is_ok());

    // Invalid cases
    let invalid_interface_empty_id = NetworkInterface {
        iface_id: "".to_string(),
        ..valid_interface.clone()
    };
    assert!(invalid_interface_empty_id.validate().is_err());

    let invalid_interface_relative_path = NetworkInterface {
        host_dev_name: "dev/tap0".to_string(),
        ..valid_interface.clone()
    };
    assert!(invalid_interface_relative_path.validate().is_err());

    let invalid_interface_mac = NetworkInterface {
        guest_mac: Some("invalid:mac:address".to_string()),
        ..valid_interface.clone()
    };
    assert!(invalid_interface_mac.validate().is_err());
}

#[test]
fn test_logger_validation() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    let log_file = temp_path.join("firecracker.log");

    // Valid cases
    let valid_logger = Logger {
        log_path: log_file.to_str().unwrap().to_string(),
        level: Some("Info".to_string()),
        show_level: Some(true),
        show_log_origin: Some(true),
    };
    assert!(valid_logger.validate().is_ok());

    // Invalid cases
    let invalid_logger_empty_path = Logger {
        log_path: "".to_string(),
        ..valid_logger.clone()
    };
    assert!(invalid_logger_empty_path.validate().is_err());

    let invalid_logger_relative_path = Logger {
        log_path: "relative/path.log".to_string(),
        ..valid_logger.clone()
    };
    assert!(invalid_logger_relative_path.validate().is_err());

    let invalid_logger_level = Logger {
        level: Some("InvalidLevel".to_string()),
        ..valid_logger.clone()
    };
    assert!(invalid_logger_level.validate().is_err());
}

#[test]
fn test_snapshot_validation() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    
    let existing_file = temp_path.join("existing.snap");
    File::create(&existing_file).unwrap();
    
    let existing_mem = temp_path.join("existing.mem");
    File::create(&existing_mem).unwrap();

    // Valid cases for create
    let valid_create = SnapshotCreateParams {
        snapshot_path: temp_path.join("new.snap").to_str().unwrap().to_string(),
        mem_file_path: temp_path.join("new.mem").to_str().unwrap().to_string(),
        snapshot_type: Some("Full".to_string()),
        version: None,
    };
    assert!(valid_create.validate().is_ok());

    // Valid cases for load
    let valid_load = SnapshotLoadParams {
        snapshot_path: existing_file.to_str().unwrap().to_string(),
        mem_file_path: existing_mem.to_str().unwrap().to_string(),
        enable_diff_snapshots: Some(true),
    };
    assert!(valid_load.validate().is_ok());

    // Invalid cases for create
    let invalid_create_relative = SnapshotCreateParams {
        snapshot_path: "relative/path.snap".to_string(),
        ..valid_create.clone()
    };
    assert!(invalid_create_relative.validate().is_err());

    let invalid_create_type = SnapshotCreateParams {
        snapshot_type: Some("Invalid".to_string()),
        ..valid_create.clone()
    };
    assert!(invalid_create_type.validate().is_err());

    // Invalid cases for load
    let invalid_load_non_existent = SnapshotLoadParams {
        snapshot_path: "/non/existent/path".to_string(),
        mem_file_path: "/non/existent/mem".to_string(),
        enable_diff_snapshots: None,
    };
    assert!(invalid_load_non_existent.validate().is_err());
}

#[test]
fn test_vsock_validation() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Valid cases
    let valid_vsock = Vsock {
        guest_cid: 3,
        uds_path: temp_path.join("vsock.sock").to_str().unwrap().to_string(),
    };
    assert!(valid_vsock.validate().is_ok());

    // Invalid cases
    let invalid_vsock_cid_low = Vsock {
        guest_cid: 2,
        ..valid_vsock.clone()
    };
    assert!(invalid_vsock_cid_low.validate().is_err());

    let invalid_vsock_cid_high = Vsock {
        guest_cid: 4294967296,
        ..valid_vsock.clone()
    };
    assert!(invalid_vsock_cid_high.validate().is_err());

    let invalid_vsock_relative_path = Vsock {
        uds_path: "relative/path.sock".to_string(),
        ..valid_vsock.clone()
    };
    assert!(invalid_vsock_relative_path.validate().is_err());
}
