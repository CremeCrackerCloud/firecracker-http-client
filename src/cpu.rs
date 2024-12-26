use crate::FirecrackerError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CpuConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}

#[async_trait]
pub trait CpuConfigOperations {
    async fn put_cpu_config(&self, config: &CpuConfig) -> Result<(), FirecrackerError>;
}

#[async_trait]
impl CpuConfigOperations for crate::FirecrackerClient {
    async fn put_cpu_config(&self, config: &CpuConfig) -> Result<(), FirecrackerError> {
        let url = self.url("cpu-config")?;
        let response = self.client.put(url).json(config).send().await?;

        if !response.status().is_success() {
            return Err(FirecrackerError::Api {
                status_code: response.status().as_u16(),
                message: response.text().await?,
            });
        }

        Ok(())
    }
}
