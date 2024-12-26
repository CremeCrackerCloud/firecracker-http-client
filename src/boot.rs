use crate::models::BootSource;
use crate::FirecrackerError;
use async_trait::async_trait;

#[async_trait]
pub trait BootSourceOperations {
    async fn put_boot_source(&self, boot_source: &BootSource) -> Result<(), FirecrackerError>;
}

#[async_trait]
impl BootSourceOperations for crate::FirecrackerClient {
    async fn put_boot_source(&self, boot_source: &BootSource) -> Result<(), FirecrackerError> {
        let url = self.url("boot-source")?;
        let response = self.client.put(url).json(boot_source).send().await?;

        if !response.status().is_success() {
            return Err(FirecrackerError::Api {
                status_code: response.status().as_u16(),
                message: response.text().await?,
            });
        }

        Ok(())
    }
}
