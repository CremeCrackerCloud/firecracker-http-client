use async_trait::async_trait;
use validator::Validate;
use crate::models::Vsock;
use crate::FirecrackerError;

#[async_trait]
pub trait VsockOperations {
    async fn put_vsock(&self, vsock: &Vsock) -> Result<(), FirecrackerError>;
}

#[async_trait]
impl VsockOperations for crate::FirecrackerClient {
    async fn put_vsock(&self, vsock: &Vsock) -> Result<(), FirecrackerError> {
        vsock.validate()?;
        
        let url = self.url("vsock")?;
        let response = self.client.put(url).json(vsock).send().await?;

        if !response.status().is_success() {
            return Err(FirecrackerError::Api {
                status_code: response.status().as_u16(),
                message: response.text().await?,
            });
        }

        Ok(())
    }
}
