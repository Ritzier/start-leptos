use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Anyhow(#[from] anyhow::Error),
}
