use crate::models::NetworkInterface;
use crate::FirecrackerError;
use async_trait::async_trait;

#[async_trait]
pub trait NetworkInterfaceOperations {
    async fn put_network_interface(
        &self,
        iface_id: &str,
        interface: &NetworkInterface,
    ) -> Result<(), FirecrackerError>;
    async fn patch_network_interface(
        &self,
        iface_id: &str,
        interface: &NetworkInterface,
    ) -> Result<(), FirecrackerError>;
}

#[async_trait]
impl NetworkInterfaceOperations for crate::FirecrackerClient {
    async fn put_network_interface(
        &self,
        iface_id: &str,
        interface: &NetworkInterface,
    ) -> Result<(), FirecrackerError> {
        let url = self.url(&format!("network-interfaces/{}", iface_id))?;
        let response = self.client.put(url).json(interface).send().await?;

        if !response.status().is_success() {
            return Err(FirecrackerError::Api {
                status_code: response.status().as_u16(),
                message: response.text().await?,
            });
        }

        Ok(())
    }

    async fn patch_network_interface(
        &self,
        iface_id: &str,
        interface: &NetworkInterface,
    ) -> Result<(), FirecrackerError> {
        let url = self.url(&format!("network-interfaces/{}", iface_id))?;
        let response = self.client.patch(url).json(interface).send().await?;

        if !response.status().is_success() {
            return Err(FirecrackerError::Api {
                status_code: response.status().as_u16(),
                message: response.text().await?,
            });
        }

        Ok(())
    }
}
