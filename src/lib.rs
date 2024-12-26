use crate::{action::InstanceActionInfo, error::FirecrackerError};
use reqwest::{Client, StatusCode};
use url::Url;

#[cfg(test)]
mod tests;

pub mod action;
pub mod balloon;
pub mod boot;
pub mod cpu;
pub mod drive;
pub mod entropy;
pub mod error;
pub mod instance;
pub mod logger;
pub mod machine;
pub mod metrics;
pub mod mmds;
pub mod models;
pub mod network;
pub mod snapshot;
pub mod validation;
pub mod version;
pub mod vm;
pub mod vsock;

pub use drive::DriveOperations;
pub use models::*;
pub use network::NetworkInterfaceOperations;
pub use snapshot::SnapshotOperations;
pub use vm::VmOperations;

pub struct FirecrackerClient {
    base_url: String,
    client: Client,
}

impl FirecrackerClient {
    pub async fn new(base_url: &str) -> Result<Self, FirecrackerError> {
        Ok(Self {
            base_url: base_url.to_string(),
            client: Client::new(),
        })
    }

    pub(crate) fn url(&self, path: &str) -> Result<Url, FirecrackerError> {
        let url = format!(
            "{}/{}",
            self.base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        );
        Url::parse(&url).map_err(FirecrackerError::UrlParseError)
    }

    pub async fn create_sync_action(
        &self,
        action: &InstanceActionInfo,
    ) -> Result<(), FirecrackerError> {
        let url = self.url("/actions")?;

        let response = self.client.put(url).json(&action).send().await?;

        match response.status() {
            StatusCode::NO_CONTENT => Ok(()),
            status => {
                let error_msg = response.text().await?;
                Err(FirecrackerError::Api {
                    status_code: status.as_u16(),
                    message: error_msg,
                })
            }
        }
    }
}
