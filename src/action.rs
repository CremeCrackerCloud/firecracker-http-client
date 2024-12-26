use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::error::FirecrackerError;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceActionInfo {
    pub action_type: String,
}

impl InstanceActionInfo {
    pub fn new(action_type: &str) -> Self {
        Self {
            action_type: action_type.to_string(),
        }
    }
}

lazy_static! {
    static ref ACTION_TYPE_REGEX: Regex = Regex::new(r"^(InstanceStart|InstanceHalt|SendCtrlAltDel)$").unwrap();
}

#[async_trait]
pub trait ActionOperations {
    async fn create_sync_action(&self, action: &InstanceActionInfo) -> Result<(), FirecrackerError>;
}

#[async_trait]
impl ActionOperations for crate::FirecrackerClient {
    async fn create_sync_action(&self, action: &InstanceActionInfo) -> Result<(), FirecrackerError> {
        let url = self.url("actions")?;
        let response = self.client.put(url).json(action).send().await?;

        if !response.status().is_success() {
            return Err(FirecrackerError::Api {
                status_code: response.status().as_u16(),
                message: response.text().await?,
            });
        }

        Ok(())
    }
}
