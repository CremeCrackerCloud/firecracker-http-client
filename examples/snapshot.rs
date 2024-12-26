use firecracker_client::{
    FirecrackerClient,
    action::InstanceActionInfo,
    snapshot::{SnapshotCreateParams, SnapshotLoadParams, SnapshotOperations},
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create client
    let client = FirecrackerClient::new("http://localhost:8080").await?;

    // Create a snapshot
    let snapshot_params = SnapshotCreateParams {
        snapshot_path: "/tmp/snapshot".to_string(),
        mem_file_path: "/tmp/snapshot.mem".to_string(),
        version: Some("1.0".to_string()),
        snapshot_type: Some("Full".to_string()),
    };
    client.create_snapshot(&snapshot_params).await?;

    // Pause the VM before loading snapshot
    let pause_action = InstanceActionInfo {
        action_type: "Pause".to_string(),
    };
    client.create_sync_action(&pause_action).await?;

    // Load a snapshot
    let load_params = SnapshotLoadParams {
        snapshot_path: "/tmp/snapshot".to_string(),
        mem_file_path: "/tmp/snapshot.mem".to_string(),
        enable_diff_snapshots: Some(true),
    };
    client.load_snapshot(&load_params).await?;

    // Resume the VM after loading snapshot
    let resume_action = InstanceActionInfo {
        action_type: "Resume".to_string(),
    };
    client.create_sync_action(&resume_action).await?;

    println!("Snapshot operations completed successfully!");
    Ok(())
}
