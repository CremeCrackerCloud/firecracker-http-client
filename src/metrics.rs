use crate::validation::validate_writable_path;
use crate::FirecrackerError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Metrics {
    #[validate(custom = "validate_writable_path")]
    pub metrics_path: String,
}

#[async_trait]
pub trait MetricsOperations {
    async fn put_metrics(&self, metrics: &Metrics) -> Result<(), FirecrackerError>;
}

#[async_trait]
impl MetricsOperations for crate::FirecrackerClient {
    async fn put_metrics(&self, metrics: &Metrics) -> Result<(), FirecrackerError> {
        metrics.validate()?;

        let url = self.url("metrics")?;
        let response = self.client.put(url).json(metrics).send().await?;

        if !response.status().is_success() {
            return Err(FirecrackerError::Api {
                status_code: response.status().as_u16(),
                message: response.text().await?,
            });
        }

        Ok(())
    }
}
