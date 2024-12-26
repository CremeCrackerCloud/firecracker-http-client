use async_trait::async_trait;
use serde_json::Value;
use crate::FirecrackerError;

#[async_trait]
pub trait MmdsOperations {
    async fn put_mmds(&self, data: Value) -> Result<(), FirecrackerError>;
    async fn patch_mmds(&self, data: Value) -> Result<(), FirecrackerError>;
    async fn get_mmds(&self) -> Result<Value, FirecrackerError>;
}

#[async_trait]
impl MmdsOperations for crate::FirecrackerClient {
    async fn put_mmds(&self, data: Value) -> Result<(), FirecrackerError> {
        let url = self.url("mmds")?;
        let response = self.client.put(url).json(&data).send().await?;

        if !response.status().is_success() {
            return Err(FirecrackerError::Api {
                status_code: response.status().as_u16(),
                message: response.text().await?,
            });
        }

        Ok(())
    }

    async fn patch_mmds(&self, data: Value) -> Result<(), FirecrackerError> {
        let url = self.url("mmds")?;
        let response = self.client.patch(url).json(&data).send().await?;

        if !response.status().is_success() {
            return Err(FirecrackerError::Api {
                status_code: response.status().as_u16(),
                message: response.text().await?,
            });
        }

        Ok(())
    }

    async fn get_mmds(&self) -> Result<Value, FirecrackerError> {
        let url = self.url("mmds")?;
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(FirecrackerError::Api {
                status_code: response.status().as_u16(),
                message: response.text().await?,
            });
        }

        Ok(response.json().await?)
    }
}
