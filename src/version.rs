use crate::models::FirecrackerVersion;
use crate::FirecrackerError;
use async_trait::async_trait;

#[async_trait]
pub trait VersionOperations {
    async fn get_version(&self) -> Result<FirecrackerVersion, FirecrackerError>;
}

#[async_trait]
impl VersionOperations for crate::FirecrackerClient {
    async fn get_version(&self) -> Result<FirecrackerVersion, FirecrackerError> {
        let url = self.url("version")?;
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
