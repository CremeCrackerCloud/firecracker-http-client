use async_trait::async_trait;
use crate::models::MachineConfig;
use crate::FirecrackerError;

#[async_trait]
pub trait MachineConfigOperations {
    async fn get_machine_config(&self) -> Result<MachineConfig, FirecrackerError>;
    async fn put_machine_config(&self, config: &MachineConfig) -> Result<(), FirecrackerError>;
    async fn patch_machine_config(&self, config: &MachineConfig) -> Result<(), FirecrackerError>;
}

#[async_trait]
impl MachineConfigOperations for crate::FirecrackerClient {
    async fn get_machine_config(&self) -> Result<MachineConfig, FirecrackerError> {
        let url = self.url("machine-config")?;
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(FirecrackerError::Api {
                status_code: response.status().as_u16(),
                message: response.text().await?,
            });
        }

        Ok(response.json().await?)
    }

    async fn put_machine_config(&self, config: &MachineConfig) -> Result<(), FirecrackerError> {
        let url = self.url("machine-config")?;
        let response = self.client.put(url).json(config).send().await?;

        if !response.status().is_success() {
            return Err(FirecrackerError::Api {
                status_code: response.status().as_u16(),
                message: response.text().await?,
            });
        }

        Ok(())
    }

    async fn patch_machine_config(&self, config: &MachineConfig) -> Result<(), FirecrackerError> {
        let url = self.url("machine-config")?;
        let response = self.client.patch(url).json(config).send().await?;

        if !response.status().is_success() {
            return Err(FirecrackerError::Api {
                status_code: response.status().as_u16(),
                message: response.text().await?,
            });
        }

        Ok(())
    }
}
