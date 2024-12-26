#[cfg(test)]
mod tests {
    use crate::balloon::BalloonStatsUpdate;
    use crate::cpu::CpuConfig;
    use crate::entropy::EntropyDevice;
    use crate::logger::Logger;
    use crate::metrics::Metrics;
    use crate::models::Vsock;
    use crate::vm::VmConfig;
    use crate::{
        balloon::BalloonOperations, cpu::CpuConfigOperations, entropy::EntropyDeviceOperations,
        logger::LoggerOperations, metrics::MetricsOperations, mmds::MmdsOperations,
        vm::VmOperations, vsock::VsockOperations, FirecrackerClient,
    };
    use mockito::{Server, ServerGuard};
    use serde_json::Value;

    async fn create_test_client() -> (ServerGuard, FirecrackerClient) {
        let server = Server::new_async().await;
        let client = FirecrackerClient::new(&server.url()).await.unwrap();
        (server, client)
    }

    #[tokio::test]
    async fn test_logger_configuration() {
        let (mut server, client) = create_test_client().await;
        let _m = server.mock("PUT", "/logger").with_status(204).create();

        let logger = Logger {
            log_path: "/tmp/firecracker.log".to_string(),
            level: Some("Info".to_string()),
            show_level: Some(true),
            show_log_origin: Some(true),
        };

        client.put_logger(&logger).await.unwrap();
    }

    #[tokio::test]
    async fn test_logger_validation() {
        let (mut server, client) = create_test_client().await;
        let _m = server.mock("PUT", "/logger").with_status(204).create();

        let logger = Logger {
            log_path: "/tmp/firecracker.log".to_string(),
            level: Some("Info".to_string()),
            show_level: Some(true),
            show_log_origin: Some(true),
        };

        client.put_logger(&logger).await.unwrap();
    }

    #[tokio::test]
    async fn test_logger_invalid_path() {
        let (_, client) = create_test_client().await;
        let logger = Logger {
            log_path: "invalid/path".to_string(),
            level: Some("Info".to_string()),
            show_level: Some(true),
            show_log_origin: Some(true),
        };

        let result = client.put_logger(&logger).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_logger_invalid_level() {
        let (_, client) = create_test_client().await;
        let logger = Logger {
            log_path: "/tmp/firecracker.log".to_string(),
            level: Some("InvalidLevel".to_string()),
            show_level: Some(true),
            show_log_origin: Some(true),
        };

        let result = client.put_logger(&logger).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_balloon_stats() {
        let (mut server, client) = create_test_client().await;
        let _m = server
            .mock("GET", "/balloon/statistics")
            .with_status(200)
            .with_body(
                r#"{
                "target_pages": 1000,
                "actual_pages": 950,
                "target_mib": 4,
                "actual_mib": 3,
                "swap_in": 0,
                "swap_out": 0,
                "major_faults": 0
            }"#,
            )
            .create();

        let response = client.get_balloon_stats().await.unwrap();
        assert!(response.target_pages > 0);
        assert!(response.actual_pages > 0);
    }

    #[tokio::test]
    async fn test_balloon_stats_update() {
        let (mut server, client) = create_test_client().await;
        let _m = server
            .mock("PATCH", "/balloon/statistics")
            .with_status(204)
            .create();

        let update = BalloonStatsUpdate {
            stats_polling_interval_s: 5,
        };

        client.patch_balloon_stats(&update).await.unwrap();
    }

    #[tokio::test]
    async fn test_cpu_config() {
        let (mut server, client) = create_test_client().await;
        let _m = server.mock("PUT", "/cpu-config").with_status(204).create();

        let config = CpuConfig {
            template: Some("C3".to_string()),
        };

        client.put_cpu_config(&config).await.unwrap();
    }

    #[tokio::test]
    async fn test_metrics_config() {
        let (mut server, client) = create_test_client().await;
        let _m = server.mock("PUT", "/metrics").with_status(204).create();

        let metrics = Metrics {
            metrics_path: "/tmp/metrics".to_string(),
        };

        client.put_metrics(&metrics).await.unwrap();
    }

    #[tokio::test]
    async fn test_mmds_config() {
        let (mut server, client) = create_test_client().await;
        let _m = server.mock("PUT", "/mmds").with_status(204).create();

        let config = Value::Object(serde_json::Map::new());

        client.put_mmds(config).await.unwrap();
    }

    #[tokio::test]
    async fn test_vsock_config() {
        let (mut server, client) = create_test_client().await;
        let _m = server.mock("PUT", "/vsock").with_status(204).create();

        let vsock = Vsock {
            guest_cid: 3,
            uds_path: "/tmp/vsock".to_string(),
            vsock_id: None,
        };

        client.put_vsock(&vsock).await.unwrap();
    }

    #[tokio::test]
    async fn test_entropy_device() {
        let (mut server, client) = create_test_client().await;
        let _m = server.mock("PUT", "/entropy").with_status(204).create();

        let device = EntropyDevice { rate_limiter: None };

        client.put_entropy_device(&device).await.unwrap();
    }

    #[tokio::test]
    async fn test_instance_actions() {
        let (mut server, client) = create_test_client().await;
        let _m = server.mock("PUT", "/actions").with_status(204).create();

        let action = crate::action::InstanceActionInfo::new("InstanceStart");
        client.create_sync_action(&action).await.unwrap();
    }

    #[tokio::test]
    async fn test_vm_config() {
        let (mut server, client) = create_test_client().await;
        let _m = server.mock("PUT", "/vm/config").with_status(204).create();

        let config = VmConfig {
            vcpu_count: Some(2),
            mem_size_mib: Some(1024),
            ht_enabled: Some(true),
            track_dirty_pages: Some(false),
        };

        client.put_vm_config(&config).await.unwrap();
    }

    #[tokio::test]
    async fn test_vm_info() {
        let (mut server, client) = create_test_client().await;
        let _m = server
            .mock("GET", "/vm")
            .with_status(200)
            .with_body(r#"{"state": "Running", "id": "test-vm"}"#)
            .create();

        let info = client.get_vm_info().await.unwrap();
        assert!(!info.state.is_empty());
    }
}
