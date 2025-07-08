use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RealDebridError {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Serde JSON error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Tauri API error: {0}")]
    Tauri(#[from] tauri::Error),

    #[error("API returned an error: {message} (Code: {code})")]
    Api { message: String, code: i64 },

    #[error("Authentication failed: {0}")]
    Auth(String),

    #[error("An unknown error occurred")]
    Unknown,
}

impl Serialize for RealDebridError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}