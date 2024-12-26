use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::FirecrackerError;
use crate::validation::validate_writable_path;
use crate::validation::validate_existing_path;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct SnapshotCreateParams {
    #[validate(custom = "validate_writable_path")]
    pub snapshot_path: String,
    #[validate(custom = "validate_writable_path")]
    pub mem_file_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(regex(path = "SNAPSHOT_TYPE_REGEX", message = "Invalid snapshot type. Must be one of: Full, Diff"))]
    pub snapshot_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct SnapshotLoadParams {
    #[validate(custom = "validate_existing_path")]
    pub snapshot_path: String,
    #[validate(custom = "validate_existing_path")]
    pub mem_file_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_diff_snapshots: Option<bool>,
}

lazy_static::lazy_static! {
    static ref SNAPSHOT_TYPE_REGEX: regex::Regex = regex::Regex::new(r"^(Full|Diff)$").unwrap();
}

#[async_trait]
pub trait SnapshotOperations {
    async fn create_snapshot(&self, params: &SnapshotCreateParams) -> Result<(), FirecrackerError>;
    async fn load_snapshot(&self, params: &SnapshotLoadParams) -> Result<(), FirecrackerError>;
}

#[async_trait]
impl SnapshotOperations for crate::FirecrackerClient {
    async fn create_snapshot(&self, params: &SnapshotCreateParams) -> Result<(), FirecrackerError> {
        params.validate()?;
        
        let url = self.url("/snapshot/create")?;
        let response = self.client.put(url).json(params).send().await?;

        if !response.status().is_success() {
            return Err(FirecrackerError::Api {
                status_code: response.status().as_u16(),
                message: response.text().await?,
            });
        }

        Ok(())
    }

    async fn load_snapshot(&self, params: &SnapshotLoadParams) -> Result<(), FirecrackerError> {
        params.validate()?;
        
        let url = self.url("/snapshot/load")?;
        let response = self.client.put(url).json(params).send().await?;

        if !response.status().is_success() {
            return Err(FirecrackerError::Api {
                status_code: response.status().as_u16(),
                message: response.text().await?,
            });
        }

        Ok(())
    }
}
