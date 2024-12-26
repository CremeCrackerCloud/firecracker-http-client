use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::validation::{validate_unix_path, validate_existing_path};
use lazy_static::lazy_static;
use regex::Regex;

// Re-exports
pub use crate::logger::Logger;

// Core types

/// Represents a memory balloon device that can dynamically adjust guest memory size.
/// This device allows for memory overcommitment by reclaiming unused memory from the guest
/// and making it available to the host or other guests. It's particularly useful in
/// environments where memory resources need to be managed efficiently across multiple VMs.
#[derive(Debug, Serialize, Deserialize)]
pub struct Balloon {
    /// Target balloon size in MiB
    pub amount_mib: u32,
    /// Whether the balloon should deflate when the guest has memory pressure
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deflate_on_oom: Option<bool>,
    /// Interval in seconds between refreshing statistics. A non-zero value will enable the statistics. Defaults to 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats_polling_interval_s: Option<u32>,
}

/// Provides detailed memory statistics from the balloon device, helping monitor
/// memory usage patterns and performance of the guest VM. These statistics are
/// essential for making informed decisions about memory allocation and identifying
/// potential memory-related issues.
#[derive(Debug, Serialize, Deserialize)]
pub struct BalloonStats {
    /// Actual amount of memory (in MiB) the device is holding
    pub actual_mib: u32,
    /// Actual number of pages the device is holding
    pub actual_pages: u32,
    /// An estimate of how much memory is available (in bytes) for starting new applications, without pushing the system to swap
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_memory: Option<i64>,
    /// The amount of memory, in bytes, that can be quickly reclaimed without additional I/O. Typically these pages are used for caching files from disk
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_caches: Option<i64>,
    /// The amount of memory not being used for any purpose (in bytes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_memory: Option<i64>,
    /// The number of successful hugetlb page allocations in the guest
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hugetlb_allocations: Option<i64>,
    /// The number of failed hugetlb page allocations in the guest
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hugetlb_failures: Option<i64>,
    /// The number of major page faults that have occurred
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major_faults: Option<i64>,
    /// The number of minor page faults that have occurred
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor_faults: Option<i64>,
    /// The amount of memory that has been swapped in (in bytes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_in: Option<i64>,
    /// The amount of memory that has been swapped out to disk (in bytes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_out: Option<i64>,
    /// Target amount of memory (in MiB) the device aims to hold
    pub target_mib: u32,
    /// Target number of pages the device aims to hold
    pub target_pages: u32,
    /// The total amount of memory available (in bytes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_memory: Option<i64>,
}

/// Used to update the statistics polling interval of a balloon device.
/// This allows for dynamic adjustment of how frequently memory statistics
/// are collected without needing to recreate the balloon device.
#[derive(Debug, Serialize, Deserialize)]
pub struct BalloonStatsUpdate {
    /// Interval in seconds between refreshing statistics
    pub stats_polling_interval_s: u32,
}

/// Defines the boot configuration for a microVM, specifying the kernel image,
/// optional initial ramdisk, and kernel boot parameters. This configuration
/// must be set before starting the microVM and cannot be modified after boot.
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct BootSource {
    /// Kernel boot arguments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_args: Option<String>,
    /// Host level path to the initrd image used to boot the guest
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_existing_path")]
    pub initrd_path: Option<String>,
    /// Host level path to the kernel image used to boot the guest
    #[validate(custom = "validate_existing_path")]
    pub kernel_image_path: String,
}

/// Provides fine-grained control over CPU features exposed to the guest VM.
/// This allows for platform-specific optimizations and security configurations
/// by enabling or disabling specific CPU capabilities on both x86_64 and aarch64
/// architectures.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CpuConfig {
    /// A collection of CPUIDs to be modified (x86_64)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpuid_modifiers: Option<serde_json::Value>,
    /// A collection of kvm capabilities to be modified (aarch64)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kvm_capabilities: Option<serde_json::Value>,
    /// A collection of model specific registers to be modified (x86_64)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msr_modifiers: Option<serde_json::Value>,
    /// A collection of registers to be modified (aarch64)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg_modifiers: Option<serde_json::Value>,
    /// A collection of vcpu features to be modified (aarch64)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpu_features: Option<serde_json::Value>,
}

/// Predefined CPU templates that configure sets of CPU features to match
/// specific AWS EC2 instance types. This ensures consistent CPU feature
/// sets across different Firecracker deployments and helps with workload
/// compatibility.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum CpuTemplate {
    C3,
    None,
    T2,
    T2A,
    T2CL,
    T2S,
    V1N1,
}

/// Represents a block device in the guest VM. This can be either a regular
/// file or a block device on the host that is exposed to the guest. Supports
/// both read-only and read-write modes, and can be configured as the root
/// device for the guest filesystem.
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct Drive {
    /// Represents the caching strategy for the block device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_type: Option<String>,
    /// Unique identifier for the drive
    pub drive_id: String,
    /// Type of IO engine
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io_engine: Option<String>,
    /// Whether the block device is read-only
    pub is_read_only: bool,
    /// Whether this is the root device
    pub is_root_device: bool,
    /// Unique id of the boot partition (only used if is_root_device is true)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(regex(path = "PARTUUID_REGEX", message = "Invalid partition UUID format"))]
    pub partuuid: Option<String>,
    /// Host level path for the guest drive
    #[validate(custom = "validate_existing_path")]
    pub path_on_host: String,
    /// Rate limiter for the drive
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limiter: Option<RateLimiter>,
    /// Socket path for the drive
    #[serde(skip_serializing_if = "Option::is_none")]
    pub socket: Option<String>,
}

/// Configures a virtual device that provides entropy/randomness to the guest VM.
/// This is crucial for applications in the guest that require cryptographic
/// operations or random number generation.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EntropyDevice {
    /// Rate limiter for the entropy device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limiter: Option<RateLimiter>,
}

/// Represents an error response from the Firecracker API. Used to provide
/// detailed information about what went wrong during an API operation.
#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    /// Error message describing the fault
    pub fault_message: String,
}

/// Contains version information about the Firecracker service.
/// Used to ensure compatibility between the client and server.
#[derive(Debug, Serialize, Deserialize)]
pub struct FirecrackerVersion {
    /// Version of the Firecracker service
    pub firecracker_version: String,
}

/// Provides metadata about a Firecracker instance, including its
/// identity, current state, and version information. This is useful
/// for monitoring and managing multiple Firecracker instances.
#[derive(Debug, Serialize, Deserialize)]
pub struct InstanceInfo {
    /// Name of the application
    pub app_name: String,
    /// Instance identifier
    pub id: String,
    /// Current state of the instance
    pub state: String,
    /// Version of the VMM
    pub vmm_version: String,
}

/// Defines the core configuration of a microVM, including CPU and memory
/// resources. These settings determine the computational capacity and
/// performance characteristics of the VM.
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct MachineConfig {
    /// CPU template for configuring guest CPU features
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_template: Option<CpuTemplate>,
    /// Huge pages configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub huge_pages: Option<String>,
    /// Memory size in MiB
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mem_size_mib: Option<u32>,
    /// Enable/disable Simultaneous Multi-Threading
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smt: Option<bool>,
    /// Enable/disable dirty page tracking
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_dirty_pages: Option<bool>,
    /// Number of vCPUs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpu_count: Option<u32>,
}

/// Configures the metrics system for Firecracker, allowing for monitoring
/// of various performance and operational metrics of the microVM.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Metrics {
    /// Path to store metrics
    pub metrics_path: String,
}

/// Configures the Microvm Metadata Service (MMDS), which provides a way
/// for the guest to securely access metadata and user data. This is similar
/// to AWS EC2's instance metadata service.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MmdsConfig {
    /// IPv4 address for the MMDS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_address: Option<String>,
    /// List of network interfaces for MMDS
    pub network_interfaces: Vec<String>,
    /// Version of the MMDS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Defines a network interface for the guest VM, allowing for network
/// connectivity. Supports configuration of MAC addresses and rate limiting
/// for both receive and transmit traffic.
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct NetworkInterface {
    /// MAC address of the guest network interface
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(regex(path = "MAC_ADDRESS_REGEX", message = "Invalid MAC address format"))]
    pub guest_mac: Option<String>,
    /// Host level path for the guest network interface
    #[validate(custom = "validate_unix_path")]
    pub host_dev_name: String,
    /// Network interface identifier
    #[validate(length(min = 1))]
    pub iface_id: String,
    /// Rate limiter for received traffic
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rx_rate_limiter: Option<RateLimiter>,
    /// Rate limiter for transmitted traffic
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_rate_limiter: Option<RateLimiter>,
}

/// Implements rate limiting for I/O operations, allowing control over
/// bandwidth and operations per second. This is used by various devices
/// like network interfaces and block devices to prevent resource exhaustion.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RateLimiter {
    /// Bandwidth rate limiter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<TokenBucket>,
    /// Operations rate limiter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops: Option<TokenBucket>,
}

/// Implements the token bucket algorithm for rate limiting. This provides
/// a way to control both the steady-state rate and burst capacity for
/// operations or bandwidth.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TokenBucket {
    /// Initial burst size
    pub one_time_burst: Option<i64>,
    /// Refill time in milliseconds
    pub refill_time: i64,
    /// Bucket size
    pub size: i64,
}

/// Represents the state of a Firecracker microVM. Used primarily in
/// the context of VM lifecycle management and snapshotting operations.
#[derive(Debug, Serialize, Deserialize)]
pub struct Vm {
    /// Current state of the VM
    pub state: String,
}

/// Configures a vsock device, which provides a communication channel
/// between the host and guest. This is particularly useful for services
/// that need to communicate across the VM boundary without using traditional
/// networking.
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Vsock {
    /// CID for the guest vsock
    pub guest_cid: u32,
    /// Path to the vsock device
    #[validate(custom = "validate_unix_path")]
    pub uds_path: String,
    /// Vsock identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsock_id: Option<String>,
}

/// Represents the configuration of a Firecracker microVM, including its
/// boot source, drives, network interfaces, and machine configuration.
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct VmConfig {
    /// Balloon configuration
    pub balloon: Option<Balloon>,
    /// Boot source configuration
    pub boot_source: Option<BootSource>,
    /// List of drives
    pub drives: Vec<Drive>,
    /// Machine configuration
    pub machine_config: Option<MachineConfig>,
    /// List of network interfaces
    pub network_interfaces: Vec<NetworkInterface>,
}

lazy_static! {
    static ref MAC_ADDRESS_REGEX: Regex = Regex::new(r"^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$").unwrap();
    static ref PARTUUID_REGEX: Regex = Regex::new(r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$").unwrap();
}
