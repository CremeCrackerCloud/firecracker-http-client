use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::FirecrackerError;
use crate::validation::validate_writable_path;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Logger {
    #[validate(custom = "validate_writable_path")]
    pub log_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(regex(path = "LOG_LEVEL_REGEX", message = "Invalid log level. Must be one of: Error, Warning, Info, Debug"))]
    pub level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_level: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_log_origin: Option<bool>,
}

lazy_static::lazy_static! {
    static ref LOG_LEVEL_REGEX: regex::Regex = regex::Regex::new(r"^(Error|Warning|Info|Debug)$").unwrap();
}

#[async_trait]
pub trait LoggerOperations {
    async fn put_logger(&self, logger: &Logger) -> Result<(), FirecrackerError>;
}

#[async_trait]
impl LoggerOperations for crate::FirecrackerClient {
    async fn put_logger(&self, logger: &Logger) -> Result<(), FirecrackerError> {
        logger.validate()?;
        
        let url = self.url("logger")?;
        let response = self.client.put(url).json(logger).send().await?;

        if !response.status().is_success() {
            return Err(FirecrackerError::Api {
                status_code: response.status().as_u16(),
                message: response.text().await?,
            });
        }

        Ok(())
    }
}
