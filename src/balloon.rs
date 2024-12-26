use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::models::{Balloon, BalloonStats};
use crate::FirecrackerError;

#[derive(Debug, Serialize, Deserialize)]
pub struct BalloonUpdate {
    pub amount_mib: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BalloonStatsUpdate {
    pub stats_polling_interval_s: u32,
}

#[async_trait]
pub trait BalloonOperations {
    async fn get_balloon_config(&self) -> Result<Balloon, FirecrackerError>;
    async fn put_balloon_config(&self, config: &Balloon) -> Result<(), FirecrackerError>;
    async fn patch_balloon_config(&self, update: &BalloonUpdate) -> Result<(), FirecrackerError>;
    async fn get_balloon_stats(&self) -> Result<BalloonStats, FirecrackerError>;
    async fn patch_balloon_stats(&self, update: &BalloonStatsUpdate) -> Result<(), FirecrackerError>;
}

#[async_trait]
impl BalloonOperations for crate::FirecrackerClient {
    async fn get_balloon_config(&self) -> Result<Balloon, FirecrackerError> {
        let url = self.url("balloon")?;
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(FirecrackerError::Api {
                status_code: response.status().as_u16(),
                message: response.text().await?,
            });
        }

        Ok(response.json().await?)
    }

    async fn put_balloon_config(&self, config: &Balloon) -> Result<(), FirecrackerError> {
        let url = self.url("balloon")?;
        let response = self.client.put(url).json(config).send().await?;

        if !response.status().is_success() {
            return Err(FirecrackerError::Api {
                status_code: response.status().as_u16(),
                message: response.text().await?,
            });
        }

        Ok(())
    }

    async fn patch_balloon_config(&self, update: &BalloonUpdate) -> Result<(), FirecrackerError> {
        let url = self.url("balloon")?;
        let response = self.client.patch(url).json(update).send().await?;

        if !response.status().is_success() {
            return Err(FirecrackerError::Api {
                status_code: response.status().as_u16(),
                message: response.text().await?,
            });
        }

        Ok(())
    }

    async fn get_balloon_stats(&self) -> Result<BalloonStats, FirecrackerError> {
        let url = self.url("balloon/statistics")?;
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(FirecrackerError::Api {
                status_code: response.status().as_u16(),
                message: response.text().await?,
            });
        }

        Ok(response.json().await?)
    }

    async fn patch_balloon_stats(&self, update: &BalloonStatsUpdate) -> Result<(), FirecrackerError> {
        let url = self.url("balloon/statistics")?;
        let response = self.client.patch(url).json(update).send().await?;

        if !response.status().is_success() {
            return Err(FirecrackerError::Api {
                status_code: response.status().as_u16(),
                message: response.text().await?,
            });
        }

        Ok(())
    }
}
