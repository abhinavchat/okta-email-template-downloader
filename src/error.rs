use std::path::PathBuf;

use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("configuration error: {message}")]
    Config { message: String },

    #[error("failed to read config file at {path}: {source}")]
    ConfigRead {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to parse config file at {path}: {source}")]
    ConfigParse {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },

    #[error("http request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("invalid Okta domain '{0}'")]
    InvalidDomain(String),

    #[error("Okta API returned status {status}: {message}")]
    Api {
        status: reqwest::StatusCode,
        message: String,
    },

    #[error("no brands were returned for this account")]
    NoBrands,

    #[error("no email templates were returned for the selected brand")]
    NoTemplates,

    #[error("prompt failed: {0}")]
    Prompt(String),

    #[error("path '{0}' is not a directory")]
    PathNotDirectory(PathBuf),

    #[error("non-interactive mode requires {0}")]
    NonInteractiveMissing(String),
}
