use thiserror::Error;

#[derive(Error, Debug)]
pub enum AnkiError {
    #[error("HTTP request failed: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Failed to serialize/deserialize JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("AnkiConnect returned an error: {0}")]
    Api(String),
}

pub type Result<T> = std::result::Result<T, AnkiError>;
