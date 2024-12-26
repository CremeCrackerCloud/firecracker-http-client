use std::path::Path;
use validator::{ValidationError};
use std::borrow::Cow;

pub fn path_validation_error(message: impl Into<Cow<'static, str>>) -> ValidationError {
    let mut err = ValidationError::new("invalid_path");
    err.message = Some(message.into());
    err
}

pub fn validate_unix_path(path: &str) -> Result<(), ValidationError> {
    // Check if path is empty
    if path.is_empty() {
        return Err(path_validation_error("Path cannot be empty"));
    }

    // Check if path is absolute
    if !path.starts_with('/') {
        return Err(path_validation_error("Path must be absolute"));
    }

    // Check if path contains invalid characters
    if path.contains('\0') || path.contains("..") {
        return Err(path_validation_error("Path cannot contain parent directory references or null characters"));
    }

    // Check if path is valid UTF-8 and can be parsed
    if Path::new(path).components().count() == 0 {
        return Err(ValidationError::new("path_invalid_format"));
    }

    Ok(())
}

// Custom validation function for paths that should exist
pub fn validate_existing_path(path: &str) -> Result<(), ValidationError> {
    validate_unix_path(path)?;
    
    if !Path::new(path).exists() {
        return Err(path_validation_error("Path does not exist"));
    }
    
    Ok(())
}

// Custom validation function for paths that should be writable
pub fn validate_writable_path(path: &str) -> Result<(), ValidationError> {
    validate_unix_path(path)?;
    
    let path = Path::new(path);
    
    // If path exists, check if it's writable
    if path.exists() {
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            if let Ok(metadata) = path.metadata() {
                let mode = metadata.mode();
                if mode & 0o200 == 0 {
                    return Err(path_validation_error("Path is not writable"));
                }
            }
        }
    } else {
        // If path doesn't exist, check if parent directory is writable
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                return Err(path_validation_error("Parent directory does not exist"));
            }
            #[cfg(unix)]
            {
                use std::os::unix::fs::MetadataExt;
                if let Ok(metadata) = parent.metadata() {
                    let mode = metadata.mode();
                    if mode & 0o200 == 0 {
                        return Err(path_validation_error("Parent directory is not writable"));
                    }
                }
            }
        }
    }
    
    Ok(())
}

// Macro to implement path validation for a struct field
#[macro_export]
macro_rules! validate_path {
    ($field:expr, $validator:expr) => {
        if let Err(e) = $validator($field) {
            let mut errors = ValidationErrors::new();
            errors.add("path", e);
            return Err(errors.into());
        }
    };
}
