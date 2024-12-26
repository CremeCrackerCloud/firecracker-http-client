use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::FirecrackerError;

#[derive(Debug, Serialize, Deserialize)]
pub struct EntropyDevice {
    pub rate_limiter: Option<crate::models::RateLimiter>,
}

#[async_trait]
pub trait EntropyDeviceOperations {
    async fn put_entropy_device(&self, device: &EntropyDevice) -> Result<(), FirecrackerError>;
}

#[async_trait]
impl EntropyDeviceOperations for crate::FirecrackerClient {
    async fn put_entropy_device(&self, device: &EntropyDevice) -> Result<(), FirecrackerError> {
        let url = self.url("entropy")?;
        let response = self.client.put(url).json(device).send().await?;

        if !response.status().is_success() {
            return Err(FirecrackerError::Api {
                status_code: response.status().as_u16(),
                message: response.text().await?,
            });
        }

        Ok(())
    }
}
