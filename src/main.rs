use firecracker_client::{
    FirecrackerClient,
    balloon::BalloonOperations,
    boot::BootSourceOperations,
    drive::DriveOperations,
    logger::LoggerOperations,
    machine::MachineConfigOperations,
    metrics::{Metrics, MetricsOperations},
    network::NetworkInterfaceOperations,
    vsock::VsockOperations,
    cpu::CpuConfigOperations,
    entropy::EntropyDeviceOperations,
    instance::InstanceOperations,
    mmds::MmdsOperations,
    version::VersionOperations,
    snapshot::SnapshotOperations,
    models::{
        Balloon, BootSource, MachineConfig, Drive, NetworkInterface,
        Vsock, Logger,
    },
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
        huge_pages: None,
        smt: None,
        track_dirty_pages: None,
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
        partuuid: None,
        rate_limiter: None,
        cache_type: None,
        io_engine: None,
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

    // Get version information
    let version = client.get_version().await?;
    println!("Version: {:?}", version);

    // Get instance information
    let instance_info = client.describe_instance().await?;
    println!("Instance Info: {:?}", instance_info);

    // Configure CPU
    let cpu_config = firecracker_client::cpu::CpuConfig {
        template: Some("C3".to_string()),
    };
    client.put_cpu_config(&cpu_config).await?;

    // Configure entropy device
    let entropy = firecracker_client::entropy::EntropyDevice {
        rate_limiter: None,
    };
    client.put_entropy_device(&entropy).await?;

    // Configure MMDS
    let mmds_data = Value::String("Hello, MMDS!".to_string());
    client.put_mmds(mmds_data).await?;

    // Start the instance
    let action = firecracker_client::action::InstanceActionInfo::new("InstanceStart");
    client.create_sync_action(&action).await?;

    // Create a snapshot
    let snapshot_params = firecracker_client::snapshot::SnapshotCreateParams {
        snapshot_type: Some("Full".to_string()),
        snapshot_path: "/path/to/snapshot".to_string(),
        mem_file_path: "/path/to/mem_file".to_string(),
        version: None,
    };
    client.create_snapshot(&snapshot_params).await?;

    // Load a snapshot
    let load_params = firecracker_client::snapshot::SnapshotLoadParams {
        snapshot_path: "/path/to/snapshot".to_string(),
        mem_file_path: "/path/to/mem_file".to_string(),
        enable_diff_snapshots: Some(true),
    };
    client.load_snapshot(&load_params).await?;

    Ok(())
}
