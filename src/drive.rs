use crate::models::Drive;
use crate::FirecrackerError;
use async_trait::async_trait;

#[async_trait]
pub trait DriveOperations {
    async fn put_drive(&self, drive_id: &str, drive: &Drive) -> Result<(), FirecrackerError>;
    async fn patch_drive(&self, drive_id: &str, drive: &Drive) -> Result<(), FirecrackerError>;
}

#[async_trait]
impl DriveOperations for crate::FirecrackerClient {
    async fn put_drive(&self, drive_id: &str, drive: &Drive) -> Result<(), FirecrackerError> {
        let url = self.url(&format!("drives/{}", drive_id))?;
        let response = self.client.put(url).json(drive).send().await?;

        if !response.status().is_success() {
            return Err(FirecrackerError::Api {
                status_code: response.status().as_u16(),
                message: response.text().await?,
            });
        }

        Ok(())
    }

    async fn patch_drive(&self, drive_id: &str, drive: &Drive) -> Result<(), FirecrackerError> {
        let url = self.url(&format!("drives/{}", drive_id))?;
        let response = self.client.patch(url).json(drive).send().await?;

        if !response.status().is_success() {
            return Err(FirecrackerError::Api {
                status_code: response.status().as_u16(),
                message: response.text().await?,
            });
        }

        Ok(())
    }
}
