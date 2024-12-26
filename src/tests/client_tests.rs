use std::fs::File;
use mockito::{Server};
use serde_json::json;
use validator::Validate;

use crate::{
    FirecrackerClient, FirecrackerError,
    Drive, NetworkInterface, Logger, Metrics,
    SnapshotCreateParams, SnapshotLoadParams,
    Vsock, InstanceActionInfo
};

fn setup_mock_server() -> Server {
    Server::new()
}

#[tokio::test]
async fn test_client_instance_info() {
    let server = setup_mock_server();
    let mock = mock("GET", "/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "id": "test-instance",
            "state": "Running",
            "memory_size": 512,
            "vcpu_count": 1
        }).to_string())
        .create();

    let client = FirecrackerClient::new(&server.url()).unwrap();
    let info = client.get_instance_info().await.unwrap();
    assert_eq!(info.id, "test-instance");
    assert_eq!(info.state, "Running");

    mock.assert();
}

#[tokio::test]
async fn test_client_error_handling() {
    let server = setup_mock_server();

    // Test 400 Bad Request
    let mock_400 = mock("PUT", "/actions")
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "error": "Invalid request"
        }).to_string())
        .create();

    let client = FirecrackerClient::new(&server.url()).unwrap();
    let action = InstanceActionInfo {
        action_type: "Invalid".to_string(),
    };
    match client.instance_action(action).await.unwrap_err() {
        FirecrackerError::Api { status_code, .. } => assert_eq!(status_code, 400),
        _ => panic!("Expected Api error"),
    }

    mock_400.assert();

    // Test 404 Not Found
    let mock_404 = mock("GET", "/non-existent")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "error": "Not found"
        }).to_string())
        .create();

    match client.get_instance_info().await.unwrap_err() {
        FirecrackerError::Api { status_code, .. } => assert_eq!(status_code, 404),
        _ => panic!("Expected Api error"),
    }

    mock_404.assert();

    // Test connection error
    let client = FirecrackerClient::new("http://invalid-host:1234").unwrap();
    match client.get_instance_info().await.unwrap_err() {
        FirecrackerError::HttpClient(_) => (),
        _ => panic!("Expected HttpClient error"),
    }
}

#[tokio::test]
async fn test_client_drive_operations() {
    let server = setup_mock_server();
    let mock = mock("PUT", "/drives/rootfs")
        .match_body(json!({
            "drive_id": "rootfs",
            "path_on_host": "/path/to/rootfs",
            "is_root_device": true,
            "is_read_only": false
        }).to_string())
        .with_status(204)
        .create();

    let client = FirecrackerClient::new(&server.url()).unwrap();
    let drive = Drive {
        drive_id: "rootfs".to_string(),
        path_on_host: "/path/to/rootfs".to_string(),
        is_root_device: true,
        is_read_only: false,
    };
    client.put_drive(&drive).await.unwrap();

    mock.assert();
}

#[tokio::test]
async fn test_client_network_operations() {
    let server = setup_mock_server();
    let mock = mock("PUT", "/network-interfaces/eth0")
        .match_body(json!({
            "iface_id": "eth0",
            "host_dev_name": "tap0"
        }).to_string())
        .with_status(204)
        .create();

    let client = FirecrackerClient::new(&server.url()).unwrap();
    let interface = NetworkInterface {
        iface_id: "eth0".to_string(),
        host_dev_name: "tap0".to_string(),
    };
    client.put_network_interface(&interface).await.unwrap();

    mock.assert();
}

#[tokio::test]
async fn test_client_snapshot_operations() {
    let server = setup_mock_server();
    let mock_create = mock("PUT", "/snapshot/create")
        .match_body(json!({
            "snapshot_type": "Full",
            "snapshot_path": "/path/to/snapshot",
            "mem_file_path": "/path/to/mem",
            "version": "1.0"
        }).to_string())
        .with_status(204)
        .create();

    let client = FirecrackerClient::new(&server.url()).unwrap();
    let create_params = SnapshotCreateParams {
        snapshot_type: Some("Full".to_string()),
        snapshot_path: "/path/to/snapshot".to_string(),
        mem_file_path: "/path/to/mem".to_string(),
        version: Some("1.0".to_string()),
    };
    client.create_snapshot(&create_params).await.unwrap();

    mock_create.assert();

    let mock_load = mock("PUT", "/snapshot/load")
        .match_body(json!({
            "snapshot_path": "/path/to/snapshot",
            "mem_file_path": "/path/to/mem",
            "enable_diff_snapshots": false
        }).to_string())
        .with_status(204)
        .create();

    let load_params = SnapshotLoadParams {
        snapshot_path: "/path/to/snapshot".to_string(),
        mem_file_path: "/path/to/mem".to_string(),
        enable_diff_snapshots: Some(false),
    };
    client.load_snapshot(&load_params).await.unwrap();

    mock_load.assert();
}

#[tokio::test]
async fn test_client_version() {
    let server = setup_mock_server();
    let mock = mock("GET", "/version")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!({
            "firecracker_version": "1.0.0",
            "api_version": "v1.0"
        }).to_string())
        .create();

    let client = FirecrackerClient::new(&server.url()).unwrap();
    let version = client.get_version().await.unwrap();
    assert_eq!(version.firecracker_version, "1.0.0");
    assert_eq!(version.api_version, "v1.0");

    mock.assert();
}
