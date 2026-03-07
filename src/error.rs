//! Custom error types for mail-read-emlx

use thiserror::Error;

/// Main application error type
#[derive(Error, Debug)]
pub enum MailError {
    #[error("Message not found with ID: {0}")]
    MessageNotFound(String),

    #[error("AppleScript execution failed: {0}")]
    AppleScriptError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

impl From<std::env::VarError> for MailError {
    fn from(err: std::env::VarError) -> Self {
        MailError::AppleScriptError(err.to_string())
    }
}

impl From<std::string::FromUtf8Error> for MailError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        MailError::AppleScriptError(err.to_string())
    }
}
