use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VmConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpu_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mem_size_mib: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ht_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_dirty_pages: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VmInfo {
    pub state: String,
    pub id: String,
}

#[async_trait]
pub trait VmOperations {
    async fn get_vm_info(&self) -> Result<VmInfo, crate::FirecrackerError>;
    async fn put_vm_config(&self, config: &VmConfig) -> Result<(), crate::FirecrackerError>;
}

#[async_trait]
impl VmOperations for crate::FirecrackerClient {
    async fn get_vm_info(&self) -> Result<VmInfo, crate::FirecrackerError> {
        let url = self.url("vm")?;
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(crate::FirecrackerError::Api {
                status_code: response.status().as_u16(),
                message: response.text().await?,
            });
        }

        Ok(response.json().await?)
    }

    async fn put_vm_config(&self, config: &VmConfig) -> Result<(), crate::FirecrackerError> {
        let url = self.url("vm/config")?;
        let response = self.client.put(url).json(config).send().await?;

        if !response.status().is_success() {
            return Err(crate::FirecrackerError::Api {
                status_code: response.status().as_u16(),
                message: response.text().await?,
            });
        }

        Ok(())
    }
}
