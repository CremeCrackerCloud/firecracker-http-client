# Firecracker Client

A comprehensive Rust client library for interacting with the Firecracker VMM API. This client provides a safe, ergonomic interface for managing Firecracker microVMs with full support for all Firecracker v1.11.0 features.

## Features

- **Complete API Coverage**: Full support for all Firecracker API endpoints
- **Async/Await**: Built on Tokio for efficient async operations
- **Type Safety**: Strong typing for all API requests and responses
- **Input Validation**: Comprehensive validation of all API inputs
- **Error Handling**: Detailed error types with context
- **Rate Limiting**: Built-in support for API rate limiting
- **Documentation**: Extensive documentation and examples

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
firecracker-client = "0.1.0"
```

## Core Components

### Client Structure

The client is organized into trait-based modules for different Firecracker operations:

- `BootSourceOperations`: Manage kernel and boot parameters
- `DriveOperations`: Configure block devices
- `NetworkInterfaceOperations`: Set up network interfaces
- `MachineConfigOperations`: Configure VM resources
- `SnapshotOperations`: Create and load VM snapshots
- `MetricsOperations`: Configure metrics collection
- `LoggerOperations`: Manage logging
- `InstanceOperations`: Control VM lifecycle

### Key Types

#### Machine Configuration

```rust
use firecracker_client::{MachineConfig, machine::MachineConfigOperations};

let config = MachineConfig {
    vcpu_count: Some(2),
    mem_size_mib: Some(1024),
    smt: Some(false),
    track_dirty_pages: Some(true),
    ..Default::default()
};
client.put_machine_config(&config).await?;
```

#### Network Configuration

```rust
use firecracker_client::{NetworkInterface, network::NetworkInterfaceOperations};

let network = NetworkInterface {
    iface_id: "eth0".to_string(),
    host_dev_name: "tap0".to_string(),
    guest_mac: Some("AA:BB:CC:DD:EE:FF".to_string()),
    ..Default::default()
};
client.put_network_interface("eth0", &network).await?;
```

#### Block Devices

```rust
use firecracker_client::{Drive, drive::DriveOperations};

let drive = Drive {
    drive_id: "rootfs".to_string(),
    path_on_host: "/path/to/rootfs.ext4".to_string(),
    is_root_device: true,
    is_read_only: false,
    ..Default::default()
};
client.put_drive("rootfs", &drive).await?;
```

## Usage Examples

### Basic VM Setup

The basic_vm.rs example shows how to configure a simple microVM:

```rust
use firecracker_client::{
    FirecrackerClient,
    BootSource,
    Drive,
    MachineConfig,
    NetworkInterface,
    boot::BootSourceOperations,
    drive::DriveOperations,
    network::NetworkInterfaceOperations,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = FirecrackerClient::new("http://localhost:8080").await?;
    
    // Configure machine resources
    let machine_config = MachineConfig {
        vcpu_count: Some(2),
        mem_size_mib: Some(1024),
        ..Default::default()
    };
    client.put_machine_config(&machine_config).await?;
    
    // Set up boot source
    let boot_source = BootSource {
        kernel_image_path: "/path/to/vmlinux".to_string(),
        boot_args: Some("console=ttyS0".to_string()),
        ..Default::default()
    };
    client.put_boot_source(&boot_source).await?;
    
    Ok(())
}
```

### Complete VM Lifecycle

The vm_lifecycle.rs example demonstrates the full VM lifecycle:

1. Initial Configuration
2. VM Start
3. Runtime Monitoring
4. Graceful Shutdown

```rust
// Start the VM
let start_action = InstanceActionInfo {
    action_type: "InstanceStart".to_string(),
};
client.create_sync_action(&start_action).await?;

// Monitor VM state
let instance_info = client.describe_instance().await?;
println!("VM state: {}", instance_info.state);

// Graceful shutdown
let shutdown_action = InstanceActionInfo {
    action_type: "SendCtrlAltDel".to_string(),
};
client.create_sync_action(&shutdown_action).await?;
```

### Snapshot Management

The snapshot.rs example shows how to create and load VM snapshots:

```rust
// Create snapshot
let snapshot_params = SnapshotCreateParams {
    snapshot_path: "/tmp/snapshot".to_string(),
    mem_file_path: "/tmp/snapshot.mem".to_string(),
    snapshot_type: Some("Full".to_string()),
    version: Some("1.0".to_string()),
};
client.create_snapshot(&snapshot_params).await?;

// Load snapshot
let load_params = SnapshotLoadParams {
    snapshot_path: "/tmp/snapshot".to_string(),
    mem_file_path: "/tmp/snapshot.mem".to_string(),
    enable_diff_snapshots: Some(true),
};
client.load_snapshot(&load_params).await?;
```

## Error Handling

The client provides detailed error types for better error handling:

```rust
use firecracker_client::FirecrackerError;

match result {
    Err(FirecrackerError::Api { status_code, message }) => {
        eprintln!("API error {}: {}", status_code, message);
    }
    Err(FirecrackerError::Network(e)) => {
        eprintln!("Network error: {}", e);
    }
    Err(FirecrackerError::Validation(e)) => {
        eprintln!("Validation error: {}", e);
    }
    Ok(_) => println!("Operation successful"),
}
```

## Running the Examples

1. Start Firecracker API server:
```bash
firecracker --api-sock /tmp/firecracker.sock
```

2. Run the examples:
```bash
# Basic VM setup
cargo run --example basic_vm

# Complete VM lifecycle
cargo run --example vm_lifecycle

# Snapshot management
cargo run --example snapshot
```

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with logs
RUST_LOG=debug cargo test
```

### Contributing

1. Fork the repository
2. Create your feature branch
3. Add tests for any new functionality
4. Ensure all tests pass
5. Submit a pull request

## License

This project is licensed under the Apache License, Version 2.0.

## Related Projects

- [Firecracker](https://github.com/firecracker-microvm/firecracker)
- [Firecracker API Spec](https://github.com/firecracker-microvm/firecracker/blob/main/src/api_server/swagger/firecracker.yaml)
