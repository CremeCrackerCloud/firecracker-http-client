use crate::models::InstanceInfo;
use crate::FirecrackerError;
use async_trait::async_trait;

#[async_trait]
pub trait InstanceOperations {
    async fn describe_instance(&self) -> Result<InstanceInfo, FirecrackerError>;
}

#[async_trait]
impl InstanceOperations for crate::FirecrackerClient {
    async fn describe_instance(&self) -> Result<InstanceInfo, FirecrackerError> {
        let url = self.url("")?;
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
