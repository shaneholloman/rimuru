use thiserror::Error;

#[derive(Debug, Error)]
pub enum RimuruError {
    #[error("adapter error: {0}")]
    Adapter(String),

    #[error("bridge error: {0}")]
    Bridge(String),

    #[error("validation error: {0}")]
    Validation(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}

impl From<iii_sdk::IIIError> for RimuruError {
    fn from(err: iii_sdk::IIIError) -> Self {
        Self::Bridge(err.to_string())
    }
}
