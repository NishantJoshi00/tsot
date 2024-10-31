#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Failed while joining a task: {0}")]
    JoinError(#[from] tokio::task::JoinError),
}
