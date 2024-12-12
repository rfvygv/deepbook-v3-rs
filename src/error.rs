use thiserror::Error;

pub type DeepBookResult<T> = Result<T,DeepBookError>;
#[derive(Error, Debug)]
pub enum DeepBookError {
    #[error(transparent)]
    AnyhowError(anyhow::Error)
}

