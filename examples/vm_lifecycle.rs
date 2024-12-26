use firecracker_http_client::{
    action::InstanceActionInfo, boot::BootSourceOperations, drive::DriveOperations,
    instance::InstanceOperations, logger::LoggerOperations, machine::MachineConfigOperations,
    metrics::Metrics, metrics::MetricsOperations, network::NetworkInterfaceOperations, BootSource,
    Drive, FirecrackerClient, Logger, MachineConfig, NetworkInterface,
};
use std::{error::Error, time::Duration};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create client
    let client = FirecrackerClient::new("http://localhost:8080").await?;

    println!("Setting up VM configuration...");

    // Configure logging
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
        smt: Some(false),
        track_dirty_pages: Some(true),
        ..Default::default()
    };
    client.put_machine_config(&machine_config).await?;

    // Configure boot source
    let boot_source = BootSource {
        kernel_image_path: "/path/to/vmlinux".to_string(),
        boot_args: Some("console=ttyS0 reboot=k panic=1 pci=off".to_string()),
        initrd_path: Some("/path/to/initrd".to_string()),
    };
    client.put_boot_source(&boot_source).await?;

    // Add root drive
    let root_drive = Drive {
        drive_id: "rootfs".to_string(),
        path_on_host: "/path/to/rootfs.ext4".to_string(),
        is_root_device: true,
        is_read_only: false,
        cache_type: Some("Unsafe".to_string()),
        ..Default::default()
    };
    client.put_drive("rootfs", &root_drive).await?;

    // Add network interface
    let network = NetworkInterface {
        iface_id: "eth0".to_string(),
        host_dev_name: "tap0".to_string(),
        guest_mac: Some("AA:BB:CC:DD:EE:FF".to_string()),
        ..Default::default()
    };
    client.put_network_interface("eth0", &network).await?;

    println!("Starting VM...");

    // Start the VM (InstanceStart action)
    let start_action = InstanceActionInfo {
        action_type: "InstanceStart".to_string(),
    };
    client.create_sync_action(&start_action).await?;

    // Wait for VM to boot and get instance info
    sleep(Duration::from_secs(2)).await;
    let instance_info = client.describe_instance().await?;
    println!(
        "VM started successfully. Instance state: {}",
        instance_info.state
    );

    // Let VM run for a while (10 seconds in this example)
    println!("VM will run for 10 seconds...");
    sleep(Duration::from_secs(10)).await;

    println!("Shutting down VM...");

    // Send shutdown action
    let shutdown_action = InstanceActionInfo {
        action_type: "SendCtrlAltDel".to_string(),
    };
    client.create_sync_action(&shutdown_action).await?;

    // Wait for VM to shutdown gracefully
    sleep(Duration::from_secs(5)).await;

    // Get final instance state
    let final_info = client.describe_instance().await?;
    println!("Final instance state: {}", final_info.state);

    println!("VM lifecycle completed successfully!");
    Ok(())
}
