use firecracker_http_client::{
    FirecrackerClient,
    action::InstanceActionInfo,
    balloon::BalloonOperations,
    boot::BootSourceOperations,
    cpu::{CpuConfig, CpuConfigOperations},
    drive::DriveOperations,
    entropy::{EntropyDevice, EntropyDeviceOperations},
    instance::InstanceOperations,
    logger::LoggerOperations,
    machine::MachineConfigOperations,
    metrics::{Metrics, MetricsOperations},
    mmds::MmdsOperations,
    network::NetworkInterfaceOperations,
    snapshot::{SnapshotCreateParams, SnapshotLoadParams, SnapshotOperations},
    version::VersionOperations,
    vsock::VsockOperations,
};
use firecracker_http_client::models::{
    Balloon,
    BootSource,
    Drive,
    Logger,
    MachineConfig,
    NetworkInterface,
    Vsock,
};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = FirecrackerClient::new("http://localhost:8080").await?;

    // Configure logger
    let logger = Logger {
        log_path: "/tmp/firecracker.log".to_string(),
        level: Some("Info".to_string()),
        show_level: Some(true),
        show_log_origin: Some(true),
    };
    client.put_logger(&logger).await?;

    // Configure metrics
    let metrics = Metrics {
        metrics_path: "/tmp/metrics".to_string(),
    };
    client.put_metrics(&metrics).await?;

    // Configure machine
    let machine_config = MachineConfig {
        vcpu_count: Some(2),
        mem_size_mib: Some(1024),
        cpu_template: None,
        ..Default::default()
    };
    client.put_machine_config(&machine_config).await?;

    // Configure boot source
    let boot_source = BootSource {
        kernel_image_path: "/path/to/kernel".to_string(),
        boot_args: Some("console=ttyS0 reboot=k panic=1 pci=off".to_string()),
        initrd_path: None,
    };
    client.put_boot_source(&boot_source).await?;

    // Configure drive
    let drive = Drive {
        drive_id: "rootfs".to_string(),
        path_on_host: "/path/to/rootfs".to_string(),
        is_root_device: true,
        is_read_only: false,
        cache_type: Some("Unsafe".to_string()),
        io_engine: None,
        rate_limiter: None,
        partuuid: None,
        socket: None,
    };
    client.put_drive("rootfs", &drive).await?;

    // Configure network
    let network = NetworkInterface {
        iface_id: "eth0".to_string(),
        host_dev_name: "tap0".to_string(),
        guest_mac: None,
        rx_rate_limiter: None,
        tx_rate_limiter: None,
    };
    client.put_network_interface("eth0", &network).await?;

    // Configure balloon
    let balloon = Balloon {
        amount_mib: 512,
        deflate_on_oom: Some(true),
        stats_polling_interval_s: Some(1),
    };
    client.put_balloon_config(&balloon).await?;

    // Configure vsock
    let vsock = Vsock {
        guest_cid: 3,
        uds_path: "/tmp/vsock".to_string(),
        vsock_id: None,
    };
    client.put_vsock(&vsock).await?;

    // Get version
    let version = client.get_version().await?;
    println!("Version: {:?}", version);

    // Get instance info
    let instance_info = client.describe_instance().await?;
    println!("Instance Info: {:?}", instance_info);

    // Test CPU configuration
    let cpu_config = CpuConfig {
        template: Some("C3".to_string()),
    };
    client.put_cpu_config(&cpu_config).await?;

    // Test entropy device
    let entropy = EntropyDevice {
        rate_limiter: None,
    };
    client.put_entropy_device(&entropy).await?;

    // Test instance actions
    let action = InstanceActionInfo {
        action_type: "InstanceStart".to_string(),
    };
    client.create_sync_action(&action).await?;

    // Test snapshots
    let snapshot_params = SnapshotCreateParams {
        snapshot_path: "/tmp/snapshot".to_string(),
        mem_file_path: "/tmp/snapshot.mem".to_string(),
        version: Some("1.0".to_string()),
        snapshot_type: Some("Full".to_string()),
    };
    client.create_snapshot(&snapshot_params).await?;

    // Test loading snapshots
    let load_params = SnapshotLoadParams {
        snapshot_path: "/tmp/snapshot".to_string(),
        mem_file_path: "/tmp/snapshot.mem".to_string(),
        enable_diff_snapshots: Some(true),
    };
    client.load_snapshot(&load_params).await?;

    // Configure MMDS
    let mmds_data = Value::String("Hello, MMDS!".to_string());
    client.put_mmds(mmds_data).await?;

    Ok(())
}
