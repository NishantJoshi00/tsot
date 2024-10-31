#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Failed while joining a task: {0}")]
    JoinError(#[from] tokio::task::JoinError),
    #[error("Failed to store value: {0}")]
    ConnectionError(String),
    #[error("Failed to deserialize value: {0}")]
    DeserializationError(String),
}
